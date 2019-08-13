extern crate chrono;
use chrono::NaiveDateTime;

use super::file;

struct Message {
    timestamp: chrono::NaiveDateTime,
}

pub(crate) fn main() {
    println!("Test");
    let file = file::file_read("data/4.txt");
    let lines = file.lines();

    let mut messages = Vec::<Message>::new();
    for line in lines {
        let first_split: Vec<String> = line.split(']').map(|s| s.to_string()).collect();
        let date_string = first_split[0][1..first_split[0].len()].to_string();
        let timestamp = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M").unwrap();
        println!("{}", timestamp);

        messages.push(Message { timestamp });
    }
}
