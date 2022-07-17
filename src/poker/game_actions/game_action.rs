use super::requests::{BetAction, DrawAction};
use super::responses::{PersonalFoldResponse, PublicFoldResponse,
                    PersonalCheckCallResponse, PublicCheckCallResponse,
                    PersonalBetRaiseResponse, PublicBetRaiseResponse,
                    PersonalDrawResponse, PublicDrawResponse,
                    StartingHandResponse, GameState};

use uuid::Uuid;

#[derive(Debug)]
pub enum GameAction {
    Pot(Uuid, PotAction),
    Draw(Uuid, DrawAction),
}

#[derive(Debug)]
pub enum PotAction {
    Fold,
    CheckCall,
    BetRaise(BetAction),
}

pub enum GameResponse {
    Broadcast(Broadcast),
    Multicast(Uuid, Multicast), // multicast to everyone BUT Uuid
    SingleResponse(Uuid, SingleResponse),
}

pub enum Broadcast {
    State(GameState),
}

// Used to send responses to other clients
// Should only contain public information (e.g. the cards drawn should not be multicasted, but the number of cards drawn should be)
pub enum Multicast {
    DrawResponse(PublicDrawResponse),
    BetRaiseResponse(PublicBetRaiseResponse),
    CheckCallResponse(PublicCheckCallResponse),
    FoldResponse(PublicFoldResponse),
    
}

// single response types allow for sending back status errors and more private information
pub enum SingleResponse {
    State(GameState), // sends back game state if client requests it (prevents the need to send everyone)

    DrawResponse(PersonalDrawResponse),
    BetRaiseResponse(PersonalBetRaiseResponse),
    CheckCallResponse(PersonalCheckCallResponse),
    FoldResponse(PersonalFoldResponse),

    StartingHandResponse(StartingHandResponse),
}
