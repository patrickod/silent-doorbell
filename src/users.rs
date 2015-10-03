use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::fs::File;
use std::option::Option;

#[derive(Debug)]
pub struct User {
    username: String,
    phone_number: Option<String>,
    slack_username: Option<String>
}

pub fn read_users(path: String) -> Result<Vec<User>> {
    let f = try!(File::open(path));
    let reader = BufReader::new(f);
    let mut users: Vec<User> = Vec::new();

    for line in reader.lines() {
        let line = try!(line);
        match parse_user(line) {
            Some(user) => { users.push(user) },
            None => ()
        }
    }
    return Ok(users);
}

fn parse_user(line: String) -> Option<User> {
    let parts: Vec<String> = line.split(",").map( |s| s.to_owned() ).collect();
    let username = parts.get(0);

    if username.is_none() || username.unwrap().is_empty() {
        return None;
    }

    let phone_number = match parts.get(1) {
        Some(number) => Some(number.to_owned()),
        None => None
    };
    let slack_username = match parts.get(2) {
        Some(number) => Some(number.to_owned()),
        None => None
    };
    return Some(User {
        username: username.unwrap().to_owned(),
        phone_number: phone_number,
        slack_username: slack_username
    });
}

#[test]
fn test_parse_user() {
    let good = "patrickod,+16507017829,patrickod".to_string();
    let bad = "".to_string();

    let option_good = parse_user(good);
    assert!(option_good.is_some());
    let parsed_good = option_good.unwrap();
    assert_eq!(parsed_good.username, "patrickod".to_string());
    assert_eq!(parsed_good.phone_number.unwrap(), "+16507017829".to_string());
    assert_eq!(parsed_good.slack_username.unwrap(), "patrickod".to_string());

    assert!(parse_user(bad).is_none());
}
