use crate::{edge::Edge, vertex::Vertex};

impl Edge {
    /// Checks if "from" or "to" vertices has an String attribute key
    pub fn has_vertex_with_attr_str_key(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_str_key_equals_to(attr_k) || self.get_to_vertex().has_attr_str_key_equals_to(attr_k)
    }
    
    /// Checks if "from" or "to" vertices has any attribute key
    pub fn has_vertex_with_attr_key(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_key(attr_k) || self.get_to_vertex().has_attr_key(attr_k)
    }
    /// Checks if "from" or "to" vertex has an attribute like
    pub fn has_vertex_with_attr_key_like(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_str_key_like(attr_k) || self.get_to_vertex().has_attr_str_key_like(attr_k)
    }

    /// Checks if "from" or "to" vertex has an attribute and equal for value
    pub fn has_vertex_with_attr_value_equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.get_from_vertex()
            .has_attr_str_equals_to(attr_k, attr_v.clone())
            || self.get_to_vertex().has_attr_str_equals_to(attr_k, attr_v.clone())
    }

    /// Checks if an attribute key exists
    pub fn has_attr_key(&self, attr_k: &str) -> bool {
        self.attr.contains_key(attr_k)
    }

    /// Checks if an attribute key is like on a edge
    pub fn has_attr_key_like(&self, attr_k: &str) -> bool {
        for key in self.attr.keys() {
            if key.to_lowercase().contains(&attr_k.to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Checks if an attribute key exists on a edge
    /// and the value matchs
    pub fn has_attr_equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
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
    pub fn attr_len(&self) -> usize {
        self.attr.len()
    }

    /// Checks if attributes for a edge is empty
    pub fn attr_is_empty(&self) -> bool {
        self.attr_len() == 0
    }

    // finds a Vertex by Id on an Edge
    pub fn find_vertex_by_id(&self, id: &str) -> Result<Vertex, &'static str> {
        if self.get_from_vertex().get_id() == *id {
            Ok(self.get_from_vertex())
        } else if self.get_to_vertex().get_id() == *id {
            Ok(self.get_to_vertex())
        } else {
            Err("Vertex not found")
        }
    }
}
