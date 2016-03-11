use rustc_serialize::base64::{STANDARD,ToBase64};
use std::env;

pub struct TwilioConfig {
    pub account_sid: String,
    pub auth_token: String,
}

impl TwilioConfig {
    pub fn from_env() -> TwilioConfig {
        let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("No TWILIO_ACCOUNT_SID");
        let auth_token = env::var("TWILIO_AUTH_TOKEN").expect("No TWILIO_AUTH_TOKEN");

        return TwilioConfig {
            account_sid: account_sid,
            auth_token: auth_token
        }
    }

    pub fn to_http_auth(&self) -> String {
        let formatted_credentials = format!("{}:{}", self.account_sid, self.auth_token);
        let encoded_credentials = formatted_credentials.into_bytes().to_base64(STANDARD);
        return format!("Basic {}", encoded_credentials);
    }
}


#[test]
fn test_twilio_config_to_http_auth() {
    let config = TwilioConfig {
        account_sid: "foo".to_string(),
        auth_token: "bar".to_string()
    };
    let header = config.to_http_auth();

    assert_eq!(header, "Basic Zm9vOmJhcg==".to_string());
}
