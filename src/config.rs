use rustc_serialize::base64::{STANDARD,ToBase64};
use std::env;

pub struct TwilioConfig {
    pub account_sid: String,
    pub auth_token: String,
}

impl TwilioConfig {
    pub fn from_env() -> TwilioConfig {
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

    pub fn to_http_auth(&self) -> String {
        let auth_string = format!("Basic {}:{}", self.account_sid, self.auth_token);
        return auth_string.into_bytes().to_base64(STANDARD);
    }
}
