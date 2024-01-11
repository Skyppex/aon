use crate::representation::Value;

pub trait ToAon {
    fn to_aon(&self) -> String;
}

impl ToAon for Option<Value> {
    fn to_aon(&self) -> String {
        match self {
            Some(value) => value.to_aon(),
            None => "null".to_owned(),
        }
    }
}

impl ToAon for bool {
    fn to_aon(&self) -> String {
        if *self {
            "true".to_owned()
        } else {
            "false".to_owned()
        }
    }
}

impl ToAon for f64 {
    fn to_aon(&self) -> String {
        self.to_string()
    }
}

impl ToAon for String {
    fn to_aon(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl ToAon for Vec<(String, Value)> {
    fn to_aon(&self) -> String {
        let mut result = String::new();

        result.push('{');

        for (index, (key, value)) in self.iter().enumerate() {
            if index > 0 {
                result.push(',');
            }

            result.push_str(&key.to_aon());
            result.push(':');
            result.push_str(&value.to_aon());
        }

        result.push('}');

        result
    }
}

impl ToAon for Vec<Value> {
    fn to_aon(&self) -> String {
        let mut result = String::new();

        result.push('[');

        for (index, value) in self.iter().enumerate() {
            if index > 0 {
                result.push(',');
            }

            result.push_str(&value.to_aon());
        }

        result.push(']');

        result
    }
}

impl ToAon for (&String, &Vec<(String, Value)>) {
    fn to_aon(&self) -> String {
        let mut result = String::new();

        result.push('#');
        result.push_str(&self.0);
        result.push('{');

        for (index, (key, value)) in self.1.iter().enumerate() {
            if index > 0 {
                result.push(',');
            }

            result.push_str(&key.to_aon());
            result.push(':');
            result.push_str(&value.to_aon());
        }

        result.push('}');

        result
    }
}

impl ToAon for Value {
    fn to_aon(&self) -> String {
        match self {
            Value::Null => "null".to_owned(),
            Value::Bool(value) => value.to_aon(),
            Value::Number(value) => value.to_aon(),
            Value::String(value) => value.to_aon(),
            Value::Struct(value) => value.to_aon(),
            Value::Union(name, value) => (name, value).to_aon(),
            Value::Array(value) => value.to_aon(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let value = Value::Null;

        assert_eq!(value.to_aon(), "null");
    }

    #[test]
    fn test_bool() {
        let value = Value::Bool(true);

        assert_eq!(value.to_aon(), "true");

        let value = Value::Bool(false);

        assert_eq!(value.to_aon(), "false");
    }

    #[test]
    fn test_number() {
        let value = Value::Number(1.0);

        assert_eq!(value.to_aon(), "1");

        let value = Value::Number(1.5);

        assert_eq!(value.to_aon(), "1.5");
    }

    #[test]
    fn test_string() {
        let value = Value::String("Hello, World!".to_owned());

        assert_eq!(value.to_aon(), r#""Hello, World!""#);
    }

    #[test]
    fn test_struct() {
        let value = Value::Struct(vec![
            ("name".to_owned(), Value::String("John Doe".to_owned())),
            ("age".to_owned(), Value::Number(42.0)),
        ]);

        assert_eq!(value.to_aon(), r#"{"name":"John Doe","age":42}"#);
    }

    #[test]
    fn test_union() {
        let value = Value::Union(
            "person".to_owned(),
            vec![
                ("name".to_owned(), Value::String("John Doe".to_owned())),
                ("age".to_owned(), Value::Number(42.0)),
            ],
        );

        assert_eq!(value.to_aon(), r#"#person{"name":"John Doe","age":42}"#);
    }

    #[test]
    fn test_array() {
        let value = Value::Array(vec![
            Value::String("John Doe".to_owned()),
            Value::Number(42.0),
        ]);

        assert_eq!(value.to_aon(), "[\"John Doe\",42]");
    }
}