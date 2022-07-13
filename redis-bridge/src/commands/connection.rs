use super::Command;
use bytes::BytesMut;
use log::debug;
use redis_protocol::resp2::prelude::*;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

pub struct Ping {

}


#[async_trait]
impl Command for Ping {
    fn accept(&self, frames: &Vec<Frame>) -> bool {
        for frame in frames {
            if frame.is_string() {
                let str = frame.as_str().unwrap().to_string();
                if str.to_lowercase() == "ping" {
                    return true;
                }
            }
        }
        false
    }

    async fn handle(&self, socket: &mut tokio::net::TcpStream) {
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
    }
}