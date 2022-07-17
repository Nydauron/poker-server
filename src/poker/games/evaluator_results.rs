use crate::poker::Player;
use playing_cards::poker::Rank;

pub trait EvaluatorResults {

    fn get_list_of_rankings(& self) -> Vec<Vec<(&Player, Rank)>>;
}