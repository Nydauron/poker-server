use std::collections::HashMap;

use crate::poker::games::GameVariation;
use crate::poker::games::evaluator_results::EvaluatorResults;
use crate::poker::CardDeck;
use crate::poker::Player;

use rs_poker::core::Rank;

pub struct FiveCardDraw {
    deck: CardDeck,
    eval: FiveCardDrawEvaluator,
}

impl FiveCardDraw {
    const MIN_PLAYER_COUNT: usize = 2;
    const MAX_PLAYER_COUNT: usize = 6;

    pub fn new() -> FiveCardDraw {
        FiveCardDraw {
            deck: CardDeck::new().unwrap(),
            eval: FiveCardDrawEvaluator {  }
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
            let (cards, muck_reshuffled) = self.deck.deal_cards(5);
            x.1.set_new_hand(cards.unwrap());
        });

        Ok(())
    }

    fn evaluate_all_hands(& self) -> Vec<Vec<(&Player, Rank)>> {
        // calls evaluator
        // TODO: Implement
        Vec::new()
    }

}

pub struct FiveCardDrawEvaluator {

}

impl EvaluatorResults for FiveCardDrawEvaluator {
    fn get_list_of_rankings(& self) -> Vec<Vec<(&Player, Rank)>> {
        // TODO: Implement with either rs_poker or yr own
        Vec::new()
    }
}