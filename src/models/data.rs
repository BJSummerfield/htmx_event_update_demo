use super::User;
use fake::{
    faker::{
        internet::raw::SafeEmail,
        name::raw::{FirstName, LastName},
    },
    locales::EN,
    Fake, Faker,
};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Data {
    pub users: HashMap<u32, User>,
}

impl Data {
    pub fn new() -> Data {
        let mut users = HashMap::new();
        for i in 1..=24 {
            let user = User::new(
                i,
                FirstName(EN).fake(),
                LastName(EN).fake(),
                Faker.fake(),
                SafeEmail(EN).fake(),
            );
            users.insert(i, user);
        }
        Data { users }
    }

    pub fn update_random_user(&mut self) -> Option<&User> {
        if let Some(user_id) = self.get_random_user_id() {
            let updated_user = User::new(
                user_id,
                FirstName(EN).fake(),
                LastName(EN).fake(),
                Faker.fake(),
                SafeEmail(EN).fake(),
            );
            self.users.insert(user_id, updated_user);
            self.users.get(&user_id)
        } else {
            None
        }
    }

    fn get_random_user_id(&self) -> Option<u32> {
        if self.users.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..self.users.len());
            self.users.keys().cloned().nth(random_index)
        }
    }
}
