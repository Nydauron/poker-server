use std::collections::HashMap;

use super::super::player::Player;
use rs_poker::core::Rank;

pub trait GameVariation {

    fn start_normal(&mut self, players:&mut HashMap<u32, Player>, btn_idx: usize) -> Result<(), &str>;

    fn evaluate_all_hands(& self) -> Vec<Vec<(&Player, Rank)>>;
}
