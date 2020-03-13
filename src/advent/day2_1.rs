use std::collections::HashMap;
use super::file;

pub(crate) fn main() {
    let file = file::file_read("data/2.txt");
    let lines = file.lines();

    let mut totals = HashMap::new();
    totals.insert(2, 0);
    totals.insert(3, 0);
    for line in lines {
        // count num of each char
        let char_counts = line.chars().group_by(|c| c);
        // count num of occurances of each count
        let line_totals = char_counts.iter().group_by(|(_k, v)| *v);

        for (k, v) in totals.iter_mut() {
            if line_totals.contains_key(k) {
                *v += 1;
            }
        }
    }
    for (k, v) in totals.iter() {
        println!("{}: {}", k, v);
    }
    println!(
        "Checksum: {}",
        totals.get(&2).unwrap() * totals.get(&3).unwrap()
    )
}

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

        counts
    }
}