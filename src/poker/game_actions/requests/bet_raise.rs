use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BetAction {
    pub amount: u64,
}

impl TryFrom<Map<String, Value>> for BetAction {
    type Error = &'static str;

    fn try_from(map: Map<String, Value>) -> Result<Self, Self::Error> {
        if let Some(amount) = map.get("amount") {
            match amount {
                Value::Number(amt) => {
                    if let Some(amt) = amt.as_u64() {
                        return Ok(Self {
                            amount: amt,
                        });
                    }
                    return Err("Amount is a number, but an unsigned 64-bit");
                },
                _ => {
                    return Err("Amount is not a number");
                }
            }
        }

        Err("String keys could not map to BetAction fields")
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json};

    use super::*;

    #[test]
    fn deserialize_from_json_int() {
        let value = json!({"amount": 42069});
        let action: BetAction = serde_json::from_value(value).expect("An error occurred!");
        let expected = BetAction {
            amount: 42069,
        };
        assert_eq!(expected, action);
    }

    #[test]
    #[should_panic]
    fn deserialize_from_json_string() {
        let value = json!({"amount": "42069"});
        let _: BetAction = serde_json::from_value(value).expect("An error occurred!");
    }
}
