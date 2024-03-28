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
    pub event_loop_toggle: Arc<Mutex<bool>>,
    event_loop_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
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
            event_loop_toggle: Arc::new(Mutex::new(false)),
            event_loop_handle: Arc::new(Mutex::new(None)),
        };
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

    pub async fn start_event_loop(&self) {
        let event_loop_toggle = self.event_loop_toggle.clone();
        let event_loop_handle = self.event_loop_handle.clone();
        let mut cloned_self = self.clone();

        let handle = tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(5000));
            loop {
                interval.tick().await;
                if *event_loop_toggle.lock().await {
                    cloned_self.update_random_user().await;
                } else {
                    break;
                }
            }
        });

        *event_loop_handle.lock().await = Some(handle);
    }

    pub async fn stop_event_loop(&self) {
        let mut event_loop_handle = self.event_loop_handle.lock().await;
        if let Some(handle) = event_loop_handle.take() {
            handle.abort();
        }
    }

    pub async fn toggle_event_loop(&self) {
        let mut event_loop_toggle = self.event_loop_toggle.lock().await;
        *event_loop_toggle = !*event_loop_toggle;

        if *event_loop_toggle {
            self.start_event_loop().await;
        } else {
            self.stop_event_loop().await;
        }
    }
}
