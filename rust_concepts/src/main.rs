macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

fn main() {
    const NUM_CONSTANT: u32 = 6000 * 32;
    println!("Hello, world! {NUM_CONSTANT}");
    say_hello!()
}



