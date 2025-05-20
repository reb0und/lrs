fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("abcasds"),
        sign_in_count: 1,
    };

    user1.email = String::from("asds");

    // Create a new User instance without using struct update syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("ad"),
        sign_in_count: user1.sign_in_count,
    };

    // Using struct update syntax
    let user3 = User {
        email: String::from("another"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Struct assignment
    let subject = AlwaysEqual;
}

// Unit-Like struct definition
struct AlwaysEqual;

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
