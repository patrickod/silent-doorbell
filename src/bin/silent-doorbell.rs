extern crate silent_doorbell;

use silent_doorbell::config; 

fn main() {
    let config = config::get_twilio_config();
}
