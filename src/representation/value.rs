use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Struct(Vec<(String, Value)>),
    Union(String, Vec<(String, Value)>),
    Array(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => write!(f, "\"{}\"", string),
            Value::Struct(kvp) => write!(f, "{{{}}}", format_object(kvp)),
            Value::Union(union, kvp) => write!(f, "#{}:{}", union, format_object(kvp)),
            Value::Array(_) => todo!(),
        }
    }
}

fn format_object(key_value_pairs: &Vec<(String, Value)>) -> String {
    key_value_pairs.iter().map(|(name, value)| format!("\"{}\":{}", name, value))
        .collect::<Vec<String>>()
        .join(", ")
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    // Always positive
    PosInt(u64),

    // Always positive
    BigPosInt(u128),

    // Always negative
    NegInt(i64),

    // Always negative
    BigNegInt(i128),

    // Always finite
    Float(f64),
}

impl Number {
    pub fn new(string: &str) -> Option<Self> {
        match string.parse::<u64>() {
            Ok(value) => Some(Number::PosInt(value)),
            Err(_) => match string.parse::<u128>() {
                Ok(value) => Some(Number::BigPosInt(value)),
                Err(_) => match string.parse::<i64>() {
                    Ok(value) => Some(Number::NegInt(value)),
                    Err(_) => match string.parse::<i128>() {
                        Ok(value) => Some(Number::BigNegInt(value)),
                        Err(_) => match string.parse::<f64>() {
                            Ok(value) => Some(Number::Float(value)),
                            Err(_) => None,
                        },
                    },
                },
            },
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::PosInt(v) => write!(f, "{}", v),
            Number::BigPosInt(v) => write!(f, "{}", v),
            Number::NegInt(v) => write!(f, "{}", v),
            Number::BigNegInt(v) => write!(f, "{}", v),
            Number::Float(v) => write!(f, "{}", v),
        }
    }
}