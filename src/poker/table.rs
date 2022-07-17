use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::poker::Player;
use crate::poker::GameVariation;
use crate::poker::games::DefaultGame;

use crate::poker::pots::{Pot, NoLimitPot};

use crate::poker::{GameActionPayload, GameActionResponse};
use crate::poker::game_actions::{GameAction, PotAction, requests::{BetAction, DrawAction}};

use crate::poker::ActionType;

use serde_json::Value;

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
        let mut game_loop_tx: Option<UnboundedSender<GameAction>> = None;
        while let Some(msg) = rx.recv().await {
            if let Some(tx) = game_loop_tx.clone() {
                match msg.action_type {
                    ActionType::CheckCall => {
                        // talk to game loop, forward message

                        tx.send(GameAction::Pot(msg.id, PotAction::CheckCall));
                        continue;
                    },
                    ActionType::BetRaise => {
                        // talk to game loop, forward message
                        // turn Value into a GameAction then forward
                        let action: Result<BetAction, _> = serde_json::from_value(Value::Object(msg.data));
                        if let Ok(action) = action {
                            tx.send(GameAction::Pot(msg.id, PotAction::BetRaise(action)));
                        } else {
                            todo!();
                        }
                        continue;
                    },
                    ActionType::Fold => {
                        // talk to game loop, forward message

                        tx.send(GameAction::Pot(msg.id, PotAction::Fold));

                        continue;
                    },
                    ActionType::Draw => {
                        // talk to game loop, forward message
                        // turn Value into a GameAction then forward

                        let action: Result<DrawAction, _> = serde_json::from_value(Value::Object(msg.data));
                        if let Ok(action) = action {
                            tx.send(GameAction::Draw(msg.id, action));
                        } else {
                            todo!();
                        }
                        continue;
                    },
                    _ => {}
                }
            }
            match msg.action_type {
                ActionType::StartGame => {
                    // TODO: check authorization jwt by looking up uuid
                    if game_loop_tx.clone().map_or_else(|| true, |tx| tx.is_closed()) {
                        let t = table.clone();
                        let mut table = table.lock().unwrap();
                        table.start_next_hand = true;

                        let (tx, rx) = unbounded_channel::<GameAction>();
                        game_loop_tx = Some(tx);

                        let mes_res_tx = res_tx.clone();
                        tokio::spawn(async move {
                            Table::game_loop(t, rx, mes_res_tx).await;
                        });
                    }

                    // let res = GameActionResponse{

                    // }
                    // res_tx.send(message);
                },
                ActionType::PauseGame => {
                    // check authorization jwt by looking up uuid

                    let mut table = table.lock().unwrap();
                    table.is_paused = true;
                },
                ActionType::ResumeGame => {
                    // check authorization jwt by looking up uuid

                    let mut table = table.lock().unwrap();
                    table.is_paused = false;
                },
                ActionType::StopGame => {
                    // check authorization jwt by looking up uuid

                    let mut table = table.lock().unwrap();
                    table.start_next_hand = false;
                },
                _ => {
                    // This means we got a GameAction and no game loop is running
                    // TODO: send some response idk
                }
            }
        }
    }

    pub async fn game_loop(table: Arc<Mutex<Table>>, main_loop_rx: UnboundedReceiver<GameAction>, res_tx: UnboundedSender<GameActionResponse>) {
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
