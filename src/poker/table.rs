use std::collections::HashMap;

use crate::poker::Player;
use crate::poker::GameVariation;
use crate::poker::games::DefaultGame;

use crate::poker::Pot;

pub struct Table<'a> {
    players: HashMap<u32, Player>,  // list of all players corresponding to their table position
    game: Box<dyn GameVariation>,   // what game the table is playing
    
    action_idx: usize,              // action_idx will always point to a Player that is in the hand

    big_blind_idx: usize,
    btn_idx: usize,
    pot: Pot<'a>,                   // handles all bets from players, checks for when all bets are good, and distributes pot based upon rankings

    is_running: bool,               // is the table running (start/stop next hand)
    is_paused: bool,                // is the current hand paused
}

impl<'a> Table<'a> {

    pub fn new() -> Table<'a> {
        Table {
            players: HashMap::new(),
            game: Box::new(DefaultGame::new()),
            action_idx: 0,
            big_blind_idx: 0,
            btn_idx: 0,
            pot: Pot::new(),
            is_running: false,
            is_paused: false,
        }
    }
    
    // TODO: How to manage check/call bet/raise and fold?
    //  Thru Player? Table will handle incoming data and actions
    fn check_all_bets_good(& self) -> bool {
        self.pot.are_all_bets_good(self.action_idx)
    }

    fn have_all_players_folded(& self) -> bool {
        self.players.iter().map(|x| {
            x.1.is_away || !x.1.is_in_hand
        }).reduce(|x, y| { x && y }).unwrap()
    }
}
