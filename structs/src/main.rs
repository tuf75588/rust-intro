struct Post {
    active: bool,
    author: String,
    email: String,
    sign_in_count: u64,
}

struct User {
    email: String,
    username: String,
    author: String,
    sign_in_count: u64,
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
    let mut user1 = build_user(
        String::from("atd285@gmail.com"),
        String::from("atd285"),
        String::from("Andrew"),
        55,
    );
    let mut post1: Post = build_post(true, String::from("andreŵ"), String::from("Andrew"), 55);
}

fn build_user(email: String, username: String, author: String, sign_in_count: u64) -> User {
    User {
        email,
        username,
        author,
        sign_in_count,
    }
}

fn build_post(active: bool, author: String, email: String, sign_in_count: u64) -> Post {
    Post {
        active,
        author,
        email,
        sign_in_count,
    }
}
