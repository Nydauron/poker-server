use std::collections::{HashSet, HashMap, BTreeMap};
use crate::poker::Player;

pub trait Pot {
    fn get_all_player_stacks_bets(& self) -> &BTreeMap<usize, (u64, u64)>;

    fn get_player_stack_bet(& self, pos: &usize) -> Result<&(u64, u64), std::string::String>;

    fn get_largest_bet_idx(& self) -> usize;

    fn are_all_bets_good(& self, action_idx: usize) -> bool {
        self.get_largest_bet_idx() == action_idx
    }

    fn set_highest_bet(&mut self, action_idx: usize, player_stack: u64, new_bet: u64) -> Result<(), &str>;

    // TODO: Define a method that is called when someone calls (or checks)

    // TODO: Define a distribute function (should we pass a reference to the HashMap of players?)

    fn is_bomb_pot(& self) -> bool;

    fn post_before_deal(&mut self, players: &mut HashMap<usize, Player>, bb_idx: usize) -> Result<(), &str>;

    fn bet_or_shove(&mut self, pos: usize, bet: u64) -> Result<u64, std::string::String>;

    fn check_call(&mut self, pos: usize) -> Result<u64, std::string::String>;

    fn reset_pot(&mut self, players: &HashMap<usize, Player>, sb: u64, bb: u64, ante: u64, is_bomb: bool) -> Result<(), &str>;
    
    fn collect_bets(&mut self);
}

#[derive(PartialEq, Debug)]
pub struct PartialPot {
    pub amount: u64,
    pub elegible_players: HashSet<usize>, // ids of all the players that are eligible to win the pot
}
