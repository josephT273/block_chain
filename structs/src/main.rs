struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

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

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;

    println!("{} {} {}", x, y, z);
    println!("{} {} {}", black.0, black.1, black.2);

    let email = String::from("someone@example.com");
    let username = String::from("someone");
    let mut user1 = build_user(email, username);

    user1.email = String::from("updated@email.com");

    let user2 = User {
        email: String::from("anotherman@example.com"),
        ..user1
    };

    println!("{}", user2.active);
    println!("{}", user2.email);
    println!("{}", user2.username);
    println!("{}", user2.sign_in_count);
}
