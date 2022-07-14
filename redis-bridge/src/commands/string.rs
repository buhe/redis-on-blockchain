use async_trait::async_trait;
use bytes::BytesMut;
use log::debug;
use redis_protocol::resp2::prelude::{Frame, encode_bytes};
use tokio::io::AsyncWriteExt;

use super::{Command, match_command};

pub struct Get {

}

#[async_trait]
impl Command for Get {
    fn accept(&self,frame: &Vec<redis_protocol::resp2::prelude::Frame>) -> bool {
        match_command(&self.name(), frame)
    }
    
    
    async fn handle(&self, socket: &mut tokio::net::TcpStream) {
        let frame = Frame::BulkString("result..".into());
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

    fn name(&self) -> String {
       "get".to_string()
    }

    fn arity(&self) -> i32 {
        2
    }

    fn flag(&self) -> Vec<String>  {
        vec!["readonly".to_string(), "fast".to_string()]
    }

    fn first_key(&self) -> i32 {
        1
    }

    fn last_key(&self) -> i32 {
        1
    }

    fn step(&self) -> i32 {
        1
    }
}

pub struct Set {

}

#[async_trait]
impl Command for Set {
    fn accept(&self,frame: &Vec<redis_protocol::resp2::prelude::Frame>) -> bool {
        match_command(&self.name(), frame)
    }
    
    
    async fn handle(&self, socket: &mut tokio::net::TcpStream) {
        let frame = Frame::BulkString("OK".into());
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

    fn name(&self) -> String {
       "set".to_string()
    }

    fn arity(&self) -> i32 {
        -3
    }

    fn flag(&self) -> Vec<String>  {
        vec!["write".to_string(), "denyoom".to_string()]
    }

    fn first_key(&self) -> i32 {
        1
    }

    fn last_key(&self) -> i32 {
        1
    }

    fn step(&self) -> i32 {
        1
    }
}