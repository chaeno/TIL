mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order()
    }
    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use crate::front_of_house::hosting;
use self::front_of_house::hosting;
use self::front_of_house::hosting::add_to_waitlist;

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {
// }

// fn function2() -> IoResult {
// }

// use std::{io, cmp::Ordering};
use std::io::{self, Write};

use std::collections::*;