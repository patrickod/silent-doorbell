extern crate url;
extern crate request;
extern crate rustc_serialize;

use std::collections::HashMap;
use rustc_serialize::json::Json;

use twilio_config::TwilioConfig;

#[derive(Debug)]
pub enum SMSError {
    UnknownException
}

pub fn send_sms(config: TwilioConfig, from: &str, to: &str, body: &str) -> Result<String, SMSError> {
    let endpoint = twilio_api_sms_url(&config);
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut params: HashMap<String, String> = HashMap::new();

    params.insert("From".to_string(), from.to_string());
    params.insert("To".to_string(), to.to_string());
    params.insert("Body".to_string(), body.to_string());

    headers.insert("Connection".to_string(), "close".to_string());
    headers.insert("Authorization".to_string(), config.to_http_auth());
    headers.insert("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string());

    let response = match request::post(&endpoint, &mut headers, serialize_message_request_mody(params).as_bytes()) {
        Ok(response) => response,
        Err(_) => { return Err(SMSError::UnknownException); }
    };

    if response.status_code != 201 {
        return Err(SMSError::UnknownException);
    }

    let json = Json::from_str(&response.body).unwrap();
    let message_sid = json.as_object().unwrap().get("sid").unwrap();

    return Ok(format!("{}", message_sid));
}

fn twilio_api_sms_url(config: &TwilioConfig) -> String {
    return format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", config.account_sid)
}

fn serialize_message_request_mody(params: HashMap<String, String>) -> String {
   return url::form_urlencoded::serialize(params.iter());
}

#[test]
fn test_serialize_message_request_body() {
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("Foo".to_string(), "Bar".to_string());
    assert_eq!(serialize_message_request_mody(params), "Foo=Bar".to_string())
}
