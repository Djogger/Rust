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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

