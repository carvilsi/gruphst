use crate::edge::Edge;
use crate::QueryAttribute;

impl Edge {
    /// Checks if "from" or "to" vertices has an attribute
    pub fn has_vertex_with_attr_key(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr(attr_k) || self.get_to_vertex().has_attr(attr_k)
    }

    /// Checks if "from" or "to" vertex has an attribute like
    pub fn has_vertex_with_attr_key_like(&self, attr_k: &str) -> bool {
        self.get_from_vertex().has_attr_like(attr_k) || self.get_to_vertex().has_attr_like(attr_k)
    }

    /// Checks if "from" or "to" vertex has an attribute and equal for value
    pub fn has_vertex_with_attr_value_equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.get_from_vertex()
            .attr_equals_to(attr_k, attr_v.clone())
            || self.get_to_vertex().attr_equals_to(attr_k, attr_v.clone())
    }
}

impl QueryAttribute for Edge {
    fn has_attr_key(&self, attr_k: &str) -> bool {
        self.attr.has_attr_key(attr_k)
    }

    fn has_attr_key_like(&self, attr_k: &str) -> bool {
        self.attr.has_attr_key_like(attr_k)
    }

    fn has_attr_equals_to<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.attr.has_attr_equals_to(attr_k, attr_v)
    }

    fn attr_len(&self) -> usize {
        self.attr.attr_len()
    }

    fn attr_is_empty(&self) -> bool {
        self.attr.attr_is_empty()
    }
}
