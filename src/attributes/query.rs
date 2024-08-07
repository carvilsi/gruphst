use super::Attributes;
use crate::QueryAttribute;

impl QueryAttribute for Attributes {
    /// Checks if an attribute key exists
    fn has_attr_key(&self, attr_k: &str) -> bool {
        self.attr.contains_key(attr_k)
    }

    /// Checks if an attribute key is like on a edge
    fn has_attr_key_like(&self, attr_k: &str) -> bool {
        for key in self.attr.keys() {
            if key.to_lowercase().contains(&attr_k.to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Checks if an attribute key exists on a edge
    /// and the value matchs
    fn has_attr_equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        match self.attr.get(attr_k) {
            Some(val) => {
                let v = attr_v.clone();
                *val == v.to_string()
            }
            None => false,
        }
    }

    /// Retrieves the lenght of attributes for a edge
    fn attr_len(&self) -> usize {
        self.attr.len()
    }

    /// Checks if attributes for a edge is empty
    fn attr_is_empty(&self) -> bool {
        self.attr_len() == 0
    }
}
