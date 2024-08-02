use crate::vertex::Vertex;
use crate::QueryAttribute;

impl Vertex {
    /// Checks if "from" or "to" edge has an attribute
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use gruphst::vertex::Vertex;
    /// use crate::gruphst::*;
    ///
    /// let mut alice = Edge::new("Alice");
    /// alice.set_attr("Address", "Elm street");
    /// alice.set_attr("age", 42);
    ///
    /// let mut bob = Edge::new("Bob");
    /// bob.set_attr("city", "Arkham");
    ///
    /// let graph = Vertex::create(&alice, "knows", &bob);
    ///
    /// assert!(!graph.has_edge_attr("phone"));
    /// assert!(graph.has_edge_attr("age"));
    /// assert!(graph.has_edge_attr("city"));
    /// ```
    pub fn has_edge_attr(&self, attr_k: &str) -> bool {
        self.from.has_attr(attr_k) || self.to.has_attr(attr_k)
    }

    /// Checks if "from" or "to" edge has a like attribute
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use gruphst::vertex::Vertex;
    /// use crate::gruphst::*;
    ///
    /// let mut alice = Edge::new("Alice");
    /// alice.set_attr("Address", "Elm street");
    /// alice.set_attr("age", 42);
    ///
    /// let mut bob = Edge::new("Bob");
    /// bob.set_attr("city", "Arkham");
    ///
    /// let graph = Vertex::create(&alice, "knows", &bob);
    ///
    /// assert!(!graph.like_edge_attr("ph"));
    /// assert!(graph.like_edge_attr("ag"));
    /// assert!(graph.like_edge_attr("cI"));
    /// ```
    pub fn like_edge_attr(&self, attr_k: &str) -> bool {
        self.from.like_attr(attr_k) || self.to.like_attr(attr_k)
    }

    /// Checks if "from" or "to" edge has an attribute and equal for value
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use gruphst::vertex::Vertex;
    /// use crate::gruphst::*;
    ///
    /// let mut alice = Edge::new("Alice");
    /// alice.set_attr("Address", "Elm street");
    /// alice.set_attr("age", 42);
    ///
    /// let mut bob = Edge::new("Bob");
    /// bob.set_attr("city", "Arkham");
    ///
    /// let graph = Vertex::create(&alice, "knows", &bob);
    ///
    /// assert!(!graph.equals_edge_attr("phone", "555-555"));
    /// assert!(graph.equals_edge_attr("age", 42));
    /// assert!(!graph.equals_edge_attr("age", 24));
    /// ```
    pub fn equals_edge_attr<T>(&self, attr_k: &str, attr_v: T) -> bool
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        self.from.equals_attr(attr_k, attr_v.clone()) || self.to.equals_attr(attr_k, attr_v.clone())
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
