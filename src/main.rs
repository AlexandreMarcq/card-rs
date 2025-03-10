use server::GameState;
use uno::{
    event::{UnoEvent, UnoEventType},
    state::UnoState,
};

mod server;
mod uno;

fn main() {
    let mut uno = UnoState::new();
    uno.next(&UnoEvent::new(UnoEventType::PlayerJoined))
        .unwrap();
    uno.next(&UnoEvent::new(UnoEventType::PlayerJoined))
        .unwrap();
    println!("{uno:#?}");
}
