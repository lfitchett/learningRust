pub mod day1 {
    use std::collections::HashSet;

    pub fn main() {
        let mut sum = 0;
        let mut freq = HashSet::new();

        loop {
            let file = super::file_read("data/1.txt");
            let lines = file.split("\n");
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
