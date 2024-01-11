use crate::tokens;

use super::Value;

pub fn parse_node(aon: &str) -> Result<Value, String> {
    let trim = aon.trim();

    if trim.is_empty() {
        return Err("Empty string".to_owned());
    }

    if let Some(node) = parse_string(trim) {
        return Ok(node);
    }

    if let Some(node) = parse_number(trim) {
        return Ok(node);
    }

    if let Some(node) = parse_bool(trim) {
        return Ok(node);
    }

    if let Some(node) = parse_null(trim) {
        return Ok(node);
    }

    if let Some(node) = parse_union(trim) {
        return Ok(node);
    }

    if let Some(node) = parse_struct(trim) {
        return Ok(node);
    }
    
    if let Some(node) = parse_array(trim) {
        return Ok(node);
    }
    
    Err("Invalid AON".to_owned())
}

fn parse_null(value: &str) -> Option<Value> {
    let trim = value.trim();

    if trim.is_empty() {
        return None;
    }

    if trim.eq_ignore_ascii_case(tokens::NULL) {
        return Some(Value::Null);
    }

    None
}

fn parse_bool(value: &str) -> Option<Value> {
    let trim = value.trim();

    if trim.is_empty() {
        return None;
    }

    if trim.eq_ignore_ascii_case(tokens::TRUE) {
        return Some(Value::Bool(true));
    }

    if trim.eq_ignore_ascii_case(tokens::FALSE) {
        return Some(Value::Bool(false));
    }

    None
}

fn parse_number(value: &str) -> Option<Value> {
    let trim = value.trim();

    if trim.is_empty() {
        return None;
    }

    match trim.parse::<f64>() {
        Ok(num) => Some(Value::Number(num)),
        Err(_) => None,
    }
}

fn parse_string(value: &str) -> Option<Value> {
    let trim = value.trim();
    
    if trim.is_empty() {
        return None;
    }

    if trim.starts_with(tokens::DOUBLE_QUOTE) && trim.ends_with(tokens::DOUBLE_QUOTE) {
        let text = trim[1..trim.len() - 1].to_owned();
        return Some(Value::String(text));
    }

    None
}

fn parse_union(array: &str) -> Option<Value> {
    let trim = array.trim();

    if trim.is_empty() {
        return None;
    }

    None
}

fn parse_struct(array: &str) -> Option<Value> {
    let trim = array.trim();

    if trim.is_empty() {
        return None;
    }

    None
}

fn parse_array(array: &str) -> Option<Value> {
    let trim = array.trim();

    if trim.is_empty() {
        return None;
    }

    None
}
