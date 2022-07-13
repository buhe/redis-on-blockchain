use async_trait::async_trait;

use super::Command;

pub struct ListCommand {

}

#[async_trait]
impl Command for ListCommand {
    fn accept(&self,frame: &Vec<redis_protocol::resp2::prelude::Frame>) -> bool {
        false
    }
    
    
    async fn handle(&self, socket: &mut tokio::net::TcpStream) {
        todo!()
    }

    fn name(&self) -> String {
       "command".to_string()
    }

    fn arity(&self) -> i32 {
        todo!()
    }

    fn flag(&self) -> Vec<String>  {
        todo!()
    }

    fn first_key(&self) -> i32 {
        todo!()
    }

    fn last_key(&self) -> i32 {
        todo!()
    }

    fn step(&self) -> i32 {
        todo!()
    }
}

// use async_trait::async_trait;

// use super::Command;

// pub struct ListCommand {

// }

// #[async_trait]
// impl Command for ListCommand {
//     fn accept(&self,frame: &Vec<redis_protocol::resp2::prelude::Frame>) -> bool {
//         todo!()
//     }
    
    
//     async fn handle(&self, socket: &mut tokio::net::TcpStream) {
//         todo!()
//     }

//     fn name() -> String {
//         todo!()
//     }

//     fn arity() -> i32 {
//         todo!()
//     }

//     fn flag() -> Vec<String>  {
//         todo!()
//     }

//     fn first_key() -> i32 {
//         todo!()
//     }

//     fn last_key() -> i32 {
//         todo!()
//     }

//     fn step() -> i32 {
//         todo!()
//     }
// }