use redis_protocol::resp2::prelude::Frame;
use tokio::net::TcpStream;

pub mod connection;
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    fn accept(&self, frame: &Vec<Frame>) -> bool;

    async fn handle(&self, socket: &mut TcpStream);
}
