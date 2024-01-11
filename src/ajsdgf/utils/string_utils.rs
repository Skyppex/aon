pub trait SurroundWith {
    fn surround_with(&self, left: &str, right: &str) -> String;
}

impl SurroundWith for String {
    fn surround_with(&self, left: &str, right: &str) -> String {
        format!("{}{}{}", left, self, right)
    }
}

impl SurroundWith for &str {
    fn surround_with(&self, left: &str, right: &str) -> String {
        format!("{}{}{}", left, self, right)
    }
}

impl SurroundWith for &String {
    fn surround_with(&self, left: &str, right: &str) -> String {
        format!("{}{}{}", left, self, right)
    }
}