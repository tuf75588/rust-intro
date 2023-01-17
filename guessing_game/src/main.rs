use std::io;

fn main() {
    println!("Guess the number"!);

    println!("Please input your guess.");

    // returns a new instance of string
    let mut guess = String::new();
    // the full purpose of this line is to read a line from the user and append it to the string guess
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");

    // creating variables
    // variables are immutable by default
    let apples = 5;
    let mut bananas = 5;
}