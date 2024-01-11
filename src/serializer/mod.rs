use std::cmp::Ordering;

use crate::representation::{Value, value::Number};

use self::formatter::Formatter;

pub mod formatter;

pub trait ToAon {
    fn to_aon(&self, formatter: &Formatter) -> String;
}

impl<T: ToAon> ToAon for Option<T> {
    fn to_aon(&self, formatter: &Formatter) -> String {
        match self {
            Some(value) => format!("#some{{{}}}", value.to_aon(formatter)),
            None => "#none".to_owned(),
        }
    }
}

impl<T: ToAon, U: ToAon> ToAon for Result<T, U> {
    fn to_aon(&self, formatter: &Formatter) -> String {
        match self {
            Ok(value) => format!("#ok{{{}}}", value.to_aon(formatter)),
            Err(error) => format!("#err{{{}}}", error.to_aon(formatter)),
        }
    }
}

impl ToAon for Ordering {
    fn to_aon(&self, _formatter: &Formatter) -> String {
        match self {
            Ordering::Less => "#less".to_owned(),
            Ordering::Equal => "#equal".to_owned(),
            Ordering::Greater => "#greater".to_owned(),
        }
    }
}

impl ToAon for bool {
    fn to_aon(&self, _formatter: &Formatter) -> String {
        if *self {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    }
}

impl ToAon for u64 {
    fn to_aon(&self, _formatter: &Formatter) -> String {
        self.to_string()
    }
}

impl ToAon for u128 {
    fn to_aon(&self, _formatter: &Formatter) -> String {
        self.to_string()
    }
}

impl ToAon for i64 {
    fn to_aon(&self, _formatter: &Formatter) -> String {
        self.to_string()
    }
}

impl ToAon for i128 {
    fn to_aon(&self, _formatter: &Formatter) -> String {
        self.to_string()
    }
}

impl ToAon for f64 {
    fn to_aon(&self, _formatter: &Formatter) -> String {
        self.to_string()
    }
}

impl ToAon for String {
    fn to_aon(&self, _formatter: &Formatter) -> String {
        format!("\"{}\"", self)
    }
}

impl ToAon for Vec<(String, Value)> {
    fn to_aon(&self, formatter: &Formatter) -> String {
        let mut result = String::new();

        result.push('{');

        for (index, (key, value)) in self.iter().enumerate() {
            if index > 0 {
                result.push(',');
            }

            result.push_str(&key.to_aon(formatter));
            result.push(':');
            result.push_str(&value.to_aon(formatter));
        }

        result.push('}');

        result
    }
}

impl ToAon for Vec<Value> {
    fn to_aon(&self, formatter: &Formatter) -> String {
        let mut result = String::new();

        result.push('[');

        for (index, value) in self.iter().enumerate() {
            if index > 0 {
                result.push(',');
            }

            result.push_str(&value.to_aon(formatter));
        }

        result.push(']');

        result
    }
}

impl ToAon for (&String, &Vec<(String, Value)>) {
    fn to_aon(&self, formatter: &Formatter) -> String {
        let mut result = String::new();

        result.push('#');
        result.push_str(&self.0);
        result.push('{');

        for (index, (key, value)) in self.1.iter().enumerate() {
            if index > 0 {
                result.push(',');
            }

            result.push_str(&key.to_aon(formatter));
            result.push(':');
            result.push_str(&value.to_aon(formatter));
        }

        result.push('}');

        result
    }
}

impl ToAon for Number {
    fn to_aon(&self, formatter: &Formatter) -> String {
        match self {
            Number::PosInt(value) => value.to_aon(formatter),
            Number::BigPosInt(value) => value.to_aon(formatter),
            Number::NegInt(value) => value.to_aon(formatter),
            Number::BigNegInt(value) => value.to_aon(formatter),
            Number::Float(value) => value.to_aon(formatter),
        }
    }

}

impl ToAon for Value {
    fn to_aon(&self, formatter: &Formatter) -> String {
        match self {
            Value::Null => "null".to_owned(),
            Value::Bool(value) => value.to_aon(formatter),
            Value::Number(value) => value.to_aon(formatter),
            Value::String(value) => value.to_aon(formatter),
            Value::Struct(value) => value.to_aon(formatter),
            Value::Union(name, value) => (name, value).to_aon(formatter),
            Value::Array(value) => value.to_aon(formatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::representation::value::Number;
    use super::*;

    #[test]
    fn test_null() {
        let value = Value::Null;

        assert_eq!(value.to_aon(&Formatter::default()), "null");
    }

    #[test]
    fn test_bool() {
        let value = Value::Bool(true);

        assert_eq!(value.to_aon(&Formatter::default()), "true");

        let value = Value::Bool(false);

        assert_eq!(value.to_aon(&Formatter::default()), "false");
    }

    #[test]
    fn test_number() {
        let value = Value::Number(Number::PosInt(1));
        assert_eq!(value.to_aon(&Formatter::default()), "1");

        let value = Value::Number(Number::Float(1.5));
        assert_eq!(value.to_aon(&Formatter::default()), "1.5");

        let value = Value::Number(Number::NegInt(-1));
        assert_eq!(value.to_aon(&Formatter::default()), "-1");
    }

    #[test]
    fn test_string() {
        let value = Value::String("Hello, World!".to_owned());

        assert_eq!(value.to_aon(&Formatter::default()), r#""Hello, World!""#);
    }

    #[test]
    fn test_struct() {
        let value = Value::Struct(vec![
            ("name".to_owned(), Value::String("John Doe".to_owned())),
            ("age".to_owned(), Value::Number(Number::PosInt(42))),
        ]);

        assert_eq!(value.to_aon(&Formatter::default()), r#"{"name":"John Doe","age":42}"#);
    }

    #[test]
    fn test_union() {
        let value = Value::Union(
            "person".to_owned(),
            vec![
                ("name".to_owned(), Value::String("John Doe".to_owned())),
                ("age".to_owned(), Value::Number(Number::PosInt(42))),
            ],
        );

        assert_eq!(value.to_aon(&Formatter::default()), r#"#person{"name":"John Doe","age":42}"#);
    }

    #[test]
    fn test_array() {
        let value = Value::Array(vec![
            Value::String("John Doe".to_owned()),
            Value::Number(Number::PosInt(42)),
        ]);

        assert_eq!(value.to_aon(&Formatter::default()), "[\"John Doe\",42]");
    }
}