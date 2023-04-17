fn main() {
    // the whole struct must be mutable
    let mut user1 = User {
        active: true,
        username: String::from("rust"),
        email: String::from("rust@example.com"),
        sign_in_count: 19,
    };

    user1.email = String::from("cargo@example.com");

    // user1's properties are moved to user2
    let user2 = User {
        email: String::from("rustaceans@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

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

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;
