extern crate silent_doorbell;
extern crate request;

use silent_doorbell::twilio_config::TwilioConfig;
use std::thread;

fn main() {
    // let twilio_config = TwilioConfig::from_env();

    // let child = thread::spawn(move || {
    //     match silent_doorbell::sms::send_sms(twilio_config, "+15005550006", "+16507017829", "Hey there") {
    //         Ok(message_sid) => { println!("I sent message: {}", message_sid) },
    //         Err(err) => { panic!("Error: {:?}", err) }
    //     }
    // });

    // child.join();

    println!("{:?}", silent_doorbell::users::read_users("/tmp/users".to_string()));
}
