use axum::{
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
    Extension, Router,
};
mod models;
mod views;
use models::Data;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use views::Handle;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(Data::new()));
    let app = Router::new()
        .route("/", get(Handle::index))
        .layer(Extension(data.clone()))
        .route("/events", get(sse_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn sse_handler() -> Sse<impl StreamExt<Item = Result<Event, axum::Error>>> {
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(2)))
        .map(|_| Ok(Event::default().data("pizza")));

    Sse::new(stream).keep_alive(KeepAlive::default())
}