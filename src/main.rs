// Project: structs
// Author: Greg Folker

struct User {
    username: String,
    email: String,
    _sign_in_count: u64,
    _active: bool,
}

// Structs can also be defined as "tuple structs"
// Tuple structs do not have names associated with their fields, just
// the type of the field
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
	println!("Hello, World!");

    let email = String::from("example@example.com");
    let username = String::from("someuser123");

    let user1 = build_user(email, username);

    println!("user1 email={} name={}", user1.email, user1.username);

    // Using struct update syntax, you can use `..` to tell the
    // compiler to set the remaining fields to the same value
    // as those fields in the given instance
    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser456"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,

        // Field definitions can optionally be shorthanded when
        // the variables have the same name as the fields
        //
        // email,
        // username,

        _active: true,
        _sign_in_count: 1,
    }
}
