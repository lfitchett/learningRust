extern crate chrono;
use crate::advent::day4::chrono::Timelike;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::iter::FromIterator;

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
    let most_slept_guard = get_most_slept(&records);
    let most_common_minute = get_most_common_minute(records.get(&most_slept_guard).unwrap());
    println!("{}, {}, {}", most_slept_guard, most_common_minute, most_slept_guard * most_common_minute as u64);
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

fn get_most_slept(guard_records: &HashMap<u64, Vec<Record>>) -> u64 {
    *guard_records
        .iter()
        .max_by_key(|(_id, records)| (**records).iter().map(|r| r.duration).sum::<i64>())
        .unwrap()
        .0
}

fn get_most_common_minute(records: &Vec<Record>) -> u32 {
    *records
        .iter()
        .map(|r| {
            if r.sleep_start < r.sleep_end {
                Vec::from_iter(r.sleep_start.minute()..r.sleep_end.minute())
            } else {
                Vec::from_iter((0..r.sleep_start.minute() + 1).chain(r.sleep_end.minute()..60))
            }
        })
        .flatten()
        .group_by(|m| m)
        .iter()
        .max_by_key(|(_k, v)| **v)
        .unwrap()
        .0
}

/* Move to seperate file */
trait GroupBy<T, I>
where
    I: Iterator<Item = T>,
{
    fn group_by<F, K>(self, func: F) -> HashMap<K, u64>
    where
        F: Fn(T) -> K,
        K: std::cmp::Eq,
        K: std::hash::Hash;
}

impl<T, I> GroupBy<T, I> for I
where
    I: Iterator<Item = T>,
{
    fn group_by<F, K>(self, func: F) -> HashMap<K, u64>
    where
        F: Fn(T) -> K,
        K: std::cmp::Eq,
        K: std::hash::Hash,
    {
        let mut counts = HashMap::<K, u64>::new();
        for val in self {
            let key = func(val);
            if let Some(i) = counts.get_mut(&key) {
                *i += 1;
            } else {
                counts.insert(key, 1);
            }
        }

        return counts;
    }
}
