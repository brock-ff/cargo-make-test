#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

fn main() {
    println!("Hello, world!");
}

pub struct Thing {
    pub value: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        #[derive(Debug)]
        // THING will be reset for every test run from cargo-make
        // however, if we run using a standard `cargo test`, it will persist
        static ref THING: Mutex<Thing> = Mutex::new(Thing {
            value: 0,
            });
    }

    // increments the Mutex<Thing> value by amount
    //
    fn increment_thing(amount: u64) {
        let mut thing = THING.lock().unwrap();
        thing.value += amount;
        println!("VALUE: {}", thing.value);
    }

    #[test]
    fn it_increments_thing_1() {
        increment_thing(1);
        assert!(THING.lock().unwrap().value == 1)
    }

    #[ignore]
    #[test]
    fn it_increments_thing_2() {
        increment_thing(2);
        assert!(THING.lock().unwrap().value == 2)
    }

    #[ignore]
    #[test]
    fn it_increments_thing_3() {
        increment_thing(3);
        assert!(THING.lock().unwrap().value == 3)
    }

    #[ignore]
    #[test]
    fn it_increments_thing_4() {
        increment_thing(4);
        assert!(THING.lock().unwrap().value == 4)
    }

    #[ignore]
    #[test]
    fn it_increments_thing_4_20() {
        for _i in 0..20 {
            increment_thing(4);
        }
        assert!(THING.lock().unwrap().value == 80)
    }
}
