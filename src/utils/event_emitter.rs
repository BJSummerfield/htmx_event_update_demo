use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum SseEvent {
    UserUpdated(u32),
    // Add more event variants as needed
}

pub struct EventEmitter {
    sender: broadcast::Sender<SseEvent>,
}

impl EventEmitter {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        EventEmitter { sender }
    }

    pub fn send(&self, event: SseEvent) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SseEvent> {
        self.sender.subscribe()
    }
}
