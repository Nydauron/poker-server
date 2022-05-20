use crate::Player;

mod player;

pub struct Table {
    players: Vec<Player>,
    game: GameVariation,
    
    big_blind_idx: usize,
    btn_idx: usize,
    pots: Vec<u64>, // might make as a struct that handles side pots
}
