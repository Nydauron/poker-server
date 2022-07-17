use std::collections::HashMap;

use playing_cards::core::{Card, CardDeck};
use playing_cards::poker::{HighEvaluator, Rank};

use crate::poker::games::GameVariation;
use crate::poker::Player;


pub struct FiveCardDraw {
    deck: CardDeck,
    eval: HighEvaluator,

    board: Vec<Card>,
}

impl FiveCardDraw {
    const MIN_PLAYER_COUNT: usize = 2;
    const MAX_PLAYER_COUNT: usize = 6;

    pub fn new() -> FiveCardDraw {
        FiveCardDraw {
            deck: CardDeck::new().unwrap(),
            eval: HighEvaluator{},

            board: Vec::new(),
        }
    }

    fn check_player_condition(& self, players:& HashMap<u32, Player>) -> bool {
        players.len() >= FiveCardDraw::MIN_PLAYER_COUNT && players.len() <= FiveCardDraw::MAX_PLAYER_COUNT
    }
}

impl GameVariation for FiveCardDraw {
    fn start_normal(&mut self, players:&mut HashMap<u32, Player>, btn_idx: usize) -> Result<(), &str> {
        if self.check_player_condition(players) {
            return Err("Does not meet player requirements");
        }

        players.iter_mut().for_each(|x| {
            let (cards, _) = self.deck.deal_cards(5);
            x.1.set_new_hand(cards.unwrap());
        });

        Ok(())
    }

    fn evaluate_all_hands(& self) -> Vec<Vec<(&Player, Rank)>> {
        // calls evaluator
        todo!()
    }

}
