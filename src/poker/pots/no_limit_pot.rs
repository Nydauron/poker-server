use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use super::{Pot, PartialPot};
use crate::poker::{Player};

pub struct NoLimitPot {
    pots: Vec<PartialPot>,

    player_stacks_bets: BTreeMap<usize, (u64, u64)>, // (stack, current bet)
    bet_sizes: BTreeSet<u64>,

    largest_bet_idx: usize,       // is set on any bet/all-in
    largest_legal_bet_idx: usize, // should only be set when a LEGAL bet/raise is made
    largest_bet: u64,             // should be set when a bet larger than largest_bet is made

    bet_diff: u64,          // used to determine the minimum amount to raise by

    sb_amt: u64,
    bb_amt: u64,
    ante_amt: u64,          // if this is a bomb pot, the ante gets used as the bomb amount. Blinds are then not posted
    is_bomb_pot: bool,
}

impl NoLimitPot {
    pub fn new() -> NoLimitPot {
        NoLimitPot {
            pots: Vec::new(),
            player_stacks_bets: BTreeMap::new(),
            bet_sizes: BTreeSet::new(),
            largest_bet_idx: 0,
            largest_legal_bet_idx: 0,
            largest_bet: 0,
            bet_diff: 0,

            // these settings get reset every hand
            sb_amt: 0,
            bb_amt: 0,
            ante_amt: 0,
            is_bomb_pot: false,
        }
    }

    // Simplay sets a player's bet (no upper limit)
    // Player bets at most bet amount. If bet amount is too much, player shoves
    fn set_bet_no_max(&mut self, pos: &usize, bet: u64) -> Result<(), std::string::String> {
        if let Some(v) = self.player_stacks_bets.get_mut(pos) {
            let bet_size = std::cmp::min(v.0, bet);
            // println!("v.0 = {}, bet = {}, minimum = {}", v.0, bet, bet_size);
            v.1 = bet_size;
            self.bet_sizes.insert(bet_size);
            Ok(())
        } else {
            Err(format!("Could not find player stack in position {}", pos))
        }
    }

    fn can_pos_raise(& self, pos: usize) -> bool {
        self.largest_legal_bet_idx < self.largest_bet_idx && (pos > self.largest_bet_idx || pos < self.largest_legal_bet_idx) ||
            self.largest_legal_bet_idx > self.largest_bet_idx && (pos > self.largest_bet_idx && pos < self.largest_legal_bet_idx) ||
            self.largest_legal_bet_idx == self.largest_bet_idx && self.largest_bet_idx != pos
    }
}

impl Pot for NoLimitPot {

    fn get_all_player_stacks_bets(& self) -> &BTreeMap<usize, (u64, u64)> {
        &self.player_stacks_bets
    }

    fn get_player_stack_bet(& self, pos: &usize) -> Result<&(u64, u64), std::string::String> {
        self.player_stacks_bets.get(pos)
            .ok_or(format!("Player position {} did not play in the current hand", pos))
    }

    // Resets the pot to all inital values (similar to new(), but doesnt create a new Pot instance)
    fn reset_pot(&mut self, players: &HashMap<usize, Player>, sb: u64, bb: u64, ante: u64, is_bomb: bool) -> Result<(), &str> {
        if is_bomb && ante == 0 {
            return Err("Ante must be non-zero for bomb pots");
        }

        if sb == 0 || bb == 0 {
            return Err("Blinds must be a positive non-zero amount");
        }

        if sb > bb {
            return Err("Small blind cannot be larger than the big blind");
        }

        self.sb_amt = sb;
        self.bb_amt = bb;
        self.ante_amt = ante;
        self.is_bomb_pot = is_bomb;

        self.bet_diff = bb;
        self.largest_bet = bb;
        self.largest_bet_idx = 0; // idk this needs to be set appropriately
        self.largest_legal_bet_idx = 0;

        for (_, p) in players {
            let entry = (p.stack, 0);
            self.player_stacks_bets.insert(p.table_position, entry);
        }

        Ok(())
    }

