use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize, Debug)]
pub struct FoldAction {}

impl TryFrom<Map<String, Value>> for FoldAction {
    type Error = &'static str;

    fn try_from(map: Map<String, Value>) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

