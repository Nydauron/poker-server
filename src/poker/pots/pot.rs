use std::collections::{HashSet, HashMap, BTreeMap};
use crate::poker::Player;

pub trait Pot {
    fn get_all_player_stacks_bets(& self) -> &BTreeMap<usize, (u64, u64)>;

    fn get_player_stack_bet(& self, pos: &usize) -> Result<&(u64, u64), std::string::String>;

    fn get_largest_bet_idxes(& self) -> Option<(usize, usize)>;

    fn are_all_bets_good(& self, action_idx: usize) -> bool {
        if let Some((lb, _)) = self.get_largest_bet_idxes() {
            lb == action_idx
        } else {
            false
        }
    }

    fn is_bomb_pot(& self) -> bool;

    fn post_before_deal(&mut self, bb_idx: &usize) -> Result<(), &str>;

    fn bet_or_shove(&mut self, pos: &usize, bet: u64) -> Result<u64, std::string::String>;

    fn check_call(&mut self, pos: &usize) -> Result<u64, std::string::String>;

    fn fold(&mut self, pos: &usize) -> Result<(), std::string::String>;

    fn reset_pot(&mut self, players: &HashMap<usize, Player>, sb: u64, bb: u64, ante: u64, is_bomb: bool) -> Result<(), &str>;
    
    fn collect_bets(&mut self);

    fn distribute_pot(&mut self, players: &mut HashMap<usize, Player>, hand_rankings: &HashMap<usize, u64>, btn_idx: &usize) -> HashMap<usize, u64>;

}

#[derive(PartialEq, Debug)]
pub struct PartialPot {
    pub amount: u64,
    pub elegible_players: HashSet<usize>, // ids of all the players that are eligible to win the pot
}
