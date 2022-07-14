use async_trait::async_trait;
use bytes::BytesMut;
use log::debug;
use redis_protocol::resp2::prelude::{Frame, encode_bytes};
use tokio::io::AsyncWriteExt;

use super::Command;

pub struct ListCommand<'a> {
    pub commands: &'a Vec<Box<dyn Command>>
}

#[async_trait]
impl<'a> Command for ListCommand<'a> {
    fn accept(&self,frames: &Vec<redis_protocol::resp2::prelude::Frame>) -> bool {
        for frame in frames {
            if frame.is_string() {
                let str = frame.as_str().unwrap().to_string();
                if str.to_lowercase() == "command" {
                    return true;
                }
            }
        }
        false
    }
    
    
    async fn handle(&self, socket: &mut tokio::net::TcpStream) {
        let mut all: Vec<Frame> = vec![];
        for command in self.commands {
            let mut per_command: Vec<Frame> = vec![];
            per_command.push(Frame::BulkString(command.name().into()));
            per_command.push(Frame::BulkString(format!("{}", command.arity()).into()));
            let mut fa = vec![];
            for f in command.flag() {
                fa.push(Frame::BulkString(f.into()));
            }
            per_command.push(Frame::Array(fa));
            per_command.push(Frame::BulkString(format!("{}", command.first_key()).into()));
            per_command.push(Frame::BulkString(format!("{}", command.last_key()).into()));
            per_command.push(Frame::BulkString(format!("{}", command.step()).into()));
            all.push(Frame::Array(per_command));
        }

        let mut buf = BytesMut::new();
        
        let _len = match encode_bytes(&mut buf, &Frame::Array(all)) {
            Ok(l) => l,
            Err(e) => panic!("Error encoding frame: {:?}", e)
        };
        socket
            .write_all(&buf)
            .await
            .expect("failed to write data to socket");
        
        debug!("write socket {:#?}", &buf);
        
    }

    fn name(&self) -> String {
       "command".to_string()
    }

    fn arity(&self) -> i32 {
        -1
    }

    fn flag(&self) -> Vec<String>  {
        vec!["stale".to_string(), "random".to_string(), "loading".to_string()]
    }

    fn first_key(&self) -> i32 {
        0
    }

    fn last_key(&self) -> i32 {
        0
    }

    fn step(&self) -> i32 {
        0
    }
}