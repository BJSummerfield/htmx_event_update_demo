use super::User;
use crate::utils::{EventEmitter, SseEvent};
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
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

#[derive(Debug, Clone)]
pub struct Data {
    pub users: HashMap<u32, User>,
    event_emitter: Arc<EventEmitter>,
}

impl Data {
    pub fn new(event_emitter: Arc<EventEmitter>) -> Data {
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

        let data = Data {
            users,
            event_emitter,
        };
        data.clone().event_loop();
        data
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

            // Emit the updated user event
            self.event_emitter.send(SseEvent::UserUpdated(user_id));

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

    fn event_loop(self) {
        let data = Arc::new(tokio::sync::Mutex::new(self));

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                let mut data = data.lock().await;
                data.update_random_user();
            }
        });
    }
}
