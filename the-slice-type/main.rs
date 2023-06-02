fn main() {
    let mut s: String = String::from("hello world");
    /* a string slice */
    /* is a part of a String */
    let word = first_word(&s);


    println!("the first word is: {}", word);
}

// &str represents the string slice type
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
