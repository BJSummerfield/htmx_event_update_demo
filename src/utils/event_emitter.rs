use axum::response::sse::Event;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum SseEvent {
    UserUpdated(u32),
    // Add more event variants as needed
}

#[derive(Debug, Clone)]
pub struct EventEmitter {
    sender: broadcast::Sender<Event>,
}

impl EventEmitter {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        EventEmitter { sender }
    }

    pub fn send(&self, event: SseEvent) {
        match event {
            SseEvent::UserUpdated(user_id) => {
                let message_event = Event::default()
                    .event("message")
                    .data(format!("User Updated: ID:{}", user_id));

                let update_event = Event::default()
                    .event(format!("user_{}", user_id))
                    .data(user_id.to_string());

                let _ = self.sender.send(message_event);
                let _ = self.sender.send(update_event);
            } // Handle other event variants and build appropriate Event instances
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}
