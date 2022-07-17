use playing_cards::core::Card;
use serde::Serialize;
use uuid::Uuid;

use super::StatusCode;


#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PublicDrawResponse {
    position: usize,
    discard_count: usize,
    cards_recieved: usize,
}

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PersonalDrawResponse {
    req_id: Uuid,
    status: StatusCode,
    new_cards: Vec<Card>,
}
