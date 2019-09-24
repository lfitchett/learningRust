extern crate futures;

use std::{thread, time};

use futures::future;
use futures::future::Map;
use futures::prelude::*;

pub fn test_main() {
    println!("Start");

    let a = future::ok::<i32, i32>(1);
    let b = future::ok::<i32, i32>(5);

    let x = pause(a).and_then(|x| {future::ok(x + 5)});
    let y = pause(b);

    println!("after pause", );

    let xy = x.join(y);

    if let Ok((xval, yval)) = xy.wait() {
        println!("x: {}, y: {}", xval, yval);
    }
}

fn pause<F>(future: F) -> Map<F, fn(i32) -> i32>
where
    F: Future<Item = i32>,
{
    fn add(a: i32) -> i32 {
        println!("Waiting {} seconds.", a);

        // let test = future::ok(1);
        thread::sleep(time::Duration::from_millis(a as u64));
        println!("Waited {} seconds.", a);
        a + 10
    }
    future.map(add)
}
