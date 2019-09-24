pub fn test() {
    println!("Test");
}

pub mod iterator_test {
    pub struct FizzBuzz {
        curr: u32,
        stop: u32,
    }

    impl FizzBuzz {
        pub fn new(stop: u32) -> FizzBuzz {
            FizzBuzz { curr: 0, stop }
        }
    }

    impl Iterator for FizzBuzz {
        type Item = String;

        fn next(&mut self) -> Option<String> {
            self.curr += 1;
            if self.curr > self.stop {
                None
            } else {
                Some(
                    String::new()
                        + if self.curr % 3 == 0 { "Fizz" } else { "" }
                        + if self.curr % 5 == 0 { "Buzz" } else { "" }
                        + &self.curr.to_string(),
                )
            }
        }
    }

    enum Test {
        Range(std::ops::Range<u64>),
        Chain(std::iter::Chain<std::ops::Range<u64>, std::ops::Range<u64>>),
    }

    impl Iterator for Test {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Test::Range(r) => r.next(),
                Test::Chain(c) => c.next(),
            }
        }
    }

    fn foo(x: bool) -> Test {
        if x {
            Test::Range(1..5)
        } else {
            Test::Chain((1..5).chain(10..15))
        }
    }
}
