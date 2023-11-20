use std::fmt::Result;
use std::fmt::Result as IOResult;

fn function1() -> Result {
    println!("function1");
}

fn function2() -> IOResult<()> {
    println!("function2");
}
