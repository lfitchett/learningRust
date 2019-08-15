#![allow(dead_code)]

mod random;
mod advent;

fn main() {

    let x = {
        let y = 6;
        y += 6;
        let c = 7;
        y + c
    };

    advent::day6::main();
}
