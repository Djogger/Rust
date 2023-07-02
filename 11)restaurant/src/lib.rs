// Gleb skazal, chto ne stoi delat neskolko moduley v odnom faile lib.rs, no zdes iskluchenie. 
// Oh uz etot lomaniy angliskiy, kak budto siel gorachuu cartoshku)) Vse etot UTF-8 vinovat, tochnee ego otsutstvie :(

// Paths on modules:

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path:
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path:
    front_of_house::hosting::add_to_waitlist();
}

// Relative path with 'super':

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// Making public stuctures (struct) and enumerations (enum):

// 1) struct:
mod back_of_house1 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant1() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house1::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// 2) enum:

mod back_of_house2 {
    pub enum Appetizer{
        Soup(i32),
        Salad(char)
    }
}

pub fn eat_at_restaurant2() {
    let order1 = back_of_house2::Appetizer::Soup(-5);
    let order2 = back_of_house2::Appetizer::Salad('A');
}

// Bringing paths into scope with the 'use' keyword:

mod front_of_house1 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house1::hosting;

pub fn eat_at_restaurant3() {
    hosting::add_to_waitlist();
}

// Creating idiomatic paths with 'use':


// The case when we want to connect two elements with the same names from different parent modules:

//use std::fmt;
//use std::io;

//fn function1() -> fmt::Result {}
//fn function2() -> io::Result<()> {}


// Providing new names using the 'as' keyword:

//use std::fmt::Result;
//use std::io::Result as ioResult;

//fn function1() -> Result {}
//fn function2() -> ioResult<()> {}


// Re-exporting names with pub use:

// Here i don't really understand the meaning of that 'pub use', so i go to next example.


// Using external packages:

// All these we knew from Second chapter of Rust Book.
// it's about rand.


// Using nested paths to reduce long use lists:

// 1-st variant is long:

//use std::cmp::Ordering;
//use std::io;

// 2-nd variant is shorter, so that's why it's better:

//use std::{cmp::Ordering, io};

// same as 1-st variant:

//use std::io;
//use std::io::Write;

// same as 2-nd variant:

use std::io::{self, Write};


// Operator * (glob):

use std::collections::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

