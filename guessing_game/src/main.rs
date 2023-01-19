use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    println!("Please input your guess.");

    // returns a new instance of string
    let mut guess = String::new();
    // the full purpose of this line is to read a line from the user and append it to the string guess
    // the & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // {} is a placeholder for a variable that will be printed later on. Think of {} as little crab princers that hold a value in place.
    // When printing the value of a variable, the variable name can go inside the curly brackets.
    println!("You guessed: {guess}");

    // creating variables
    // variables are immutable by default
    // control flow
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}