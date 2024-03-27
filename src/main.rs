use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, get_service, post},
    Router,
};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::services::ServeDir;

mod models;
mod utils;
mod views;

use models::Data;
use utils::{EventEmitter, SseEvent};
use views::Handler;

#[derive(Clone)]
pub struct AppState {
    data: Arc<Mutex<Data>>,
    event_emitter: Arc<EventEmitter>,
}

#[tokio::main]
async fn main() {
    let event_emitter = Arc::new(EventEmitter::new());
    let data = Arc::new(Mutex::new(Data::new()));

    let app_state = AppState {
        data: data.clone(),
        event_emitter: event_emitter.clone(),
    };

    let app = Router::new()
        .route("/", get(Handler::index))
        .route("/users", post(Handler::users))
        .nest_service("/static", get_service(ServeDir::new("./static")))
        .route("/events", get(sse_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn sse_handler(
    State(app_state): State<AppState>,
) -> Sse<impl futures::Stream<Item = Result<Event, axum::Error>>> {
    let stream =
        BroadcastStream::new(app_state.event_emitter.subscribe()).map(|event| match event {
            Ok(SseEvent::UserUpdated(user_id)) => Ok(Event::default()
                .event("user_updated")
                .data(user_id.to_string())),
            Err(e) => Err(axum::Error::new(e)),
        });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
