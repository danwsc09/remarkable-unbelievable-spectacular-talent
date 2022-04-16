fn main() {
    let mut user1 = User {
        active: false,
        email: "temporaryaion1@hotmail.com".to_string(),
        sign_in_count: 33,
        username: String::from("brainzerg"),
    };
    println!("user1: {:?}", user1);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
