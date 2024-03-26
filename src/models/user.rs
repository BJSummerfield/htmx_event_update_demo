#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub email: String,
}

impl User {
    pub fn new(id: u32, first_name: String, last_name: String, age: u8, email: String) -> User {
        User {
            id,
            first_name,
            last_name,
            age,
            email,
        }
    }
}
