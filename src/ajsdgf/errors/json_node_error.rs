use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, JsonNodeError>;

/// An error that can occur when parsing a JSON node.
#[derive(Debug, PartialEq, Clone)]
pub enum JsonNodeError {
    /// The JSON string is empty or has only white space.
    /// If the `Option<String>` is `Some`, then the current node has a parent node which is the string.
    EmptyJson(Option<Box<String>>),

    /// The JSON string could not be parsed.
    /// The `String` is the JSON string that could not be parsed.
    CouldntParseNode(String),

    /// The JSON object has multiple properties with the same key.
    /// The `String` is the key that is duplicated.
    MultiplePropertiesWithSameKey(String),

    /// The JSON object does not have a property with the given key.
    /// The `String` is the key that was not found.
    KeyNotFound(String),
}

impl Display for JsonNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonNodeError::EmptyJson(parent_node) => {
                if let Some(parent_node) = parent_node {
                    return write!(f, "{}", parent_node);
                }
                
                write!(f, "{}", "Json node has no parent".to_string())
            },
            JsonNodeError::CouldntParseNode(node) => write!(f, "{}", node),
            JsonNodeError::MultiplePropertiesWithSameKey(key) => write!(f, "{}", key),
            JsonNodeError::KeyNotFound(key) => write!(f, "{}", key),
        }
    }
}

impl Error for JsonNodeError {}