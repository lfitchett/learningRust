use std::collections::HashMap;

pub fn main() {
    let file = file_read("data/2.txt");
    let lines = file.lines();
    for line in lines {
        let counts = line.chars().group_by(|c| c);
        let totals = counts.values().map(|i| *i).group_by(|c| c);

        for (k, v) in totals {
            println!("{}: {}", k, v);
        }
    }
}

trait GroupBy<T, I>
where
    I: Iterator<Item = T>,
{
    fn group_by<F, K>(&mut self, func: F) -> HashMap<K, u64>
    where F: Fn(T) -> K,
    K: std::cmp::Eq,
    K: std::hash::Hash;
}

impl<T, I> GroupBy<T, I> for I
where
    I: Iterator<Item = T>,
{
    fn group_by<F, K>(&mut self, func: F) -> HashMap<K, u64> 
    where F: Fn(T) -> K,
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

fn file_read(f: &str) -> String {
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new(f);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => s,
    }
}
