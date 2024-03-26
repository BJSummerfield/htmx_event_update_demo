use super::{data::Data, user::User};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Users;

impl Users {
    // pub fn new(data: Arc<Mutex<Data>>) -> Users {
    //     Users { data }
    // }
    //
    // pub async fn add_user(&self, user: User) {
    //     let mut data = self.data.lock().await;
    //     let id = data.users.len() as u32 + 1;
    //     data.users.insert(id, user);
    // }
    //
    // pub async fn get_user(&self, user_id: u32) -> Option<User> {
    //     let data = self.data.lock().await;
    //     data.users.get(&user_id).cloned()
    // }

    pub async fn list_users(data: Arc<Mutex<Data>>) -> Vec<User> {
        let data = data.lock().await;
        data.users.values().cloned().collect()
    }
}
