mod observable {
    use std::collections::HashMap;
    use std::cell::RefCell;

    pub struct Observable {
        callbacks: RefCell<HashMap<u32, Box<Fn()>>>,
        id: u32,
    }

    impl Observable {
        pub fn new() -> Observable {
            Observable {
                callbacks: RefCell::new(HashMap::new()),
                id: 0,
            }
        }

        // pub fn subscribe<F>(&mut self, func: F) -> Subscription
        // where
        //     F: Fn(),
        // {
        //     self.callbacks.insert(self.id, func);
        //     Subscription {}
        // }

        pub fn subscribe(&mut self, func: Box<Fn()>) -> Subscription {
            self.id += 1;
            self.callbacks.borrow_mut().insert(self.id, func);
            Subscription { 
                unsubscribe: self.getUnsubscribe(self.id)
            }
        }


        fn getUnsubscribe(&mut self, id: u32) -> Box<FnOnce()> {
            let tempId = id;
            Box::new(move || {
                println!("{}", tempId);
            })
        }
    }

    pub struct Subscription {
        unsubscribe: Box<FnOnce()>,
    }

    impl Drop for Subscription {
        fn drop(&mut self) {
            self.unsubscribe.as_mut()();
        }
    }
}