struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User{
    User{
        email, //short hand syntax for email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(String::from("foo@example.com"), String::from("someusername123"));
    let user2 = User{username: String::from("hamel"), ..user1}; //like destrcuturing in js
    println!("user2: {}", user2.username);


    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("black: {}, origin: {}", black.0, origin.0);
}
