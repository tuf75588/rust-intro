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
    say_hello!();
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;

    let f: bool = false; // with explicit type annotation

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // arrays
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let first = a[0];
    let second = a[1];

    // array
    let a = [1, 2, 3, 4, 5];
    // statics 
    static LANGUAGE: &str = "Rust";


    // variable shadowing examples
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The inner scope value of x is: {x}");

    }
    println!("The value of x is: {x}");

 
    // parse a string into a number
    let strings = "45";
    let strings: String = strings.parse().expect("Not a number!");
    println!("The value of strings is: {strings}cd ")
}



 