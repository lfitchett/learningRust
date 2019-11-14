#![allow(dead_code)]
#![allow(unused_imports)]

mod advent;
mod future;
mod random;
use std::path::{Path, PathBuf};


fn main() {
    future::test_main();

    match uptime_lib::get() {
        Ok(uptime) => {
            println!("uptime: {} seconds", uptime.num_milliseconds() as f64 / 1000.0);
        }
        Err(err) => {
            eprintln!("uptime: {}", err);
            std::process::exit(1);
        }
    }
}
