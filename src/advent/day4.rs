extern crate chrono;
use chrono::NaiveDateTime;
use std::collections::HashMap;

use super::file;

struct Message {
    timestamp: chrono::NaiveDateTime,
    message_type: MessageType,
    guard_id: u64,
}

#[derive(Copy, Clone)]
enum MessageType {
    WakeUp,
    GoToSleep,
    StartShift,
}

struct Record {
    sleep_start: NaiveDateTime,
    sleep_end: NaiveDateTime,
    duration: i64,
}

pub(crate) fn main() {
    let file = file::file_read("data/4.txt");
    let lines = file.lines();

    let mut messages = parse_lines(lines);
    messages.sort_by_key(|m| m.timestamp);
    let records = parse_messages(messages);
}

fn parse_lines(lines: std::str::Lines<'_>) -> Vec<Message> {
    let mut messages = Vec::<Message>::new();

    for line in lines {
        let first_split: Vec<String> = line.split(']').map(|s| s.to_string()).collect();
        let date_string = first_split[0][1..first_split[0].len()].to_string();
        let timestamp = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M").unwrap();

        let message_type: MessageType;
        let mut guard_id: u64 = 0;
        if line.contains("wakes up") {
            message_type = MessageType::WakeUp;
        } else if line.contains("falls asleep") {
            message_type = MessageType::GoToSleep;
        } else if line.contains("Guard") {
            message_type = MessageType::StartShift;
            guard_id = line
                .split('#')
                .nth(1)
                .unwrap()
                .split(" ")
                .nth(0)
                .unwrap()
                .parse()
                .unwrap();
        } else {
            println!("line unrecognized: {}", line);
            continue;
        }

        messages.push(Message {
            timestamp,
            message_type,
            guard_id,
        });
    }

    return messages;
}

fn parse_messages(messages: Vec<Message>) -> HashMap<u64, Vec<Record>> {
    let mut curr_id = 0;
    let mut sleep_start_time = messages[0].timestamp;
    let mut guard_records: HashMap<u64, Vec<Record>> = HashMap::new();
    for message in messages {
        match message.message_type {
            MessageType::StartShift => curr_id = message.guard_id,
            MessageType::GoToSleep => sleep_start_time = message.timestamp,
            MessageType::WakeUp => {
                let duration = (message.timestamp - sleep_start_time).num_minutes();
                let record = Record {
                    sleep_start: sleep_start_time,
                    sleep_end: message.timestamp,
                    duration,
                };
                if let Some(records) = guard_records.get_mut(&curr_id) {
                    records.push(record);
                } else {
                    guard_records.insert(curr_id, vec![record]);
                }
            }
        }
    }

    return guard_records;
}
