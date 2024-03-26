use crate::models::{Data, Users};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Handle;

impl Handle {
    pub async fn index(Extension(data): Extension<Arc<Mutex<Data>>>) -> impl IntoResponse {
        let user_list = Users::list_users(data).await;
        println!("{:?}", user_list);
        // For now, just return "Hello, world!"
        Html::from("Hello, world!")
    }
}
