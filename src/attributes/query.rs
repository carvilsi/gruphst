use super::Attributes;
use crate::QueryAttribute;

impl QueryAttribute for Attributes {
    /// Checks if an attribute key exists
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use crate::gruphst::*;
    ///
    /// let mut edge = Edge::new("Alice");
    /// edge.set_attr("Address", "Elm street");
    /// edge.set_attr("age", 42);
    ///
    /// assert!(!edge.has_attr("phone"));
    /// assert!(edge.has_attr("age"));
    /// ```
    fn has_attr(&self, attr_k: &str) -> bool {
        self.attr.contains_key(attr_k)
    }

    /// Checks if an attribute key is like on a edge
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use crate::gruphst::*;
    ///
    /// let mut edge = Edge::new("Alice");
    /// edge.set_attr("Address", "Elm street");
    /// edge.set_attr("age", 42);
    ///
    /// assert!(!edge.like_attr("ph"));
    /// assert!(edge.like_attr("ag"));
    /// assert!(edge.like_attr("adDr"));
    /// ```
    fn like_attr(&self, attr_k: &str) -> bool {
        for key in self.attr.keys() {
            if key.to_lowercase().contains(&attr_k.to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Checks if an attribute key exists on a edge
    /// and the value matchs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use crate::gruphst::*;
    ///
    /// let mut edge = Edge::new("Alice");
    /// edge.set_attr("Address", "Elm street");
    /// edge.set_attr("age", 42);
    ///
    /// assert!(!edge.equals_attr("phone", "555-555"));
    /// assert!(edge.equals_attr("age", 42));
    /// assert!(!edge.equals_attr("age", 24));
    /// ```
    fn equals_attr<T>(&self, attr_k: &str, attr_v: T) -> bool
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
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use crate::gruphst::*;
    ///
    /// let mut edge = Edge::new("Alice");
    /// edge.set_attr("Address", "Elm street");
    /// edge.set_attr("age", 25);
    /// assert_eq!(edge.len_attr(), 2);
    /// ```
    fn len_attr(&self) -> usize {
        self.attr.len()
    }

    /// Checks if attributes for a edge is empty
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use crate::gruphst::*;
    ///
    /// let mut edge = Edge::new("Alice");
    /// assert!(edge.is_empty_attr());
    /// edge.set_attr("Address", "Elm street");
    /// edge.set_attr("age", 25);
    /// assert!(!edge.is_empty_attr());
    /// ```
    fn is_empty_attr(&self) -> bool {
        self.len_attr() == 0
    }
}
