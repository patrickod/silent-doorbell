#[macro_use]
extern crate log;
extern crate env_logger;
extern crate wuphf;
extern crate request;

use std::sync::mpsc::channel;
use std::thread;

fn main() {
    env_logger::init().unwrap();

    let address = match std::env::var("EARL_URL") {
        Ok(val) => val,
        Err(_) => "127.0.0.1:1234".to_string()
    };

    let (tx, rx) = channel();
    let reader = wuphf::earl::connect(&address);

    thread::spawn(move || {
        wuphf::earl::listen(reader, tx);
    });

    println!("{}", rx.recv().unwrap());
}
