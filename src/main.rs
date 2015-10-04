extern crate silent_doorbell;
extern crate request;

use silent_doorbell::twilio_config::TwilioConfig;
use std::env;

fn main() {
    let user_file = env::args().nth(1).unwrap();
    let users = silent_doorbell::users::read_users(user_file).unwrap();
    let patrick: silent_doorbell::users::User = users[0].clone();

    match silent_doorbell::sms::send_sms("+15005550006", &patrick.phone_number.unwrap(), "Hey there") {
        Ok(message_sid) => { println!("I sent message: {}", message_sid) },
        Err(err) => { panic!("Error: {:?}", err) }
    }
}
