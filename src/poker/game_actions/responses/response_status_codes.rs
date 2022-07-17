use serde_repr::Serialize_repr;

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum StatusCode {
    OK = 0,
    NotYourTurn = 1,
    InvalidBet = 2,
}