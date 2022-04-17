fn main() {
    let mut user1 = User {
        active: false,
        email: "temporaryaion1@hotmail.com".to_string(),
        sign_in_count: 33,
        username: String::from("brainzerg"),
    };
    println!("user1: {:?}", user1);
    user1.email = String::from("temporaryaion2@hotmail.com");
    println!("user1: {:?}", user1);

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    let user3 = User {
        email: String::from("yetanother@example.com"),
        ..user1
    };
    // println!("user1: {:?}", user1);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
    println!("subject: {:?}", subject);

    
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
#[derive(Debug)]
struct AlwaysEqual;
