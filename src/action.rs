use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

/// Node Action Object
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct Action {
    ///  Name of action as described in the first column of "Actions and Parameters" Identifies the function of the action.
    pub action_type: String,
    ///  ID to distinguish between multiple actions, either instant or with the same type on the same node/edge.
    pub action_id: String,
    ///  Additional information on the action.
    pub action_description: Option<String>,
    ///  Regulates if the action is allowed to be executed during movement and/or parallel to other actions.
    pub blocking_type: BlockingType,
    ///  Array of actionParameter objects for the indicated action e.g. deviceId, loadId, external triggers.
    pub action_parameters: Vec<ActionParameter>
}

/// Regulates if the action is allowed to be executed during movement and/or parallel to other actions.
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum BlockingType {
    /// Action can happen in parallel with others, including movement.
    None,
    /// Action can happen simultaneously with others, but not while moving.
    Soft,
    /// No other actions can be performed while this action is running.
    Hard
}

/// ActionParameter Object
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ActionParameter {
    ///  The key of the action parameter. For example. duration, direction, signal.
    pub key: String,
    ///  The value of the action parameter. For example: 103.2, "left", true, [ 1, 2, 3].
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_value"))]
    pub value: ActionParameterValue
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(untagged)
)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ActionParameterValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

#[cfg(feature = "serde")]
fn deserialize_value<'de, D>(deserializer: D) -> Result<ActionParameterValue, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Value;

    impl <'de> serde::de::Visitor<'de> for Value {
        type Value = ActionParameterValue;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str("null, boolean, integer, float, string")
        }

        fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
            Ok(ActionParameterValue::Boolean(value))
        }

        fn visit_i8<E: serde::de::Error>(self, value: i8) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_i16<E: serde::de::Error>(self, value: i16) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_i32<E: serde::de::Error>(self, value: i32) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
            Ok(ActionParameterValue::Integer(value))
        }

        fn visit_u8<E: serde::de::Error>(self, value: u8) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_u16<E: serde::de::Error>(self, value: u16) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_u32<E: serde::de::Error>(self, value: u32) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
            self.visit_i64(value as i64)
        }

        fn visit_f32<E: serde::de::Error>(self, value: f32) -> Result<Self::Value, E> {
            self.visit_f64(value as f64)
        }

        fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
            Ok(ActionParameterValue::Float(value))
        }

        fn visit_char<E: serde::de::Error>(self, value: char) -> Result<Self::Value, E> {
            Ok(ActionParameterValue::String(String::from(value)))
        }

        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
            self.visit_string(value.to_owned())
        }

        fn visit_borrowed_str<E: serde::de::Error>(self, value: &'de str) -> Result<Self::Value, E> {
            self.visit_string(value.to_owned())
        }

        fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
            Ok(ActionParameterValue::String(value))
        }

        fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(ActionParameterValue::Null)
        }
    }

    deserializer.deserialize_any(Value)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use alloc::string::String;
    use super::{ActionParameter, ActionParameterValue};
    use googletest::prelude::*;
    use rstest::rstest;

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_serde_ActionParameter_with_null_value() {

        let parameter = ActionParameter {
            key: String::from("my-null"),
            value: ActionParameterValue::Null,
        };

        let json = r#"{"key":"my-null","value":null}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(from, ok(matches_pattern!(
            ActionParameter {
                key: eq("my-null"),
                value: eq(&ActionParameterValue::Null)
            }
        )));
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_deserialize_ActionParameter_with_bool_value() {

        let parameter = ActionParameter {
            key: String::from("my-bool"),
            value: ActionParameterValue::Boolean(true),
        };

        let json = r#"{"key":"my-bool","value":true}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(from, ok(matches_pattern!(
            ActionParameter {
                key: eq("my-bool"),
                value: eq(&ActionParameterValue::Boolean(true))
            }
        )));
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_deserialize_ActionParameter_with_integer_value() {

        let parameter = ActionParameter {
            key: String::from("my-integer"),
            value: ActionParameterValue::Integer(42),
        };

        let json = r#"{"key":"my-integer","value":42}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(from, ok(matches_pattern!(
            ActionParameter {
                key: eq("my-integer"),
                value: eq(&ActionParameterValue::Integer(42))
            }
        )));
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_deserialize_ActionParameter_with_float_value() {

        let parameter = ActionParameter {
            key: String::from("my-float"),
            value: ActionParameterValue::Float(42.73),
        };

        let json = r#"{"key":"my-float","value":42.73}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(from, ok(matches_pattern!(
            ActionParameter {
                key: eq("my-float"),
                value: eq(&ActionParameterValue::Float(42.73))
            }
        )));
    }

    #[cfg(feature = "serde")]
    #[rstest]
    fn test_deserialize_ActionParameter_with_string_value() {

        let parameter = ActionParameter {
            key: String::from("my-string"),
            value: ActionParameterValue::String(String::from("Hello World")),
        };

        let json = r#"{"key":"my-string","value":"Hello World"}"#;

        let to = serde_json::to_string(&parameter);
        let from = serde_json::from_str::<ActionParameter>(&json);

        assert_that!(to, ok(eq(json)));

        assert_that!(from, ok(matches_pattern!(
            ActionParameter {
                key: eq("my-string"),
                value: eq(&ActionParameterValue::String(String::from("Hello World")))
            }
        )));
    }
}
