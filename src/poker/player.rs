use std::fmt::Error;

use rs_poker::core::Card;

pub struct Player {
    table_position: u32,
    name: String,
    stack: u64,
    hand: Vec<Card>,

    // player config stuff
    is_in_hand: bool,
    play_in_bomb_pots: bool,
}

impl Player {
    pub fn new(position: u32, name: String, starting_stack: u64) -> Player {
        Player {
            table_position: position,
            name: name,
            stack: starting_stack,
            hand: Vec::new(),

            is_in_hand: true,
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
