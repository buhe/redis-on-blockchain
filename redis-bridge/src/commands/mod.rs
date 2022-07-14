use redis_protocol::resp2::prelude::Frame;
use tokio::net::TcpStream;

pub mod connection;
pub mod server;
// pub mod string;
use async_trait::async_trait;

#[async_trait]
pub trait Command: Send + Sync{
    fn accept(&self, frame: &Vec<Frame>) -> bool;

    async fn handle(&self, socket: &mut TcpStream);

    fn name(&self) -> String;

    fn arity(&self) -> i32;

    fn flag(&self) -> Vec<String>;

    fn first_key(&self) -> i32;

    fn last_key(&self) -> i32;

    fn step(&self) -> i32;
}
