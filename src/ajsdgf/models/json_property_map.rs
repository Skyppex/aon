use std::ops::{Index, IndexMut};

use crate::{models::JsonNode, errors::JsonNodeError};

#[derive(Debug, Clone, PartialEq)]
pub struct JsonPropertyMap(Vec<(String, JsonNode)>);

impl JsonPropertyMap {
    /// Create a new property map with no mappings.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Get the `JsonNode` associated with a name.
    /// 
    /// # Arguments
    /// 
    /// * `property_name` - The name of the property you want.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonPropertyMap};
    /// 
    /// // Create node with mappings.
    /// let object_node = JsonNode::Object(JsonPropertyMap::from([
    ///     ("name".to_owned(), JsonNode::String("John Doe".to_owned())),
    ///     ("age".to_owned(), JsonNode::Integer(42)),
    /// ]));
    /// 
    /// let map = object_node.as_object().unwrap(); // &JsonPropertyMap.
    /// let property = map.get("name").unwrap(); // &JsonNode.
    /// let name = property.as_string().unwrap(); // &str.
    /// 
    /// assert_eq!(name, "John Doe");
    /// ```
    pub fn get(&self, property_name: &str) -> Option<&JsonNode> {
        self.0.iter()
              .find(|(k, _)| k == property_name)
              .map(|(_, v)| v)
    }

    /// Get the `JsonNode` associated with a name as a mutable value.
    /// 
    /// # Arguments
    /// 
    /// * `property_name` - The name of the property you want.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonPropertyMap};
    /// 
    /// // Create node with mappings.
    /// let mut object_node = JsonNode::Object(JsonPropertyMap::from([
    ///     ("name".to_owned(), JsonNode::String("John Doe".to_owned())),
    ///     ("age".to_owned(), JsonNode::Integer(42)),
    /// ]));
    /// 
    /// let mut_map = object_node.as_object_mut().unwrap(); // &mut JsonPropertyMap.
    /// let mut_property = mut_map.get_mut("name").unwrap(); // &mut JsonNode.
    /// let mut_name = mut_property.as_string_mut().unwrap(); // &mut str.
    /// 
    /// mut_name.make_ascii_uppercase(); // Mutates the string slice.
    /// 
    /// let map = object_node.as_object().unwrap(); // &JsonPropertyMap.
    /// let property = map.get("name").unwrap(); // &JsonNode.
    /// let name = property.as_string().unwrap(); // &String.
    /// 
    /// assert_eq!(name, "JOHN DOE");
    /// ```
    pub fn get_mut(&mut self, property_name: &str) -> Option<&mut JsonNode> {
        self.0.iter_mut()
              .find(|(k, _)| k == property_name)
              .map(|(_, v)| v)
    }

    /// Adds a new mapping to the object.
    /// 
    /// # Arguments
    /// 
    /// * `property_name` - Name of the new property.
    /// * `json_node` - The `JsonNode` to be associated with the `property_name`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonPropertyMap};
    /// 
    /// let mut map = JsonPropertyMap::new();
    /// 
    /// map.add("number", JsonNode::Integer(42));
    /// 
    /// let expected = JsonPropertyMap::from([
    ///     ("number".to_owned(), JsonNode::Integer(42))
    /// ]);
    /// 
    /// assert_eq!(map, expected);
    /// ```
    pub fn add(&mut self, property_name: &str, json_node: JsonNode) {
        if self.contains_property(&property_name) {
            return;
        }

        self.0.push((property_name.to_owned(), json_node));
    }

    /// Removes a mapping from the object if it exists.
    /// 
    /// # Arguments
    /// 
    /// * `property_name` - Name of the property to be removed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonPropertyMap};
    /// 
    /// let mut map = JsonPropertyMap::from([
    ///     ("number".to_owned(), JsonNode::Integer(42))
    /// ]);
    /// 
    /// map.remove("number");
    /// 
    /// let expected = JsonPropertyMap::new();
    /// assert_eq!(map, expected);
    /// ```
    pub fn remove(&mut self, property_name: &str) -> crate::Result<JsonNode> {
        if self.0.iter().filter(|(k, _)| k == property_name).count() > 1 {
            return Err(JsonNodeError::MultiplePropertiesWithSameKey(property_name.to_string()))
        }

        self.0.iter()
              .position(|(k, _)| k == property_name)
              .map(|i| self.0.remove(i).1)
              .ok_or(JsonNodeError::KeyNotFound(property_name.to_string()))
    }

