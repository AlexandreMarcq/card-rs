use std::{collections::HashMap, ops::Index};

use rand::seq::SliceRandom;

use crate::server::GameState;

use super::{
    card::{UnoCard, UnoColor},
    event::{UnoEvent, UnoEventType},
    player::UnoPlayer,
    UnoError,
};

#[derive(Debug, PartialEq)]
enum UnoStateType {
    WaitingForPlayer,
    WaitingForDiscard,
    WaitingForDraw,
}

#[derive(Debug)]
pub struct UnoState {
    current_state: UnoStateType,
    current_player: u8,
    number_nocard: u8,
    max_players: usize,
    pub players: HashMap<u8, UnoPlayer>,
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
            current_player: 1,
            number_nocard: 0,
            max_players: 2,
            players: HashMap::new(),
            pile: vec![deck.pop().expect("deck should not be empty")],
            deck,
        }
    }
}

impl GameState for UnoState {
    type Event = UnoEvent;
    type Error = UnoError;

    fn validate(&self, event: &Self::Event) -> Result<(), Self::Error> {
        if event.event_type != UnoEventType::PlayerJoined && event.player_id != self.current_player
        {
            return Err(UnoError::InvalidPlayer);
        }

        match (&self.current_state, &event.event_type) {
            (UnoStateType::WaitingForPlayer, UnoEventType::PlayerJoined) => {
                if self.players.len() == self.max_players {
                    Err(UnoError::TooManyPlayers)
                } else {
                    Ok(())
                }
            }
            (UnoStateType::WaitingForDiscard, UnoEventType::Discard { card, .. }) => {
                let top_card = self.pile.last().unwrap();
                if card.color == top_card.color || card.value == top_card.value {
                    Ok(())
                } else {
                    Err(UnoError::InvalidCard)
                }
            }
            (UnoStateType::WaitingForDiscard, UnoEventType::NoCard) => Ok(()),
            (UnoStateType::WaitingForDraw, UnoEventType::Draw) => Ok(()),
            _ => Err(UnoError::InvalidEvent),
        }
    }

    fn consume(&mut self, event: &Self::Event) {
        match &event.event_type {
            UnoEventType::PlayerJoined => {
                self.players.insert(
                    event.player_id,
                    UnoPlayer::new(self.deck.drain(0..7).collect()),
                );

                if self.players.len() == self.max_players {
                    self.current_state = UnoStateType::WaitingForDiscard;
                }
            }
            UnoEventType::Discard { card, index } => {
                self.players
                    .get_mut(&event.player_id)
                    .expect("player should exist")
                    .hand
                    .remove(*index);

                self.pile.push(card.clone());
            }
            UnoEventType::Draw => {
                self.players
                    .get_mut(&event.player_id)
                    .expect("player should exist")
                    .hand
                    .push(self.deck.pop().expect("deck should not be empty"));

                if self.deck.is_empty() {
                    self.pile.shuffle(&mut rand::rng());
                    self.deck.append(&mut self.pile);
                }

                self.current_state = UnoStateType::WaitingForDiscard;
            }
            UnoEventType::NoCard => {
                self.number_nocard += 1;

                if self.number_nocard == 1 {
                    self.current_state = UnoStateType::WaitingForDraw;
                } else {
                    self.current_state = UnoStateType::WaitingForDiscard;
                    self.current_player = (self.current_player + 1) % self.max_players as u8;
                }
            }
            _ => todo!(),
        }
    }
}
