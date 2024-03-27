use crate::models::{Data, User, Users};
use askama_axum::Template;
use axum::{
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct UsersParams {
    sort_by: Option<String>,
    sort_order: Option<String>,
    prev_sort_by: Option<String>,
}

#[derive(Template)]
#[template(path = "user_table.html")]
pub struct UserTableTemplate {
    users: Vec<User>,
    sort_by: String,
    sort_order: String,
    sort_order_icon: String,
}

pub struct Handler;

impl Handler {
    pub async fn index(Extension(data): Extension<Arc<Mutex<Data>>>) -> impl IntoResponse {
        let user_template = Self::get_users_template(
            UsersParams {
                sort_by: Some("id".to_string()),
                sort_order: Some("asc".to_string()),
                prev_sort_by: None,
            },
            data.clone(),
        )
        .await;

        let template = IndexTemplate {
            title: "Users".to_string(),
            content: user_template.to_string(),
        };

        Html(template.render().unwrap())
    }

    pub async fn users(
        Extension(data): Extension<Arc<Mutex<Data>>>,
        Form(params): Form<UsersParams>,
    ) -> impl IntoResponse {
        let template = Self::get_users_template(params, data.clone()).await;
        Html(template.render().unwrap())
    }

    pub async fn get_users_template(
        params: UsersParams,
        data: Arc<Mutex<Data>>,
    ) -> UserTableTemplate {
        let sort_by = params.sort_by.unwrap_or_else(|| "id".to_string());
        let prev_sort_by = params.prev_sort_by.unwrap_or_else(|| "".to_string());
        let sort_order = if sort_by == prev_sort_by {
            match params
                .sort_order
                .unwrap_or_else(|| "asc".to_string())
                .as_str()
            {
                "asc" => "desc".to_string(),
                _ => "asc".to_string(),
            }
        } else {
            "asc".to_string()
        };

        let users = Users::list_users(data, Some(sort_by.clone()), Some(sort_order.clone())).await;

        let sort_order_icon = match sort_order.as_str() {
            "asc" => "▲",
            "desc" => "▼",
            _ => "",
        };

        UserTableTemplate {
            users,
            sort_by,
            sort_order,
            sort_order_icon: sort_order_icon.to_string(),
        }
    }
}
