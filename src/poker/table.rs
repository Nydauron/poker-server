use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::poker::Player;
use crate::poker::GameVariation;
use crate::poker::games::DefaultGame;

use crate::poker::pots::{Pot, NoLimitPot};

use crate::poker::{GameActionPayload, GameActionResponse};

pub struct Table {
    players: HashMap<usize, Player>,        // list of all players corresponding to their table position
    game: Box<dyn GameVariation + Send>,    // what game the table is playing
    
    action_idx: usize,                      // action_idx will always point to a Player that is in the hand

    big_blind_idx: usize,
    btn_idx: usize,
    pot: Box<dyn Pot + Send>,               // handles all bets from players, checks for when all bets are good, and distributes pot based upon rankings

    start_next_hand: bool,                  // is the table running (start/stop next hand)
    is_paused: bool,                        // is the current hand paused
    is_next_hand_bomb: bool,
}

impl Table {

    pub fn new() -> Table {
        Table {
            players: HashMap::new(),
            game: Box::new(DefaultGame::new()),
            action_idx: 0,
            big_blind_idx: 0,
            btn_idx: 0,
            pot: Box::new(NoLimitPot::new()),
            start_next_hand: false,
            is_paused: false,
            is_next_hand_bomb: false,
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

    // TODO: Probably should figure out what types are going to be sent thru the channels here
    pub async fn run_loop(table: Arc<Mutex<Table>>, rx: &mut UnboundedReceiver<GameActionPayload>, res_tx: UnboundedSender<GameActionResponse>) {
        while let Some(msg) = rx.recv().await {

        }
    }

    pub async fn game_loop(table: Arc<Mutex<Table>>, res_tx: UnboundedSender<GameActionResponse>) {
        loop {
            {
                let table = table.lock().unwrap();

                if !table.start_next_hand {
                    break
                }
            }
        }
    }
}
