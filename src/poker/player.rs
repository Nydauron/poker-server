use std::fmt::Error;

use playing_cards::core::Card;

#[derive(Debug)]
pub struct Player {
    pub table_position: usize,
    pub name: String,
    pub stack: u64,
    pub bet: u64,
    pub is_in_hand: bool,
    hand: Vec<Card>,

    // player config stuff
    pub is_away: bool,
    pub play_in_bomb_pots: bool,
}

impl Player {
    pub fn new(position: usize, name: String, starting_stack: u64) -> Player {
        Player {
            table_position: position,
            name: name,
            stack: starting_stack,
            bet: 0,
            is_in_hand: true,
            hand: Vec::new(),

            is_away: false,
            play_in_bomb_pots: true,
        }
    }

    pub fn set_new_hand(&mut self, hand: Vec<Card>) {
        self.hand = hand;
    }

    pub fn draw_cards(&self, draw: Vec<(Card, Card)>) -> Result<(), Error> { // All-or-none approach, (Card to discard, card to replace with)
        // find all
        Ok(())
    }
}
