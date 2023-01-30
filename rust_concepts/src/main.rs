macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

// scalar types -- integers, floating-point numbers, booleans, and characters

//  Integer Types
//  Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)   
// unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size) - unsigned integers can only store positive numbers


fn main() {
    const NUM_CONSTANT: u32 = 6000 * 32;
    println!("Hello, world! {NUM_CONSTANT}");
    say_hello!()
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

}



 