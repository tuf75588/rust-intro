#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
dbg!(&rect1);
println!("{:?}", &rect1);
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

// adding useful information with derived traits
