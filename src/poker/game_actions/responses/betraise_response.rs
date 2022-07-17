use serde::Serialize;
use uuid::Uuid;

use super::StatusCode;

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PublicBetRaiseResponse {
    position: usize,
    bet_amount: u64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PersonalBetRaiseResponse {
    msg_id: Uuid,
    status: StatusCode,
    bet_amount: u64,
}