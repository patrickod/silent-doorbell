use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::fs::File;
use std::collections::HashMap;

pub fn read_users(path: String) -> Result<HashMap<String, String>> {
    let f = try!(File::open(path));
    let reader = BufReader::new(f);
    let mut users: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = try!(line);
        let parts: Vec<&str> = line.split(",").collect();
        users.insert(parts[0].to_string(), parts[1].to_string()); 
    }
    return Ok(users);
}
