use serde::Serialize;


#[derive(Serialize, Debug)]
#[serde(rename_all(serialize = "camelCase"))]
// TODO: Needs to accompany all info
pub struct GameState {
    
}