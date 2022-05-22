use super::Pot;
use crate::poker::Player;

pub struct NoLimitPot<'a> {
    pots: Vec<u64>,
    elegible_players: Vec<Vec<&'a Player>>,

    largest_bet_idx: usize, // should be set when a bet larger than largest_bet is made
    largest_bet: u64,       // should be set when a bet larger than largest_bet is made

    bet_diff: u64,          // used to determine the minimum amount to raise by
}

impl<'a> NoLimitPot<'a> {
    pub fn new() -> NoLimitPot<'a> {
        NoLimitPot {
            pots: Vec::new(),
            elegible_players: Vec::new(),
            largest_bet_idx: 0,
            largest_bet: 0,
            bet_diff: 0,
        }
    }
}

impl Pot for NoLimitPot<'_> {

    fn get_largest_bet_idx(& self) -> usize {
        self.largest_bet_idx
    }

    // This is the function that will work for No Limit
    // TODO: Need to polymorphisize this. Turn struct Pot into a trait that is used in NoLimitPot, LimitPot, and PotLimitPot.
    fn set_highest_bet(&mut self, action_idx: usize, player_stack: u64, new_bet: u64) -> Result<(), &str> {
        if new_bet != player_stack && self.bet_diff + self.largest_bet > new_bet {
            return Err("Bet not high enough");
        }
        self.bet_diff = new_bet - self.largest_bet;
        self.largest_bet = new_bet;
        self.largest_bet_idx = action_idx;
        Ok(())
    }
}