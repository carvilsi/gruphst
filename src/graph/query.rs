use crate::graph::Graph;
use crate::QueryAttr;

impl Graph {
    /// Checks if "from" or "to" node has an attribute
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    ///
    /// let mut alice = Node::new("Alice");
    /// alice.set_attr("Address", "Elm street");
    /// alice.set_attr("age", 42);
    ///
    /// let mut bob = Node::new("Bob");
    /// bob.set_attr("city", "Arkham");
    ///
    /// let graph = Graph::new(&alice, "knows", &bob);
    ///
    /// assert!(!graph.has_node_attr("phone"));
    /// assert!(graph.has_node_attr("age"));
    /// assert!(graph.has_node_attr("city"));
    /// ```
    pub fn has_node_attr(&self, attr_k: &str) -> bool {
        self.from.has_attr(attr_k) || self.to.has_attr(attr_k)
    }

    /// Checks if "from" or "to" node has a like attribute
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    ///
    /// let mut alice = Node::new("Alice");
    /// alice.set_attr("Address", "Elm street");
    /// alice.set_attr("age", 42);
    ///
    /// let mut bob = Node::new("Bob");
    /// bob.set_attr("city", "Arkham");
    ///
    /// let graph = Graph::new(&alice, "knows", &bob);
    ///
    /// assert!(!graph.like_node_attr("ph"));
    /// assert!(graph.like_node_attr("ag"));
    /// assert!(graph.like_node_attr("cI"));
    /// ```
    pub fn like_node_attr(&self, attr_k: &str) -> bool {
        self.from.like_attr(attr_k) || self.to.like_attr(attr_k)
    }

    /// Checks if "from" or "to" node has an attribute and equal for value
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    ///
    /// let mut alice = Node::new("Alice");
    /// alice.set_attr("Address", "Elm street");
    /// alice.set_attr("age", 42);
    ///
    /// let mut bob = Node::new("Bob");
    /// bob.set_attr("city", "Arkham");
    ///
    /// let graph = Graph::new(&alice, "knows", &bob);
    ///
    /// assert!(!graph.equals_node_attr("phone", "555-555"));
    /// assert!(graph.equals_node_attr("age", 42));
    /// assert!(!graph.equals_node_attr("age", 24));
    /// ```
    pub fn equals_node_attr<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.from.equals_attr(attr_k, attr_v.clone()) || self.to.equals_attr(attr_k, attr_v.clone())
    }
}
