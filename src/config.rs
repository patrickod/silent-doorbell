use std::env;

pub struct TwilioConfig {
    account_sid: String,
    auth_token: String
}

pub fn get_twilio_config() -> TwilioConfig {
    let account_sid = env::var("TWILIO_ACCOUNT_SID");
    let auth_token = env::var("TWILIO_AUTH_TOKEN");

    if account_sid.is_ok() && auth_token.is_ok() {
        return TwilioConfig {
            account_sid: account_sid.unwrap(),
            auth_token: auth_token.unwrap()
        }
    } else {
        panic!("Unable to read Twilio config from environment")
    }
}
