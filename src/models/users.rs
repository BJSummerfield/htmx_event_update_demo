use super::{data::Data, user::User};
use std::sync::Arc;

pub struct Users;

impl Users {
    pub async fn list_users(
        data: Arc<Data>,
        sort_by: Option<String>,
        sort_order: Option<String>,
    ) -> Vec<User> {
        let mut users = {
            let users_map = data.users.lock().await;
            users_map.values().cloned().collect::<Vec<User>>()
        };

        if let Some(column) = sort_by {
            let order = sort_order.unwrap_or_else(|| "asc".to_string());
            let sort_fn = match column.as_str() {
                "id" => |a: &User, b: &User| a.id.cmp(&b.id),
                "first_name" => |a: &User, b: &User| a.first_name.cmp(&b.first_name),
                "last_name" => |a: &User, b: &User| a.last_name.cmp(&b.last_name),
                "age" => |a: &User, b: &User| a.age.cmp(&b.age),
                "email" => |a: &User, b: &User| a.email.cmp(&b.email),
                _ => return users,
            };

            if order == "asc" {
                users.sort_by(sort_fn);
            } else {
                users.sort_by(|a, b| sort_fn(b, a));
            }
        }

        users
    }
}
