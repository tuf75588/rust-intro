fn main() {
    let celsius = convert_to_c(59);
    println!("celsius {}", celsius);
}

fn convert_to_c(f: i32) -> f64 {
    ((f - 32) * 5/9).into()
}

