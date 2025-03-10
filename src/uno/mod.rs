pub mod card;
pub mod event;
pub mod player;
pub mod state;

#[derive(Debug)]
pub enum UnoError {
    TooManyPlayers,
}
