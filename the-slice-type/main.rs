fn main() {
    println!("{}", first_word(&String::from("hello world")));
    let str_slice: &[&str] = &["one", "two", "three"];
    println!("{:?}", str_slice)
}

fn log_num(s: u32) -> u32 {
    return s;
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("{}", bytes[0]);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
