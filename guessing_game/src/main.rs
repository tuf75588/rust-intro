use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // returns a new instance of string
    let mut guess = String::new();
    // the full purpose of this line is to read a line from the user and append it to the string guess
    // the & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times
    io::stdin().read_line(&mut guess).expect("Failed to read line");


    // {} is a placeholder for a variable that will be printed later on. Think of {} as little crab princers that hold a value in place.
    // When printing the value of a variable, the variable name can go inside the curly brackets. 
    println!("You guessed: {guess}");

    // creating variables
    // variables are immutable by default
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}