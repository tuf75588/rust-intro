fn main() {
    let s: String = String::from("hello world");
    /* a string slice */
    /* is a part of a String */
    let hello: &str = &s[0..2];
    let world: &str = &s[6..11];
    println!("{hello} {world}");


}


// fn first_word(s: &String) -> usize {
//     let bytes: &[u8] = s.as_bytes();
//     for (_i, &item) in bytes.iter().enumerate() {
//       // check for a space character with byte literal syntax
//       if item == b' ' {
//         println!("{_i}");
//         return _i;
//       }
//     }
//     s.len()
// }

