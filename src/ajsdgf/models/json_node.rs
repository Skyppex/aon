use std::fmt::Display;

use crate::models::JsonPropertyMap;
use crate::parsing::JsonNodeParser;
use crate::utils::SurroundWith;
use crate::Result;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonNode {
    Object(JsonPropertyMap),
    Array(Vec<JsonNode>),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl JsonNode {
    /// Parse a JSON string slice into a `JsonNode` structure.
    /// 
    /// # Arguments
    /// 
    /// * `json` - The JSON you wish to be parsed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// // Create a valid JSON string.
    /// let json = "10";
    /// // Manually create a tree with the expected structure and value.
    /// let expected = JsonNode::Integer(10);
    /// 
    /// // Parse the json string into a node tree.
    /// let node_tree = JsonNode::parse(json).unwrap();
    /// 
    /// assert_eq!(node_tree, expected);
    /// ```
    pub fn parse(json: &str) -> Result<JsonNode> {
        JsonNodeParser::parse_node(json, None)
    }

    /// Checks if the node is the JsonNode::Object discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonPropertyMap};
    /// 
    /// // Create an object node.
    /// let object_node = JsonNode::Object(JsonPropertyMap::new());
    /// // Create a non-object node.
    /// let non_object_node = JsonNode::Null;
    /// 
    /// assert!(object_node.is_object());
    /// assert!(!non_object_node.is_object())
    /// ```
    pub fn is_object(&self) -> bool {
        match self {
            JsonNode::Object(_) => true,
            _ => false,
        }
    }

    /// Checks if the node is the JsonNode::Array discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// // Create an array node.
    /// let array_node = JsonNode::Array(Vec::new());
    /// // Create a non-array node.
    /// let non_array_node = JsonNode::Null;
    /// 
    /// assert!(array_node.is_array());
    /// assert!(!non_array_node.is_array())
    /// ```
    pub fn is_array(&self) -> bool {
        match self {
            JsonNode::Array(_) => true,
            _ => false,
        }
    }

    /// Extracts the `JsonPropertyMap` contained inside the node if it is the `JsonNode::Object` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonPropertyMap};
    /// 
    /// // Create an object node.
    /// let object_node = JsonNode::Object(JsonPropertyMap::new());
    /// 
    /// // Extract `JsonPropertyMap`.
    /// let as_object_some = object_node.as_object(); // Option<&JsonPropertyMap>
    /// 
    /// assert_eq!(as_object_some, Some(&JsonPropertyMap::new()));
    /// 
    /// // Create a non-object node.
    /// let non_object_node = JsonNode::Null;
    /// 
    /// // Fail to extract `JsonPropertyMap`.
    /// let as_object_none = non_object_node.as_object();
    /// 
    /// assert_eq!(as_object_none, None);
    /// ```
    pub fn as_object(&self) -> Option<&JsonPropertyMap> {
        match self {
            JsonNode::Object(object) => Some(object),
            _ => None,
        }
    }

    /// Extracts the `Vec<JsonNode>` contained inside the node if it is the `JsonNode::Array` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// // Create an array node.
    /// let array_node = JsonNode::Array(Vec::new());
    /// 
    /// // Extract `Vec<JsonNode>`.
    /// let as_array_some = array_node.as_array(); // Option<&Vec<JsonNode>>
    /// 
    /// assert_eq!(as_array_some, Some(&Vec::new()));
    /// 
    /// // Create a non-array node.
    /// let non_array_node = JsonNode::Null;
    /// 
    /// // Fail to extract `Vec<JsonNode>`.
    /// let as_array_none = non_array_node.as_array();
    /// 
    /// assert_eq!(as_array_none, None);
    /// ```
    pub fn as_array(&self) -> Option<&Vec<JsonNode>> {
        match self {
            JsonNode::Array(array) => Some(array),
            _ => None,
        }
    }

    /// Extracts the `JsonPropertyMap` contained inside the node if it is the `JsonNode::Object` discriminant as a mutable value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonPropertyMap};
    /// 
    /// // Create an object node.
    /// let mut object_node = JsonNode::Object(JsonPropertyMap::new());
    /// 
    /// // Extract `JsonPropertyMap`.
    /// let as_object_some = object_node.as_object_mut(); // Option<&mut JsonPropertyMap>
    /// 
    /// assert_eq!(as_object_some, Some(&mut JsonPropertyMap::new()));
    /// 
    /// // Create a non-object node.
    /// let mut non_object_node = JsonNode::Null;
    /// 
    /// // Fail to extract `JsonPropertyMap`.
    /// let as_object_none = non_object_node.as_object_mut();
    /// 
    /// assert_eq!(as_object_none, None);
    /// ```
    pub fn as_object_mut(&mut self) -> Option<&mut JsonPropertyMap> {
        match self {
            JsonNode::Object(object) => Some(object),
            _ => None,
        }
    }

    /// Extracts the `Vec<JsonNode>` contained inside the node if it is the `JsonNode::Array` discriminant as a mutable value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonPropertyMap};
    /// 
    /// // Create an array node.
    /// let mut array_node = JsonNode::Array(Vec::new());
    /// 
    /// // Extract `Vec<JsonNode>`.
    /// let as_array_some = array_node.as_array_mut(); // Option<&mut Vec<JsonNode>>
    /// 
    /// assert_eq!(as_array_some, Some(&mut Vec::new()));
    /// 
    /// // Create a non-array node.
    /// let mut non_array_node = JsonNode::Null;
    /// 
    /// // Fail to extract `JsonPropertyMap`.
    /// let as_array_none = non_array_node.as_array_mut();
    /// 
    /// assert_eq!(as_array_none, None);
    /// ```
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<JsonNode>> {
        match self {
            JsonNode::Array(array) => Some(array),
            _ => None,
        }
    }

    /// Checks if the value is the `JsonNode::String` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let string_value = JsonNode::String("Hello World!".to_owned());
    /// let non_string_value = JsonNode::Null;
    /// 
    /// assert!(string_value.is_string());
    /// assert!(!non_string_value.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        match self {
            JsonNode::String(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is the `JsonNode::Integer` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let integer_value = JsonNode::Integer(42);
    /// let non_integer_value = JsonNode::Null;
    /// 
    /// assert!(integer_value.is_integer());
    /// assert!(!non_integer_value.is_integer());
    /// ```
    pub fn is_integer(&self) -> bool {
        match self {
            JsonNode::Integer(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is the `JsonNode::Float` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let float_value = JsonNode::Float(3.14);
    /// let non_float_value = JsonNode::Null;
    /// 
    /// assert!(float_value.is_float());
    /// assert!(!non_float_value.is_float());
    /// ```
    pub fn is_float(&self) -> bool {
        match self {
            JsonNode::Float(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is the `JsonNode::Boolean` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let bool_value = JsonNode::Boolean(true);
    /// let non_bool_value = JsonNode::Null;
    /// 
    /// assert!(bool_value.is_bool());
    /// assert!(!non_bool_value.is_bool());
    /// ```
    pub fn is_bool(&self) -> bool {
        match self {
            JsonNode::Boolean(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is the `JsonNode::Null` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let null_value = JsonNode::Null;
    /// let non_null_value = JsonNode::Integer(42);
    /// 
    /// assert!(null_value.is_null());
    /// assert!(!non_null_value.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        match self {
            JsonNode::Null => true,
            _ => false,
        }
    }

    /// Extracts the inner `str` contained inside the node if it is the `JsonNode::String` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let string_value = JsonNode::String("Hello World!".to_owned());
    /// let non_string_value = JsonNode::Null;
    /// 
    /// assert_eq!(string_value.as_string(), Some("Hello World!"));
    /// assert_eq!(non_string_value.as_string(), None);
    /// ```
    pub fn as_string(&self) -> Option<&str> {
        match self {
            JsonNode::String(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `i64` contained inside the node if it is the `JsonNode::Integer` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let integer_value = JsonNode::Integer(42);
    /// let non_integer_value = JsonNode::Null;
    /// 
    /// assert_eq!(integer_value.as_integer(), Some(&42));
    /// assert_eq!(non_integer_value.as_integer(), None);
    /// ```
    pub fn as_integer(&self) -> Option<&i64> {
        match self {
            JsonNode::Integer(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `f64` contained inside the node if it is the `JsonNode::Float` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let float_value = JsonNode::Float(3.14);
    /// let non_float_value = JsonNode::Null;
    /// 
    /// assert_eq!(float_value.as_float(), Some(&3.14));
    /// assert_eq!(non_float_value.as_float(), None);
    /// ```
    pub fn as_float(&self) -> Option<&f64> {
        match self {
            JsonNode::Float(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `bool` contained inside the node if it is the `JsonNode::Boolean` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let bool_value = JsonNode::Boolean(true);
    /// let non_bool_value = JsonNode::Null;
    /// 
    /// assert_eq!(bool_value.as_boolean(), Some(&true));
    /// assert_eq!(non_bool_value.as_boolean(), None);
    /// ```
    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            JsonNode::Boolean(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `mut str` contained inside the node if it is the `JsonNode::String` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let mut string_value = JsonNode::String("Hello World!".to_owned());
    /// let mut non_string_value = JsonNode::Null;
    /// 
    /// assert_eq!(string_value.as_string_mut(), Some("Hello World!".to_string().as_mut_str()));
    /// assert_eq!(non_string_value.as_string_mut(), None);
    /// ```
    pub fn as_string_mut(&mut self) -> Option<&mut str> {
        match self {
            JsonNode::String(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `mut i64` contained inside the node if it is the `JsonNode::Integer` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let mut integer_value = JsonNode::Integer(42);
    /// let mut non_integer_value = JsonNode::Null;
    /// 
    /// assert_eq!(integer_value.as_integer_mut(), Some(&mut 42));
    /// assert_eq!(non_integer_value.as_integer_mut(), None);
    /// ```
    pub fn as_integer_mut(&mut self) -> Option<&mut i64> {
        match self {
            JsonNode::Integer(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `mut f64` contained inside the node if it is the `JsonNode::Float` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let mut float_value = JsonNode::Float(3.14);
    /// let mut non_float_value = JsonNode::Null;
    /// 
    /// assert_eq!(float_value.as_float_mut(), Some(&mut 3.14));
    /// assert_eq!(non_float_value.as_float_mut(), None);
    /// ```
    pub fn as_float_mut(&mut self) -> Option<&mut f64> {
        match self {
            JsonNode::Float(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `mut bool` contained inside the node if it is the `JsonNode::Boolean` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// let mut bool_value = JsonNode::Boolean(true);
    /// let mut non_bool_value = JsonNode::Null;
    /// 
    /// assert_eq!(bool_value.as_boolean_mut(), Some(&mut true));
    /// assert_eq!(non_bool_value.as_boolean_mut(), None);
    /// ```
    pub fn as_boolean_mut(&mut self) -> Option<&mut bool> {
        match self {
            JsonNode::Boolean(value) => Some(value),
            _ => None,
        }
    }

    /// Convert the node tree to a JSON string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    /// 
    /// // Create a JsonNode tree.
    /// let node_tree = JsonNode::Array(Vec::from([
    ///     JsonNode::Integer(0),
    ///     JsonNode::Float(0.5),
    ///     JsonNode::Integer(1),
    ///     JsonNode::Null,
    ///     JsonNode::Boolean(false)
    /// ]));
    /// 
    /// let json_string = node_tree.to_json_string();
    /// 
    /// assert_eq!(json_string, "[0,0.5,1,null,false]".to_owned());
    /// ```
    /// 
    /// # Remarks
    /// 
    /// This function does zero formatting. The entire JSON string is returned without any spaces or new-lines.
    pub fn to_json_string(&self) -> String {
        match self {
            JsonNode::String(value) => value.to_string().to_string().surround_with("\"", "\""),
            JsonNode::Integer(value) => value.to_string(),
            JsonNode::Float(value) => value.to_string(),
            JsonNode::Boolean(value) => value.to_string(),
            JsonNode::Null => String::from("null"),
            JsonNode::Object(object) => object.to_json_string(),
            JsonNode::Array(array) => {
                array
                .iter()
                .map(|node| node.to_json_string())
                .collect::<Vec<String>>()
                .join(",")
                .surround_with("[", "]")
            },
        }
    }
    
}

impl<'a> IntoIterator for &'a JsonNode {
    type Item = &'a JsonNode;
    type IntoIter = Iter<'a>;

    /// Turns the node tree into an iterator which iterates over evey `JsonNode` in the tree in a depth first manner.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonNode;
    ///     
    /// let node_tree = JsonNode::Array(Vec::from([
    ///     JsonNode::Array(Vec::from([                     // First element is an array with the value `1` inside.
    ///         JsonNode::Integer(1),
    ///     ])),
    ///     JsonNode::Integer(2),         // Second element is the value `2`.
    ///     JsonNode::Array(Vec::from([
    ///         JsonNode::Integer(3)      // Third element is an array with the value `3` inside.
    ///     ]))
    /// ]));
    /// 
    /// let sequence = node_tree.into_iter().collect::<Vec<&JsonNode>>();
    /// 
    /// let expected = vec![
    ///     &JsonNode::Integer(1),
    ///     &JsonNode::Integer(2),
    ///     &JsonNode::Integer(3)
    /// ];
    /// 
    /// assert_eq!(sequence, expected);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            node: Some(self),
            array_index: None,
            object_index: None,
            child: None,
        }
    }
}

pub struct Iter<'a> {
    node: Option<&'a JsonNode>,
    array_index: Option<usize>,
    object_index: Option<usize>,
    child: Option<Box<Iter<'a>>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a JsonNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.child {
            if let Some(node) = iter.next() {
                return Some(node);
            }

            self.child = None;
        }

        if let None = self.node {
            return None; // Termination point for iteration. If the iterator has recursed, this allows the parent iterator to continue.
        }

        match self.node.unwrap() {
            JsonNode::Array(nodes) => {
                match self.array_index {
                    Some(mut index) => {
                        index = index + 1;
                        self.array_index = Some(index);
                        self.child = Some(Box::new(nodes[index].into_iter()));
                        let next = self.next();

                        if index == nodes.len() - 1 {
                            self.array_index = None;
                            self.node = None;
                        }

                        return next;
                    },
                    None => {
                        self.array_index = Some(0);
                        self.child = Some(Box::new(nodes[0].into_iter()));
                        let next = self.next();

                        if nodes.len() == 1 {
                            self.array_index = None;
                            self.node = None;
                        }

                        return next;
                    },
                }
            },
            JsonNode::Object(properties) => {
                match self.object_index {
                    Some(mut index) => {
                        index = index + 1;
                        self.object_index = Some(index);
                        self.child = Some(Box::new(properties[index].1.into_iter()));
                        let next = self.next();

                        if index == properties.len() - 1 {
                            self.object_index = None;
                            self.node = None;
                        }

                        return next;
                    },
                    None => {
                        self.object_index = Some(0);
                        self.child = Some(Box::new(properties[0].1.into_iter()));
                        let next = self.next();

                        if properties.len() == 1 {
                            self.object_index = None;
                        }

                        return next;
                    },
                }
            },
            _ => {
                let node = self.node.unwrap();
                self.node = None;
                Some(node)
            },
        }
    }
}

impl Display for JsonNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonNode::String(value) => write!(f, "{}", value),
            JsonNode::Integer(value) => write!(f, "{}", value),
            JsonNode::Float(value) => write!(f, "{}", value),
            JsonNode::Boolean(value) => write!(f, "{}", value),
            JsonNode::Null => write!(f, "null"),
            JsonNode::Object(object) => write!(f, "{}", object.to_json_string()),
            JsonNode::Array(array) => write!(f, "{}", {
                array
                .iter()
                .map(|node| node.to_json_string())
                .collect::<Vec<String>>()
                .join(",")
                .surround_with("[", "]")
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::JsonNode;

    #[test]
    fn iterate_works() {
        let json = r#"
        {
            "name": "Jason",
            "age": 30,
            "isMale": true,
            "height": 1.8,
            "numbers": [1, 2, 3, 4, 5],
            "children": [
                {
                    "name": "Jason Jr.",
                    "age": 5,
                    "isMale": true,
                    "height": 1.2
                },
                {
                    "name": "Jasmine",
                    "age": 3,
                    "isMale": false,
                    "height": 1.1
                }
            ]
        }"#;

        let node = JsonNode::parse(json).unwrap();
        for e in node.into_iter() {
            println!("{:?}", e)
        }
    }
}

#[cfg(test)]
mod doc_tests{

    #[test]
    fn parse_doc() {
        use super::JsonNode;
        
        // Create a valid JSON string.
        let json = "10";
        // Manually create a tree with the expected structure and value.
        let expected = JsonNode::Integer(10);

        // Parse the json string into a node tree.
        let node_tree = JsonNode::parse(json).unwrap();
        
        assert_eq!(node_tree, expected);
    }

    #[test]
    fn into_iter() {
        use crate::JsonNode;
        
        let node_tree = JsonNode::Array(Vec::from([
            JsonNode::Array(Vec::from([                     // First element is an array with the value `1` inside.
                JsonNode::Integer(1),
            ])),
            JsonNode::Integer(2),         // Second element is the value `2`.
            JsonNode::Array(Vec::from([
                JsonNode::Integer(3)      // Third element is an array with the value `3` inside.
            ]))
        ]));
        
        let sequence = node_tree.into_iter().collect::<Vec<&JsonNode>>();

        let expected = vec![
            &JsonNode::Integer(1),
            &JsonNode::Integer(2),
            &JsonNode::Integer(3)
        ];

        assert_eq!(sequence, expected);
    }
}
