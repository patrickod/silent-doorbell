use std::io::prelude::*;
use std::io::BufReader;
use std::sync::mpsc::Sender;
use std::net::{TcpListener,TcpStream};

pub fn connect(address: &str) -> BufReader<TcpStream> {
    let stream = match TcpStream::connect(address) {
        Ok(stream) => stream,
        Err(e) => { return panic!("Unable to connect to earl: {:?}", e)}
    };

    return BufReader::new(stream); 
}

pub fn listen(reader: BufReader<TcpStream>, events: Sender<String>) {
    for line in reader.lines() {
        events.send(line.unwrap()).unwrap();
    }
}
