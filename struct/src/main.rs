use std::mem::size_of;

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

    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels", rec1.area());
    println!("rect1 is {:?}", rec1);

    let rec2 = Rectangle {
        width: dbg!(30 * 3),
        height: 70,
    };

    dbg!(&rec2);

    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rect1 hold rec3? {}", rec1.can_hold(&rec3));
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
