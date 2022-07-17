use async_trait::async_trait;
use bytes::BytesMut;
use log::{info};
use redis_protocol::resp2::prelude::{Frame, encode_bytes};
use tokio::io::AsyncWriteExt;

use crate::web3::client::Wallet;

use super::{Command, match_command};

pub struct Get {
   
}

#[async_trait]
impl Command for Get {
    fn accept(&self,frame: &Vec<redis_protocol::resp2::prelude::Frame>) -> bool {
        match_command(&self.name(), frame)
    }
    
    
    async fn handle(&self, socket: &mut tokio::net::TcpStream, frames: &Vec<redis_protocol::resp2::prelude::Frame>,  wallet: &Wallet) {
        let key = frames.get(1).unwrap();
        let key = key.as_str().unwrap();
        let value = wallet.get(key).await.unwrap().to_string();
        let frame = Frame::BulkString(value.into());
        let mut buf = BytesMut::new();
        
        let _len = match encode_bytes(&mut buf, &frame) {
            Ok(l) => l,
            Err(e) => panic!("Error encoding frame: {:?}", e)
        };
        socket
            .write_all(&buf)
            .await
            .expect("failed to write data to socket");
        
        info!("write socket {:#?}", &buf);
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
    
    
    async fn handle(&self, socket: &mut tokio::net::TcpStream, frames: &Vec<redis_protocol::resp2::prelude::Frame>, wallet: &Wallet) {
        let key = frames.get(1).unwrap();
        let key = key.as_str().unwrap();
        let value = frames.get(2).unwrap();
        let value = value.as_str().unwrap();
        wallet.set(key, value).await.unwrap();
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
        
        info!("write socket {:#?}", &buf);
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