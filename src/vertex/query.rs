use crate::vertex::Vertex;
use crate::QueryAttribute;

impl Vertex {
    /// Checks if "from" or "to" edge has an attribute
    pub fn has_edge_attr(&self, attr_k: &str) -> bool {
        self.get_from_edge().has_attr(attr_k) || self.get_to_edge().has_attr(attr_k)
    }

    /// Checks if "from" or "to" edge has a like attribute
    pub fn like_edge_attr(&self, attr_k: &str) -> bool {
        self.get_from_edge().like_attr(attr_k) || self.get_to_edge().like_attr(attr_k)
    }

    /// Checks if "from" or "to" edge has an attribute and equal for value
    pub fn equals_edge_attr<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.get_from_edge().equals_attr(attr_k, attr_v.clone()) || self.get_to_edge().equals_attr(attr_k, attr_v.clone())
    }
}

impl QueryAttribute for Vertex {
    fn has_attr(&self, attr_k: &str) -> bool {
        self.attr.has_attr(attr_k)
    }

    fn like_attr(&self, attr_k: &str) -> bool {
        self.attr.like_attr(attr_k)
    }

    fn equals_attr<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.attr.equals_attr(attr_k, attr_v)
    }

    fn len_attr(&self) -> usize {
        self.attr.len_attr()
    }

    fn is_empty_attr(&self) -> bool {
        self.attr.is_empty_attr()
    }
}
