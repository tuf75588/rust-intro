struct Post {
    active: bool,
    author: String,
    email: String,
    sign_in_count: u64,
}

struct User {
    email: String,
    username: String,
}

fn main() {
    println!("Hello, world!");
    // struct instance
    let mut post = Post {
        active: true,
        author: String::from("Andrew"),
        email: String::from("atd285@gmail.com"),
        sign_in_count: 1,
    };
    let mut user1 = build_user(String::from("atd285@gmail.com"), String::from("atd285"));
}

fn build_user(email: String, username: String) -> User {
    User { email, username }
}
