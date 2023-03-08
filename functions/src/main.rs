fn main() {
    println!("Hello, world!");

    let x = five();
    let y = x_plus_one(65);
    println!("The value of x is {x} {}", y);
}


fn five() -> i32 {
    5
}

fn x_plus_one(x:i32) -> i32 {
    x + 1
}