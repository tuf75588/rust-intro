use std::fmt::Result;
use std::fmt::Result as IOResult;

fn function1() -> Result {
    println!("function1");
}

fn function2() -> IOResult<()> {
    println!("function2");
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to Waitlist");
        }
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}