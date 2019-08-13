use std::collections::HashSet;
use super::file;

pub(crate) fn main() {
    let mut sum = 0;
    let mut freq = HashSet::new();

    loop {
        let file = file::file_read("data/1.txt");   
        let lines = file.lines();
        for line in lines {
            if !freq.insert(sum) {
                println!("{}", sum);
                return;
            }

            let sign = if line[0..1] == *"+" { 1 } else { -1 };

            let temp = &line[1..line.chars().count()].to_string();
            match temp.parse::<i64>() {
                Ok(num) => sum += num * sign,
                Err(e) => println!("{}", e),
            }

            println!("{}", sum);
        }
    }
}

