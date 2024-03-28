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
use tokio::{sync::Mutex, time};

#[derive(Debug, Clone)]
pub struct Data {
    pub users: Arc<Mutex<HashMap<u32, User>>>,
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
            users: Arc::new(Mutex::new(users)),
            event_emitter,
        };
        data.clone().event_loop();
        data
    }

    async fn update_random_user(&mut self) {
        if let Some(user_id) = self.get_random_user_id().await {
            let updated_user = User::new(
                user_id,
                FirstName(EN).fake(),
                LastName(EN).fake(),
                Faker.fake(),
                SafeEmail(EN).fake(),
            );
            let mut users_map = self.users.lock().await;
            users_map.insert(user_id, updated_user.clone());
            self.event_emitter.send(SseEvent::UserUpdated(user_id));
        }
    }

    async fn get_random_user_id(&self) -> Option<u32> {
        let users = self.users.lock().await;
        if users.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..users.len());
            users.keys().cloned().nth(random_index)
        }
    }

    fn event_loop(&self) {
        let mut cloned_self = self.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(500));
            loop {
                interval.tick().await;
                cloned_self.update_random_user().await;
            }
        });
    }
}
