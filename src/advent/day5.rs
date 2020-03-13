use super::file;
use linked_list::Cursor;
use linked_list::LinkedList;
use std::collections::HashMap;
use std::iter::FromIterator;

const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";

pub(crate) fn main() {
    let mut alpha_map = HashMap::new();
    for lower in ALPHA_LOWER.chars() {
        let upper = lower.to_ascii_uppercase();
        alpha_map.insert(lower, upper);
        alpha_map.insert(upper, lower);
    }

    let file = file::file_read("data/5.txt");
    let mut input: LinkedList<char> = file.lines().nth(0).unwrap().to_string().chars().collect();
    reduce(&mut input.cursor(), &alpha_map);
    let result = String::from_iter(input);

    println!("{}", result.len());
    println!("{}", reduce_further(result, &alpha_map));
}

fn reduce(cursor: &mut Cursor<char>, alpha_map: &HashMap<char, char>) {
    while cursor.peek_next() != None {
        let prev = if let Some(p) = cursor.peek_prev() {
            *p
        } else {
            cursor.next();
            continue;
        };
        let next = if let Some(n) = cursor.peek_next() {
            *n
        } else {
            continue;
        };
        // print!("{} {}", prev, next);

        if let Some(p) = alpha_map.get(&prev) {
            if *p == next {
                cursor.prev();
                cursor.remove();
                cursor.remove();
            } else {
                cursor.next();
            }
        }
    }
}

fn reduce_further(input: String, alpha_map: &HashMap<char, char>) -> usize {
    ALPHA_LOWER
        .chars()
        .map(|char_to_check| {
            let mut result: LinkedList<char> = input
                .chars()
                .filter(|c| *c != char_to_check && *c != char_to_check.to_ascii_uppercase())
                .collect();
            reduce(&mut result.cursor(), alpha_map);
            result.len()
        })
        .min()
        .unwrap()
}

fn has_pair(input: String, alpha_map: &HashMap<char, char>) -> bool {
    let mut iter = input.chars();
    let mut prev = iter.next().unwrap();
    while let Some(curr) = iter.next() {
        if let Some(p) = alpha_map.get(&prev) {
            if *p == curr {
                return true;
            }
        }

        prev = curr;
    }

    false
}
