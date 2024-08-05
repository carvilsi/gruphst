use super::Edge;
use crate::QueryAttribute;

impl QueryAttribute for Edge {
    fn has_attr(&self, attr_k: &str) -> bool {
        self.edge.borrow().attr.has_attr(attr_k)
    }

    fn like_attr(&self, attr_k: &str) -> bool {
        self.edge.borrow().attr.like_attr(attr_k)
    }

    fn equals_attr<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.edge.borrow().attr.equals_attr(attr_k, attr_v)
    }

    fn len_attr(&self) -> usize {
        self.edge.borrow().attr.len_attr()
    }

    fn is_empty_attr(&self) -> bool {
        self.edge.borrow().attr.is_empty_attr()
    }
}
