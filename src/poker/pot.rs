use crate::poker::Player;

pub struct Pot<'a> {
    pots: Vec<u64>,
    elegible_players: Vec<Vec<&'a Player>>,

    largest_bet_idx: usize, // should be set when a bet larger than largest_bet is made
    largest_bet: usize,     // should be set when a bet larger than largest_bet is made


}

impl<'a> Pot<'a> {
    pub fn new() -> Pot<'a> {
        Pot {
            pots: Vec::new(),
            elegible_players: Vec::new(),
            largest_bet_idx: 0,
            largest_bet: 0,
        }
    }

    pub fn are_all_bets_good(& self, action_idx: usize) -> bool {
        self.largest_bet_idx == action_idx
    }
}
