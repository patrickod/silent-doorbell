#[macro_use]
extern crate log;
extern crate env_logger;
extern crate wuphf;
extern crate request;

use std::env;

fn main() {
    env_logger::init().unwrap();

    let user_file = env::args().nth(1).unwrap();
    let users = wuphf::users::read_users(user_file).unwrap();
    let patrick: wuphf::users::User = users[0].clone();

    match wuphf::sms::send_sms("+15005550006", &patrick.phone_number.unwrap(), "Hey there") {
        Ok(message_sid) => { info!("Sent SMS: {}", message_sid); },
        Err(err) => { panic!("Error: {:?}", err) }
    }
}
