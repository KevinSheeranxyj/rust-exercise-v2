use crate::example01::User;
use crate::example01::find_user_by_name;
use crate::example02::{ unwrap_check};

mod example01;
mod example02;

fn main() {

    let users = vec![
        User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
    },
        User {
            username: String::from("bob"),
            email: String::from("bob@example.com")
        }
    ];

    match find_user_by_name(users, "alice") {
        Some(user) => {println!("Found user {} with email {}", user.username, user.email)},
    None => println!("user not found")}

    /*
        example02: using unwrap
    */
    let some_value: Option<i32> = Some(10);
    let no_value: Option<i32> = None;

    unwrap_check(some_value);
}
