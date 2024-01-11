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
