use super::file;
use std::collections::HashMap;

pub(crate) fn main() {
    println!("Test");
    let file = file::file_read("data/5.txt");
    let mut lines = file.lines();
    let input = lines.nth(0).unwrap().to_string();

    let ascii_lower = "abcdefghijklmnopqrstuvwxyz";
    let mut alphaMap = HashMap::new();
    for lower in ascii_lower.chars() {
        let upper = lower.to_ascii_uppercase();
        alphaMap.insert(lower, upper);
        alphaMap.insert(upper, lower);
    }

    println!("{}", reduce(input, &alphaMap));
}

fn reduce(input: String, map: &HashMap<char, char>) -> String {
    let mut prev = ' ';
    for (i, curr) in input.chars().enumerate() {
        if let Some(p) = map.get(&prev) {
            if *p == curr {
                let temp1 = input.get(..i - 1);
                let temp2 = input.get(i + 1..);

                return reduce(temp1.unwrap().chars().chain(temp2.unwrap().chars()).collect(), map);
            }
        }

        prev = curr;
    }

    input
}
