enum NameMatch {
    Andrew,
    Bob,
    John,
    Mary,
}
fn main() {
    let name = NameMatch::Andrew;
    find_match(name);
}

fn find_match(name: NameMatch) {
    match name {
        NameMatch::Andrew => println!("Lucky penny"),
        NameMatch::Bob => println!("Not so lucky"),
        NameMatch::John => println!("Lucky Dime"),
        NameMatch::Mary => println!("Lucky Quarter"),
    }
}
