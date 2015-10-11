#[macro_use]
extern crate log;
extern crate env_logger;
extern crate wuphf;
extern crate request;

use std::env;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    env_logger::init().unwrap();

    let (tx, rx) = channel();
    let reader = wuphf::earl::connect("127.0.0.1:1234");

    thread::spawn(move || {
        wuphf::earl::listen(reader, tx);
    });

    println!("{}", rx.recv().unwrap());
}
