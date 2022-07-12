use env_logger::Builder;
use log::{LevelFilter, debug};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use redis_protocol::resp2::prelude::*;
use bytes::{Bytes, BytesMut};

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
                    Frame::SimpleString(_) => todo!(),
                    Frame::Error(_) => todo!(),
                    Frame::Integer(_) => todo!(),
                    Frame::BulkString(_) => todo!(),
                    Frame::Array(array) => {
                        for f in array {
                            match f {
                                Frame::SimpleString(_) => todo!(),
                                Frame::Error(_) => todo!(),
                                Frame::Integer(_) => todo!(),
                                Frame::BulkString(_) => {
                                    let frame = Frame::BulkString("PONG".into());
                                    let mut buf = BytesMut::new();
                                    
                                    let _len = match encode_bytes(&mut buf, &frame) {
                                        Ok(l) => l,
                                        Err(e) => panic!("Error encoding frame: {:?}", e)
                                    };
                                    socket
                                        .write_all(&buf)
                                        .await
                                        .expect("failed to write data to socket");
                                    
                                    debug!("write socket {:#?}", &buf);
                                },
                                Frame::Array(_) => todo!(),
                                Frame::Null => todo!(),
                            }
                        }
                    },
                    Frame::Null => todo!(),
                }
                // debug!("debug {}", std::str::from_utf8(&buf[0..n]).unwrap());
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