    fn collect_bets(&mut self) {
        let mut iter = self.bet_sizes.iter();
        let mut prev_bet_collected = 0;

        while let Some(&b) = iter.next() {
            let bet = b - prev_bet_collected;
            let mut side_pot = self.pots.pop()
                .unwrap_or_else(|| {
                    let p: Vec<usize> = self.player_stacks_bets.keys().cloned().collect();
                    PartialPot {
                        amount: 0,
                        elegible_players: HashSet::from_iter(p),
                    }
                }
            );

            let mut elegible_players = HashSet::<usize>::new();
            let mut all_in_players = HashSet::<usize>::new();
            for (pos, stack_bet) in &mut self.player_stacks_bets {
                // could check if bettor is an elegible player
                if side_pot.elegible_players.contains(pos) && stack_bet.1 >= bet {
                    stack_bet.0 -= bet;
                    stack_bet.1 -= bet;
                    side_pot.amount  += bet;
                    elegible_players.insert(*pos);
                    if stack_bet.0 == 0 {
                        all_in_players.insert(*pos);
                    }
                }
            }

            side_pot.elegible_players = elegible_players.clone();

            self.pots.push(side_pot);
            prev_bet_collected = b;

            if !all_in_players.is_empty() {
                self.pots.push(PartialPot {
                    amount: 0,
                    elegible_players: &elegible_players - &all_in_players,
                });
            }
        }

        self.bet_sizes.clear();

        self.bet_diff = self.bb_amt;
        self.largest_bet = self.bb_amt;
        self.largest_bet_idx = 0; // idk this needs to be set appropriately
        self.largest_legal_bet_idx = 0;
    }

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

    fn is_bomb_pot(& self) -> bool {
        self.is_bomb_pot
    }

    // First method that will be called before the hand begins
    fn post_before_deal(&mut self, players: &mut HashMap<usize, Player>, bb_idx: usize) -> Result<(), &str> {
        // pay ante first
        for (id, player) in players {
            if !player.is_away {
                self.set_bet_no_max(id, self.ante_amt);
            }
        }

        if !self.is_bomb_pot {
            // dont pay blinds as the ante is the bomb amount
            todo!();
        }
        Ok(())
    }

    // Function to indicate player in position pos is betting/raising/shoving an amount of bet
    fn bet_or_shove(&mut self, pos: usize, bet: u64) -> Result<u64, std::string::String> {
        if !self.can_pos_raise(pos) {
            // This case is very much the edge case
            Err(format!("You already called the latest legal bet, so you are no longer allowed to raise"))
        } else if let Some(v) = self.player_stacks_bets.get_mut(&pos) {
            let min_bet = self.largest_bet + self.bet_diff;
            if v.0 <= bet || bet >= min_bet {
                let bet_size = std::cmp::min(v.0, bet);
                v.1 = bet_size;
                self.bet_sizes.insert(bet_size);

                if bet >= min_bet {
                    self.bet_diff = bet_size - self.largest_bet;
                    self.largest_legal_bet_idx = pos;
                }
                // Question: How much can you raise by once there is a shove for only 100 more? (e.g. 1500 effective, Bet 200, raise to 900, jam for 1000. Can a person 4-bet to 1700?) 

                self.largest_bet = bet_size;
                self.largest_bet_idx = pos;

                Ok(bet_size)
            } else {
                if v.0 <= min_bet {
                    Err(format!("Bet of {} is too small (must shove)", bet))
                } else {
                    Err(format!("Bet of {} is too small (must be at least {})", bet, min_bet))
                }
            }
        } else {
            Err(format!("Could not find player stack in position {}", pos))
        }
    }

