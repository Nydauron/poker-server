use crate::poker::Player;

pub struct Pot<'a> {
    pots: Vec<u64>,
    elegible_players: Vec<Vec<&'a Player>>,

    largest_bet_idx: usize, // should be set when a bet larger than largest_bet is made
    largest_bet: u64,       // should be set when a bet larger than largest_bet is made

    bet_diff: u64,          // used to determine the minimum amount to raise by
}

impl<'a> Pot<'a> {
    pub fn new() -> Pot<'a> {
        Pot {
            pots: Vec::new(),
            elegible_players: Vec::new(),
            largest_bet_idx: 0,
            largest_bet: 0,
            bet_diff: 0,
        }
    }

    pub fn are_all_bets_good(& self, action_idx: usize) -> bool {
        self.largest_bet_idx == action_idx
    }

    // This is the function that will work for No Limit
    // TODO: Need to polymorphisize this. Turn struct Pot into a trait that is used in NoLimitPot, LimitPot, and PotLimitPot.
    pub fn set_highest_bet(&mut self, action_idx: usize, new_bet: u64) -> Result<(), &str> {
        if self.bet_diff + self.largest_bet > new_bet {
            return Err("Bet not high enough");
        }
        self.bet_diff = new_bet - self.largest_bet;
        self.largest_bet = new_bet;
        self.largest_bet_idx = action_idx;
        Ok(())
    }
}
