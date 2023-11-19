use rand::Rng;
use std::io;

fn main() {
    // generate an array words that are all 5 letters long
    let words = [
        "abbas", "abbes", "abbey", "abbot", "abeam", "abele", "abets", "abhor", "abide", "abies",
        "abler", "ables", "abmho", "abode", "abohm", "aboil", "aboon", "abort", "about", "above",
        "abrin", "abris", "abuna", "abuse",
    ];
    let _random_num = rand::thread_rng().gen_range(0..words.len());
    println!("Please guess a word, {}", _random_num);
    // pick a correct word
    let correct_word = words[_random_num];
    println!("The correct word is {}", correct_word);
    let mut guess = String::new();

    // loop until the correct word is guessed
    loop {
        // get a guess from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // check if the guess is correct
        if guess.trim() == correct_word {
            println!("You guessed correctly!");
            break;
        } else {
            println!("You guessed incorrectly!");
        }
    }
}