    // Function to indicate player in position pos is calling the largest bet
    fn check_call(&mut self, pos: usize) -> Result<u64, std::string::String> {
        if let Some(v) = self.player_stacks_bets.get_mut(&pos) {
            let bet_size = std::cmp::min(v.0, self.largest_bet);
            v.1 = bet_size;
            self.bet_sizes.insert(bet_size); // still need to attempt an insert as we could be shoving

            Ok(bet_size)
        } else {
            Err(format!("Could not find player stack in position {}", pos))
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
    use crate::poker::Player;

    #[test]
    fn collect_basic_bets() {
        let mut players = HashMap::<usize, Player>::new();
        // let mut action_idx: usize = 0;

        let mut pot = NoLimitPot::new();

        let starting_stacks = [100, 50, 75, 60];

        for id in 0..4 {
            players.insert(id, Player::new(id, format!("Player {}", id), starting_stacks[id]));
        }

        assert_eq!(pot.reset_pot(&players, 1, 2, 0, false), Ok(()));

        let bet = 20;

        for (id, _) in &players {
            assert_eq!(pot.set_bet_no_max(id, bet), Ok(()));
        }

        pot.collect_bets();

        let expected_pot = vec![
            PartialPot {
                amount: 80,
                elegible_players: HashSet::from([0,1,2,3]),
            },
        ];

        let expected_player_stack_bets = BTreeMap::from([
            (0, (80, 0)),
            (1, (30, 0)),
            (2, (55, 0)),
            (3, (40, 0)),
        ]);

        assert_eq!(expected_pot, pot.pots);
        assert_eq!(BTreeSet::new(), pot.bet_sizes);
        assert_eq!(expected_player_stack_bets, pot.player_stacks_bets);
    }

    #[test]
    fn collect_all_in_bet() {
        let mut players = HashMap::<usize, Player>::new();
        // let mut action_idx: usize = 0;

        let mut pot = NoLimitPot::new();

        let starting_stacks = [100, 50, 75, 60];

        for id in 0..4 {
            players.insert(id, Player::new(id, format!("Player {}", id), starting_stacks[id]));
        }

        assert_eq!(pot.reset_pot(&players, 1, 2, 0, false), Ok(()));

        let bet = 50;

        for (id, _) in &players {
            assert_eq!(pot.set_bet_no_max(id, bet), Ok(()));
        }

        pot.collect_bets();

        let expected_pot = vec![
            PartialPot {
                amount: 200,
                elegible_players: HashSet::from([0,1,2,3]),
            },
            PartialPot {
                amount: 0,
                elegible_players: HashSet::from([0,2,3]),
            }
        ];

        let expected_player_stack_bets = BTreeMap::from([
            (0, (50, 0)),
            (1, (0, 0)),
            (2, (25, 0)),
            (3, (10, 0)),
        ]);

        assert_eq!(expected_pot, pot.pots);
        assert_eq!(BTreeSet::new(), pot.bet_sizes);
        assert_eq!(expected_player_stack_bets, pot.player_stacks_bets);
    }

    #[test]
    fn collect_advanced_betting() {
        let mut players = HashMap::<usize, Player>::new();
        // let mut action_idx: usize = 0;

        let mut pot = NoLimitPot::new();

        let starting_stacks = [100, 20, 75, 5];

        for id in 0..4 {
            players.insert(id, Player::new(id, format!("Player {}", id), starting_stacks[id]));
        }

        assert_eq!(pot.reset_pot(&players, 1, 2, 0, false), Ok(()));

        let bet = 25;

        for (id, _) in &players {
            assert_eq!(pot.set_bet_no_max(id, bet), Ok(()));
        }

        pot.collect_bets();

        let expected_pot = vec![
            PartialPot {
                amount: 20,
                elegible_players: HashSet::from([0,1,2,3]),
            },
            PartialPot {
                amount: 45,
                elegible_players: HashSet::from([0,1,2]),
            },
            PartialPot {
                amount: 10,
                elegible_players: HashSet::from([0,2]),
            }
        ];

        let expected_player_stack_bets = BTreeMap::from([
            (0, (75, 0)),
            (1, (0, 0)),
            (2, (50, 0)),
            (3, (0, 0)),
        ]);

        assert_eq!(expected_pot, pot.pots);
        assert_eq!(BTreeSet::new(), pot.bet_sizes);
        assert_eq!(expected_player_stack_bets, pot.player_stacks_bets);
    }

    #[test]
    fn collect_nonzero_pot() {
        let mut players = HashMap::<usize, Player>::new();
        // let mut action_idx: usize = 0;

        let mut pot = NoLimitPot::new();

        let starting_stacks = [100, 20, 75, 5];

        for id in 0..4 {
            players.insert(id, Player::new(id, format!("Player {}", id), starting_stacks[id]));
        }

        assert_eq!(pot.reset_pot(&players, 1, 2, 0, false), Ok(()));

        // artificially making the pot big for testing
        pot.pots.push(PartialPot {
            amount: 20,
            elegible_players: HashSet::from([0, 1, 2, 3]),
        });

        let bet = 50;

        for (id, _) in &players {
            assert_eq!(pot.set_bet_no_max(id, bet), Ok(()));
        }

        pot.collect_bets();

        let expected_pot = vec![
            PartialPot {
                amount: 40,
                elegible_players: HashSet::from([0,1,2,3]),
            },
            PartialPot {
                amount: 45,
                elegible_players: HashSet::from([0,1,2]),
            },
            PartialPot {
                amount: 60,
                elegible_players: HashSet::from([0,2]),
            }
        ];

        let expected_player_stack_bets = BTreeMap::from([
            (0, (50, 0)),
            (1, (0, 0)),
            (2, (25, 0)),
            (3, (0, 0)),
        ]);

        assert_eq!(expected_pot, pot.pots);
        assert_eq!(BTreeSet::new(), pot.bet_sizes);
        assert_eq!(expected_player_stack_bets, pot.player_stacks_bets);
    }

    #[test]
    fn collect_nonzero_pot_two_callers() {
        let mut players = HashMap::<usize, Player>::new();
        // let mut action_idx: usize = 0;

        let mut pot = NoLimitPot::new();

        let starting_stacks = [100, 20, 75, 5];

        for id in 0..4 {
            players.insert(id, Player::new(id, format!("Player {}", id), starting_stacks[id]));
        }

        assert_eq!(pot.reset_pot(&players, 1, 2, 0, false), Ok(()));

        // artificially making the pot big for testing
        pot.pots.push(PartialPot {
            amount: 20,
            elegible_players: HashSet::from([0, 1, 2, 3]),
        });

        let bet = 25;

        assert_eq!(pot.set_bet_no_max(&0, bet), Ok(()));
        assert_eq!(pot.set_bet_no_max(&2, bet), Ok(()));

        pot.collect_bets();

        let expected_pot = vec![
            PartialPot {
                amount: 70,
                elegible_players: HashSet::from([0,2]),
            },
        ];

        let expected_player_stack_bets = BTreeMap::from([
           (0, (75, 0)),
           (1, (20, 0)),
           (2, (50, 0)),
           (3, (5, 0)),
        ]);

        assert_eq!(expected_pot, pot.pots);
        assert_eq!(BTreeSet::new(), pot.bet_sizes);
        assert_eq!(expected_player_stack_bets, pot.player_stacks_bets);
    }

    fn check_normal(pot: &NoLimitPot) {
        for i in 0..pot.largest_bet_idx+1 {
            assert!(!pot.can_pos_raise(i), "Position {} was incorrect", i);
        }

        for i in pot.largest_bet_idx+1..pot.largest_legal_bet_idx {
            assert!(pot.can_pos_raise(i), "Position {} was incorrect", i);
        }

        for i in pot.largest_legal_bet_idx..10 {
            assert!(!pot.can_pos_raise(i), "Position {} was incorrect", i);
        }
    }

    fn check_flipped(pot: &NoLimitPot) {
        for i in 0..pot.largest_legal_bet_idx {
            assert!(pot.can_pos_raise(i), "Position {} was incorrect", i);
        }

        for i in pot.largest_legal_bet_idx..pot.largest_bet_idx+1 {
            assert!(!pot.can_pos_raise(i), "Position {} was incorrect", i);
        }

        for i in pot.largest_bet_idx+1..10 {
            assert!(pot.can_pos_raise(i), "Position {} was incorrect", i);
        }
    }

    #[test]
    fn can_raise_action() {
        let mut pot = NoLimitPot::new();

        pot.largest_bet_idx = 3;
        pot.largest_legal_bet_idx = 9;

        check_normal(&pot);
    }

    #[test]
    fn can_raise_action_flipped() {
        let mut pot = NoLimitPot::new();

        pot.largest_bet_idx = 8;
        pot.largest_legal_bet_idx = 2;

        check_flipped(&pot);
    }

    #[test]
    fn can_raise_action_close() {
        let mut pot = NoLimitPot::new();

        pot.largest_bet_idx = 5;
        pot.largest_legal_bet_idx = 6;

        check_normal(&pot);
    }

    #[test]
    fn can_raise_action_flipped_close() {
        let mut pot = NoLimitPot::new();

        pot.largest_bet_idx = 3;
        pot.largest_legal_bet_idx = 2;

        check_flipped(&pot);
    }

    #[test]
    fn can_raise_action_same_idx() {
        let mut pot = NoLimitPot::new();

        let mut rng = rand::thread_rng();
        let rand_idx: usize = rng.gen_range(0..=9);
        pot.largest_bet_idx = rand_idx;
        pot.largest_legal_bet_idx = rand_idx;

        check_flipped(&pot);
    }
}
