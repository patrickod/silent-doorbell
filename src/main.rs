#[macro_use]
extern crate log;
extern crate env_logger;
extern crate wuphf;
extern crate request;

use std::env;
use std::thread;
use std::io::Write;
use std::net::{TcpListener,TcpStream};

fn handle_client(mut stream: TcpStream) {
    info!("New connection from {:?}", stream.peer_addr().unwrap());
    stream.write(b"Hello there\r\n");
}

fn main() {
    env_logger::init().unwrap();

    let user_file = env::args().nth(1).unwrap();
    let users = wuphf::users::read_users(user_file).unwrap();

    let listener = TcpListener::bind("127.0.0.1:1214").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                error!("Incoming connection failed: {:?}", e);
            }
        }
    }
    drop(listener);
}
