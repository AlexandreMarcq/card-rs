use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::server::GameState;

use super::{
    card::{UnoCard, UnoColor},
    event::{UnoEvent, UnoEventType},
    player::UnoPlayer,
    UnoError,
};

#[derive(Debug)]
enum UnoStateType {
    WaitingForPlayer,
    WaitingForDraw,
}

#[derive(Debug)]
pub struct UnoState {
    current_state: UnoStateType,
    max_players: usize,
    players: HashMap<u8, UnoPlayer>,
    pile: Vec<UnoCard>,
    deck: Vec<UnoCard>,
}

impl UnoState {
    pub fn new() -> Self {
        UnoState::default()
    }
}

impl Default for UnoState {
    fn default() -> Self {
        let mut deck: Vec<UnoCard> = vec![
            UnoColor::Blue,
            UnoColor::Red,
            UnoColor::Green,
            UnoColor::Yellow,
        ]
        .into_iter()
        .flat_map(|c| (0..=9).into_iter().map(move |n| UnoCard::new(n, c.clone())))
        .collect();
        deck.shuffle(&mut rand::rng());

        UnoState {
            current_state: UnoStateType::WaitingForPlayer,
            max_players: 2,
            players: HashMap::new(),
            pile: Vec::new(),
            deck,
        }
    }
}

impl GameState for UnoState {
    type Event = UnoEvent;
    type Error = UnoError;

    fn validate(&self, event: &Self::Event) -> Result<(), Self::Error> {
        match (&self.current_state, &event.event_type) {
            (UnoStateType::WaitingForPlayer, UnoEventType::PlayerJoined) => {
                if self.players.len() == self.max_players {
                    Err(UnoError::TooManyPlayers)
                } else {
                    Ok(())
                }
            }
            (UnoStateType::WaitingForDraw, UnoEventType::Draw) => Ok(()),
            _ => todo!(),
        }
    }

    fn consume(&mut self, event: &Self::Event) {
        match &event.event_type {
            UnoEventType::PlayerJoined => {
                let id = (self.players.len() + 1) as u8;
                self.players
                    .insert(id, UnoPlayer::new(self.deck.drain(0..7).collect()));

                if self.players.len() == self.max_players {
                    self.current_state = UnoStateType::WaitingForDraw;
                }
            }
            UnoEventType::Draw => self.pile.push(self.deck.pop().unwrap()),
            _ => todo!(),
        }
    }
}
