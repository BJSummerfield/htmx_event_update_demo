use super::{data::Data, user::User};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Users;

impl Users {
    pub async fn list_users(data: Arc<Mutex<Data>>) -> Vec<User> {
        let data = data.lock().await;
        data.users.values().cloned().collect()
    }
}
