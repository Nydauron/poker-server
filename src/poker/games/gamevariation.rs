use std::collections::HashMap;

use playing_cards::core::Card;

use super::super::player::Player;
use playing_cards::poker::Rank;

pub trait GameVariation {

    fn start_normal(&mut self, players:&mut HashMap<u32, Player>, btn_idx: usize) -> Result<(), &str>;

    fn evaluate_all_hands(& self) -> Vec<Vec<(&Player, Rank)>>;
}
