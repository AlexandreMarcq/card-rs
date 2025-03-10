pub enum UnoEventType {
    PlayerJoined,
    Draw,
}

pub struct UnoEvent {
    pub event_type: UnoEventType,
}

impl UnoEvent {
    pub fn new(event_type: UnoEventType) -> Self {
        UnoEvent { event_type }
    }
}
