use rand::Rng;

fn main() {
    // generate an array words that are all 5 letters long
    let words = [
        "abbas", "abbes", "abbey", "abbot", "abeam", "abele", "abets", "abhor", "abide", "abies",
        "abler", "ables", "abmho", "abode", "abohm", "aboil", "aboon", "abort", "about", "above",
        "abrin", "abris", "abuna", "abuse",
    ];
    let random_num = rand::thread_rng().gen_range(0..words.len());
    println!("Hello, world!, {}", words[random_num]);
}
