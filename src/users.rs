use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::fs::File;
use std::option::Option;

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub phone_number: Option<String>,
    pub slack_username: Option<String>
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

    if phone_number.is_some() || slack_username.is_some() {
        return Some(User {
            username: username.unwrap().to_owned(),
            phone_number: phone_number,
            slack_username: slack_username
        });
    } else {
        return None;
    }
}

#[test]
fn test_parse_user() {
    let line = "patrickod,+16507017829,patrickod".to_string();
    let user = parse_user(line).unwrap();

    assert_eq!(user.username, "patrickod".to_string());
    assert_eq!(user.slack_username.unwrap(), "patrickod".to_string());
    assert_eq!(user.phone_number.unwrap(), "+16507017829".to_string());
}

#[test]
fn test_parse_empty_user() {
    let line = "".to_string();
    assert!(parse_user(line).is_none());
}

#[test]
fn test_parse_user_username_only() {
    let line = "patrickod".to_string();
    assert!(parse_user(line).is_none());
}
