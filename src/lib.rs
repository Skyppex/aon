#![allow(dead_code)]

mod representation;
mod tokens;
mod serializer;

#[cfg(test)]
mod tests {
    use crate::{representation::Value, serializer::ToAon};

    #[test]
    fn test_serialize() {
        let value = Value::Struct(vec![
            ("name".to_owned(), Value::String("John Doe".to_owned())),
            ("age".to_owned(), Value::Number(42.0)),
            ("is_cool".to_owned(), Value::Bool(true)),
            ("friends".to_owned(), Value::Array(vec![
                Value::String("Jane Doe".to_owned()),
                Value::String("Jack Doe".to_owned()),
            ])),
        ]);

        assert_eq!(value.to_aon(), "#person{name:\"John Doe\",age:42,is_cool:true,friends:[\"Jane Doe\",\"Jack Doe\"]}");
    }
}