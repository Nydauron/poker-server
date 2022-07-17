use serde::Serialize;
use uuid::Uuid;

use super::StatusCode;

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PublicCheckCallResponse {
    position: usize,
}

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PersonalCheckCallResponse {
    req_id: Uuid,
    status: StatusCode,
}
