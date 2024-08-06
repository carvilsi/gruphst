use crate::vertex::Vertex;
use crate::QueryAttribute;

impl Vertex {
    /// Checks if "from" or "to" edge has an attribute
    pub fn has_edge_with_attr_key(&self, attr_k: &str) -> bool {
        self.get_from_edge().has_attr(attr_k) || self.get_to_edge().has_attr(attr_k)
    }

    /// Checks if "from" or "to" edge has a like attribute
    pub fn has_edge_with_attr_key_like(&self, attr_k: &str) -> bool {
        self.get_from_edge().has_attr_like(attr_k) || self.get_to_edge().has_attr_like(attr_k)
    }

    /// Checks if "from" or "to" edge has an attribute and equal for value
    pub fn has_edge_with_attr_value_equal<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.get_from_edge().attr_equals_to(attr_k, attr_v.clone())
            || self.get_to_edge().attr_equals_to(attr_k, attr_v.clone())
    }
}

impl QueryAttribute for Vertex {
    fn has(&self, attr_k: &str) -> bool {
        self.attr.has(attr_k)
    }

    fn like(&self, attr_k: &str) -> bool {
        self.attr.like(attr_k)
    }

    fn equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.attr.equals_to(attr_k, attr_v)
    }

    fn len(&self) -> usize {
        self.attr.len()
    }

    fn is_empty(&self) -> bool {
        self.attr.is_empty()
    }
}
