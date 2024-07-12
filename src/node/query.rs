use log::{debug, warn};

use crate::node::Node;

impl Node {
    /// Get attribute for a Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// let attr = node.get_attr("Address").unwrap();
    /// assert_eq!(attr, "Elm street");
    /// ```
    pub fn get_attr(&self, attr_k: &str) -> Result<&String, &'static str> {
        let res = self.attr.get(attr_k);
        match res {
            Some(res) => {
                debug!(
                    "retrieved attribute value '{}' for '{}' for node [{}]",
                    res, attr_k, self.id
                );
                Ok(res)
            }
            None => {
                warn!("attribute '{}' not found", attr_k);
                Err("attribute not found")
            }
        }
    }

    /// Checks if an attribute key exists on a node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// node.set_attr("age", 42);
    ///
    /// assert!(!node.has_attr("phone"));
    /// assert!(node.has_attr("age"));
    /// ```
    pub fn has_attr(&self, attr_k: &str) -> bool {
        self.attr.contains_key(attr_k)
    }

    /// Checks if an attribute key is like on a node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// node.set_attr("age", 42);
    ///
    /// assert!(!node.like_attr("ph"));
    /// assert!(node.like_attr("ag"));
    /// assert!(node.like_attr("adDr"));
    /// ```
    pub fn like_attr(&self, attr_k: &str) -> bool {
        for key in self.attr.keys() {
            if key.to_lowercase().contains(&attr_k.to_lowercase()) {
                return true;
            }
        }
        false
    }

    /// Checks if an attribute key exists on a node
    /// and the value matchs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    ///
    /// let mut node = Node::new("Alice");
    /// node.set_attr("Address", "Elm street");
    /// node.set_attr("age", 42);
    ///
    /// assert!(!node.equals_attr("phone", "555-555"));
    /// assert!(node.equals_attr("age", 42));
    /// assert!(!node.equals_attr("age", 24));
    /// ```
    pub fn equals_attr<T>(&self, attr_k: &str, attr_v: T) -> bool
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
}
