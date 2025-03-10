use super::card::UnoCard;

#[derive(Debug)]
pub struct UnoPlayer {
    hand: Vec<UnoCard>,
}

impl UnoPlayer {
    pub fn new(hand: Vec<UnoCard>) -> Self {
        UnoPlayer { hand }
    }
}
