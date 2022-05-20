use crate::poker::Player;
use rs_poker::core::Rank;

pub trait EvaluatorResults {

    fn get_list_of_rankings(& self) -> Vec<Vec<(&Player, Rank)>>;
}