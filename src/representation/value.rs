pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Struct(Vec<(String, Value)>),
    Union(String, Vec<(String, Value)>),
    Array(Vec<Value>),
}
