use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    println!("function1");
}

fn function2() -> io::Result<()> {
    println!("function2");
}