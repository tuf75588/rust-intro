fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length (s: &String) -> usize { // & is a reference
    s.len()
    // s is a reference to a String
    // s goes out of scope, but because it does not have ownership of what
    // it refers to, nothing happens.
    // it is not dropped
}

