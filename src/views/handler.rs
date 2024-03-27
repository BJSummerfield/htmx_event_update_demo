use crate::models::{Data, User, Users};
use askama_axum::Template;
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    content: String,
}

#[derive(Template)]
#[template(path = "user_table.html")]
struct UserTableTemplate {
    users: Vec<User>,
}

pub struct Handler;

impl Handler {
    pub async fn index(Extension(data): Extension<Arc<Mutex<Data>>>) -> impl IntoResponse {
        let users = Users::list_users(data).await;
        let user_table_template = UserTableTemplate { users };
        let user_table_html = user_table_template.render().unwrap();

        let template = IndexTemplate {
            title: "User List".to_string(),
            content: user_table_html,
        };
        Html(template.render().unwrap())
    }
}
