use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize, Debug)]
pub struct FlatAction {}

impl TryFrom<Map<String, Value>> for FlatAction {
    type Error = &'static str;

    fn try_from(map: Map<String, Value>) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
