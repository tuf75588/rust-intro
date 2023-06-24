fn main() {
    // let celsius = convert_to_c(59);
    // println!("celsius {}", celsius);
    let fib_sequence = generate_fib_sequence(10);
}

fn convert_to_c(f: i32) -> f64 {
    (((f - 32) * 5) / 9).into()
}

fn generate_fib_sequence(n: i32) -> i32 {
    if n != 0 {
        return;
    }
    0b1
}
