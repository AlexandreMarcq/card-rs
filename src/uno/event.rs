use super::card::UnoCard;

#[derive(PartialEq)]
pub enum UnoEventType {
    Discard { card: UnoCard, index: usize },
    Draw,
    NoCard,
    PlayerJoined,
}

pub struct UnoEvent {
    pub event_type: UnoEventType,
    pub player_id: u8,
}

impl UnoEvent {
    pub fn new(event_type: UnoEventType, player_id: u8) -> Self {
        UnoEvent {
            event_type,
            player_id,
        }
    }
}
