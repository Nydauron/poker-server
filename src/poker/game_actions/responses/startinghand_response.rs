use playing_cards::core::Card;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct StartingHandResponse {
    hand: Vec<Card>,
}