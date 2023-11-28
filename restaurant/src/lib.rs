use std::fmt::Result;
fn function1() -> Result<(), std::fmt::Error> {
    println!("function1");
    Ok(())
}

fn function2() -> Result<(), std::fmt::Error> {
    println!("function2");
    Ok(())
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

