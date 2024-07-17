use super::Node;
use crate::QueryAttribute;

impl QueryAttribute for Node {
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
