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

// `#[derive(Debug)]` is an option we need to explicitelly set
// if we want to print debug info about an instance of this specific struct
// See Line 55
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Definitions for Rectangle methods
impl Rectangle {
    // This is called a `method` which is similar to a function
    // except that it is only defined for the context of this struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        if rectangle.area() > self.area() {
            return false;
        }
        return true;
    }

    // Functions defined in an `impl` block that do not take `&self`
    // as a parameter are known as associated functions
    // They are still associated with the struct but do not actually
    // have an instance of it
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

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

    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rec1 is {} square pixels", area(&rec1));

    println!("rec1 is {:?}", rec1);

    println!("The area of the rec1 is {} square pixels", rec1.area());

    let rec2 = Rectangle {
        width: 20,
        height: 10,
    };

    let rec3 = Rectangle {
        width: 200,
        height: 100,
    };

    println!("can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("can rec1 hold rec3? {}", rec1.can_hold(&rec3));

    // Example of calling an associated function
    let sq1 = Rectangle::square(3);

    println!("sq1 is {:?}", sq1);
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

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
