

pub struct User {
    pub username: String,
    pub email: String,
}


pub fn find_user_by_name(users: Vec<User>, username: &str) -> Option<User> {
    for user in users {
        if user.username == username {
            return Some(user);
        }
    }
    None
}