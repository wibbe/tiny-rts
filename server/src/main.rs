
use std::net::{TcpListener, TcpStream};

fn main() {
   println!("Starting server");

   let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

   // Start accepting new connections
   for stream in listener.incoming() {
      match stream {
         Ok(stream) => {

         }
         Err(e) => {
            println!("Connection failed: {}", e);
         }
      }
   }
}