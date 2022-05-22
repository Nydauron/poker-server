use crate::poker::Player;

pub trait Pot {
    fn get_largest_bet_idx(& self) -> usize;

    fn are_all_bets_good(& self, action_idx: usize) -> bool {
        self.get_largest_bet_idx() == action_idx
    }

    fn set_highest_bet(&mut self, action_idx: usize, player_stack: u64, new_bet: u64) -> Result<(), &str>;

    // TODO: Define a method that is called when someone calls (or checks)

    // TODO: Define a distribute function (should we pass a reference to the HashMap of players?)
}
