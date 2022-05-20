use crate::poker::Player;
use crate::poker::GameVariation;

pub struct Table {
    players: Vec<Player>,
    
    big_blind_idx: usize,
    btn_idx: usize,
    pots: Vec<u64>, // might make as a struct that handles side pots

    game: dyn GameVariation,
}
