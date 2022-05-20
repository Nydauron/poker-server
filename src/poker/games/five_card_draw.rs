use crate::poker::games::GameVariation;
use crate::poker::games::evaluator_results::EvaluatorResults;
use crate::poker::CardDeck;
use super::super::player::Player;

use rs_poker::core::Rank;

pub struct FiveCardDraw {
    deck: CardDeck,
    eval: FiveCardDrawEvaluator,
}

impl FiveCardDraw {
    pub fn new() -> FiveCardDraw {
        FiveCardDraw {
            deck: CardDeck::new().unwrap(),
            eval: FiveCardDrawEvaluator {  }
        }
    }

    fn check_player_condition(& self, players:& Vec<Player>) -> bool {
        players.len() >= 2 && players.len() <= 6
    }
}

impl GameVariation for FiveCardDraw {
    fn start_normal(&mut self, players:&mut Vec<Player>, btn_idx: usize) -> Result<(), &str> {
        if self.check_player_condition(players) {
            return Err("Does not meet player requirements");
        }

        players.iter().for_each(|x| {
            let (cards, muck_reshuffled) = self.deck.deal_cards(5);
            x.set_new_hand(cards.unwrap());
        });

        Ok(())
    }

    fn evaluate_all_hands(& self) -> Vec<Vec<(&Player, Rank)>> {
        // calls evaluator
        // TODO: Implement
    }

}

pub struct FiveCardDrawEvaluator {

}

impl EvaluatorResults for FiveCardDrawEvaluator {
    fn get_list_of_rankings(& self) -> Vec<Vec<(&Player, Rank)>> {
        // TODO: Implement with either rs_poker or yr own
    }
}