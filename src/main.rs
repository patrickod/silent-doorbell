extern crate silent_doorbell;

use silent_doorbell::config::TwilioConfig;

fn main() {
    let twilio_config = TwilioConfig::from_env();
    println!("auth: {}", twilio_config.to_http_auth());
}
