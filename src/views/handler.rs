use crate::models::{Data, User, Users};
use crate::AppState;
use askama_axum::Template;
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
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
#[derive(Template)]
#[template(path = "user_row.html")]
pub struct UserRowTemplate {
    user: User,
}
pub struct Handler;

impl Handler {
    pub async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
        let data = app_state.data.clone();
        println!("{:?}", data);

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
    pub async fn user(
        State(app_state): State<AppState>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let user_id = params.get("id").and_then(|id| id.parse().ok()).unwrap_or(0);
        let data = app_state.data.lock().await;
        let users = data.users.lock().await;
        let user = users.get(&user_id).cloned();

        match user {
            Some(user) => {
                let template = UserRowTemplate { user };
                Html(template.render().unwrap())
            }
            None => Html("User not found".to_string()),
        }
    }
    pub async fn users(
        State(app_state): State<AppState>,
        Form(params): Form<UsersParams>,
    ) -> impl IntoResponse {
        let template = Self::get_users_template(params, app_state.data.clone()).await;
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
