use serde::Serialize;
use uuid::Uuid;

use super::StatusCode;

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PublicFoldResponse {
    position: usize,
}

#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PersonalFoldResponse {
    req_id: Uuid,
    status: StatusCode,
}
