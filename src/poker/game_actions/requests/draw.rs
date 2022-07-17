use std::str::FromStr;

use playing_cards::core::Card;
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DrawAction {
    pub cards_to_discard: Vec<Card>,
}

impl TryFrom<Map<String, Value>> for DrawAction {
    type Error = &'static str;

    fn try_from(map: Map<String, Value>) -> Result<Self, Self::Error> {
        if let Some(discards) = map.get("cardsToDiscard") {
            match discards {
                Value::Array(arr) => {
                    let mut cards = Vec::new();
                    for val in arr {
                        match val {
                            Value::String(card_str) => {
                                if let Ok(c) = Card::from_str(card_str) {
                                    cards.push(c);
                                } else {
                                    return Err("String entry in cardsToDiscard is not properly formatted")
                                }
                            },
                            _ => {
                                return Err("Entry in cardsToDiscard is not a String");
                            }
                        }
                    }
                    return Ok(Self {
                        cards_to_discard: cards,
                    })
                },
                _ => {
                    return Err("cardsToDiscard is not an array");
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
    fn deserialize_from_json_array_of_strings() {
        let value = json!({"cardsToDiscard": ["2s", "7d", "Ts"]});
        let action: DrawAction = serde_json::from_value(value).expect("An error occurred!");
        let expected = DrawAction {
            cards_to_discard: Card::vec_from_str("2s7dTs").unwrap(),
        };
        assert_eq!(expected, action);
    }

    #[test]
    #[should_panic]
    fn deserialize_from_bad_array() {
        let value = json!({"cardsToDiscard": ["3d", "2d", "lmao"]});
        let _: DrawAction = serde_json::from_value(value).expect("An error occurred!");
    }
}
