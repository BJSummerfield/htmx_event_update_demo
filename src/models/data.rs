use super::User;
use fake::{
    faker::{
        internet::raw::SafeEmail,
        name::raw::{FirstName, LastName},
    },
    locales::EN,
    Fake, Faker,
};
use std::collections::HashMap;

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
}
