struct User {
    _active: bool,
    _username: String,
    email: String,
    _sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

pub fn main() {
    let mut user1 = User {
        _active: true,
        _username: String::from("user1"),
        email: String::from("user1@example.com"),
        _sign_in_count: 1,
    };

    user1.email = String::from("user1_changed@example.com");

    // after the creation of user2, user1 is no longer valid as a whole
    // since user1.username is moved into user2. there're no issues with
    // the other fields as they are scalar types implementing the Copy trait
    let _user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    // can be destructured
    // properties accessed with dot notation, followed by the index
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // unit-like structs can be useful when its needed to implement a trait
    // on a type but don't have any data that you want to store in the type itself
    let _subject = AlwaysEqual;
}

fn _build_user(email: String, username: String) -> User {
    User {
        _active: true,
        _username: username,
        email,
        _sign_in_count: 1,
    }
}

// DOES NOT COMPILE
// the initial User type definition used owned `String`s rather than
// the &str string slice type. This is a deliberate choice becase we
// want each instance of this struct to own all of its data and
// for that data to be valid for as long as the struct itself is valid.
//
// it's also possible to store references to data owned by something
// else, but doing so requires the use of lifetimes. lifetimes ensure
// that the data referenced by a struct is valid for as long as the
// struct itself is valid.
// fn ownership_of_struct_data() {
//     struct User {
//         active: bool,
//         username: &str,
//         email: &str,
//         sign_in_count: u64,
//     }

//     let user = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }
