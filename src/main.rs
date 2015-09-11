extern crate silent_doorbell;
extern crate request;

use silent_doorbell::config::TwilioConfig;
use std::collections::HashMap;

fn main() {
    let twilio_config = TwilioConfig::from_env();

    let url = "http://requestb.in/pb2c67pb";
    let mut headers: HashMap<String, String> = HashMap::new();

    headers.insert("Authentication".to_string(), twilio_config.to_http_auth());

    let response = match request::get(&url, &mut headers) {
        Ok(response) => response,
        Err(e) => { println!("{}", e); return ;}
    };

    println!("{}", response.http_version);
    println!("{}", response.status_code);
    println!("{}", response.status_message);
    println!("{}", response.body);
}
