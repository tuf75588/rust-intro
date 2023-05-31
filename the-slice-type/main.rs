fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    s.clear();


    /* word is still available after we clear s */
    println!("{word}");


}


fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();
    for (_i, &item) in bytes.iter().enumerate() {
      // check for a space character with byte literal syntax
      if item == b' ' {
        println!("{_i}");
        return _i;
      }
    }
    s.len()
}
