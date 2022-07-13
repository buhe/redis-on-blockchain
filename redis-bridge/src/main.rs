use commands::Command;
use commands::connection::Ping;
use commands::server::ListCommand;
use commands::string::Get;
use env_logger::Builder;
use failure::Error;
use log::{LevelFilter, debug};
use tokio::io::{AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use redis_protocol::resp2::prelude::*;
use bytes::{Bytes};
use async_recursion::async_recursion;
use std::env;

mod commands;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    init_logging(LevelFilter::Trace);
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6379".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                let b: Bytes = Bytes::copy_from_slice(&buf[0..n]);
                let (frame, consumed) = match decode(&b) {
                    Ok(Some((f, c))) => (f, c),
                    Ok(None) => panic!("Incomplete frame."),
                    Err(e) => panic!("Error parsing bytes: {:?}", e)
                };
                debug!("Parsed frame {:?} and consumed {} bytes", &frame, consumed);
                match &frame {
                    Frame::Array(array) => {
                        handle_array(array, &mut socket).await;
                    },
                    _ => unreachable!(),
                }
            }
        });
    }
}

pub fn init_logging(default_lvl: LevelFilter) {
    match env::var("RUST_LOG") {
        Ok(_) => env_logger::init(),
        Err(_) => Builder::new().filter_level(default_lvl).init(),
    }
}


#[async_recursion]
async fn handle_array(frames: &Vec<Frame>, socket: &mut TcpStream) {
    let commands: Vec<Box<dyn Command>> = vec![Box::new(Ping{}), Box::new(ListCommand{}), Box::new(Get{})];
    for c in commands {
        if c.accept(frames) {
            c.handle(socket).await;
        }
    }
}