    /// Checks if a property with the name `property_name` exists.
    /// 
    /// # Arguments
    /// 
    /// * `property_name` - The name to check for.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonPropertyMap};
    /// 
    /// let mut map = JsonPropertyMap::from([
    ///     ("number".to_owned(), JsonNode::Integer(42))
    /// ]);
    /// 
    /// assert!(map.contains_property("number"));
    /// assert!(!map.contains_property("name"));
    /// ```
    pub fn contains_property(&self, property_name: &str) -> bool {
        self.0.iter().any(|(k, _)| k == property_name)
    }

    /// Gets all property names in the object.
    pub fn property_names(&self) -> Vec<&String> {
        self.0.iter().map(|(k, _)| k).collect()
    }

    pub fn property_names_mut(&mut self) -> Vec<&mut String> {
        self.0.iter_mut().map(|(k, _)| k).collect()
    }

    /// Gets all child nodes in the object.
    pub fn nodes(&self) -> Vec<&JsonNode> {
        self.0.iter().map(|(_, v)| v).collect()
    }

    /// Gets all child nodes in the object as mutable references.
    pub fn nodes_mut(&mut self) -> Vec<&mut JsonNode> {
        self.0.iter_mut().map(|(_, v)| v).collect()
    }

    /// Clears the map of all mappings.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns an iterator over the mappings represented as tuples.
    pub fn iter(&self) -> std::slice::Iter<(String, JsonNode)> {
        self.0.iter()
    }

    /// Returns an iterator over the mappings represented as tuples that allows modifying each element and its name.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<(String, JsonNode)> {
        self.0.iter_mut()
    }
    
    /// Returns the number of mappings in the object.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the object has zero mappings.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Serialized the object as a JSON object string.
    /// 
    /// # Remarks
    /// 
    /// This function does zero formatting meaning the JSON string will have no spaces or new-lines.
    pub fn to_json_string(&self) -> String {
        let mut result = "{".to_string();

        for (key, value) in &self.0 {
            result.push_str(&format!("\"{}\":{},", key, value.to_json_string()));
        }

        result.pop(); // Pops the trailing comma
        result.push('}');

        result
    }
}

impl Index<usize> for JsonPropertyMap {
    type Output = (String, JsonNode);

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for JsonPropertyMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl FromIterator<(String, JsonNode)> for JsonPropertyMap {
    fn from_iter<T: IntoIterator<Item = (String, JsonNode)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<Vec<(String, JsonNode)>> for JsonPropertyMap {
    fn from(value: Vec<(String, JsonNode)>) -> Self {
        Self(value)
    }
}

impl<const COUNT: usize> From<[(String, JsonNode); COUNT]> for JsonPropertyMap {
    fn from(value: [(String, JsonNode); COUNT]) -> Self {
        Self(value.to_vec())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_mut() {
        use crate::{JsonNode, JsonPropertyMap};
        
        // Create node with mappings.
        let mut object_node = JsonNode::Object(JsonPropertyMap::from([
            ("name".to_owned(), JsonNode::String("John Doe".to_owned())),
            ("age".to_owned(), JsonNode::Integer(42)),
        ]));
        
        let mut_map = object_node.as_object_mut().unwrap(); // &mut JsonPropertyMap.
        let mut_property = mut_map.get_mut("name").unwrap(); // &mut JsonNode.
        let mut_name = mut_property.as_string_mut().unwrap(); // &mut JsonValue.

        mut_name.make_ascii_uppercase(); // Mutates the string slice.

        let map = object_node.as_object().unwrap(); // &JsonPropertyMap.
        let property = map.get("name").unwrap(); // &JsonNode.
        let name = property.as_string().unwrap(); // &JsonValue.
        
        assert_eq!(name, "JOHN DOE");
    }
}
