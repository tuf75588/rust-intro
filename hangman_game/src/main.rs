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
    println!("Please guess a word");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("you guessed {guess}")
}
