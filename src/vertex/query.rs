use super::Vertex;

impl Vertex {
    /// Checks if an attribute key exists
    pub fn has_attr(&self, attr_k: &str) -> bool {
        self.vrtx.borrow().attr.contains_key(attr_k)
    }

    /// Checks if an attribute key is like on a vertex
    pub fn has_attr_like(&self, attr_k: &str) -> bool {
        for key in self.vrtx.borrow().attr.keys() {
            if key.to_lowercase().contains(&attr_k.to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Checks if an attribute key exists on a vertex
    /// and the value matchs
    pub fn attr_equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        match self.vrtx.borrow().attr.get(attr_k) {
            Some(val) => {
                let v = attr_v.clone();
                *val == v.to_string()
            }
            None => false,
        }
    }

    /// Retrieves the lenght of attributes for a vertex
    pub fn attr_len(&self) -> usize {
        self.vrtx.borrow().attr.len()
    }

    /// Checks if attributes for a vertex is empty
    pub fn attr_is_empty(&self) -> bool {
        self.attr_len() == 0
    }
}
