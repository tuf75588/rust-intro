fn main() {
   let num: i32 = -5;
   if num > 10 {
    println!("{} is greater than 10", num)
   } else {
    println!("{} is less than 10", num)
   }

   let number = if num > 10 { 10 } else { 5 };
   println!("The value of number is: {number}");

   let mut counter: i32 = 0;
   let result = loop {
      counter += 1;
      if counter == 10 {
         break counter * 2;
         
      }
   };
   println!("The result is {}", result);
}
