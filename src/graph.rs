use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::node::Node;

/// Representation of a Graph, relating two nodes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Graph {
    /// A Graph has an uuid
    pub id: String,
    /// A name fot the relation
    pub relation: String,
    /// Origin node
    pub from: Node,
    /// Target node
    pub to: Node,
}

impl Graph {
    /// Creates a Graph, the id is generated
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob_graph =
    ///     Graph::new(&alice, "friend of", &bob);
    /// ```
    pub fn new(from: &Node, relation: &str, to: &Node) -> Self {
        let graph = Graph {
            relation: String::from(relation),
            id: Uuid::new_v4().to_string(),
            from: from.clone(),
            to: to.clone(),
        };
        debug!("The created Graph: {:#?}", graph);
        graph
    }

    /// Updates the relation for the Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    ///
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let mut alice_bob_graph = Graph::new(&alice, "friend of", &bob);
    ///
    /// assert_eq!(alice_bob_graph.relation, "friend of");
    ///
    /// alice_bob_graph.update_relation("best friends");
    /// assert_eq!(alice_bob_graph.relation, "best friends");
    /// ```
    pub fn update_relation(&mut self, relation: &str) {
        debug!("Updated Graph [{}] with Relation: {}", self.id, relation);
        self.relation = relation.to_string();
    }

    /// Updates the "from" node in Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    ///
    ///
    /// let mut alice_node = Node::new("alice node");
    /// let bob_node = Node::new("bob node");
    /// let mut graph = Graph::new(&alice_node, "best friends", &bob_node);
    /// assert_eq!(graph.from.name, "alice node");
    /// assert_eq!(graph.to.name, "bob node");
    /// alice_node.update_name("alice");
    /// graph.update_from(&alice_node);
    /// assert_eq!(graph.from.name, "alice");
    /// ```
    pub fn update_from(&mut self, from_node: &Node) {
        debug!("Updated Graph [{}] from Node: {:#?}", self.id, from_node);
        self.from = from_node.clone();
    }

    /// Updates the "to" node in Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    ///
    ///
    /// let alice_node = Node::new("alice node");
    /// let bob_node = Node::new("bob node");
    /// let mut graph = Graph::new(&alice_node, "best friends", &bob_node);
    /// assert_eq!(graph.from.name, "alice node");
    /// assert_eq!(graph.to.name, "bob node");
    /// let fred_node = Node::new("fred node");
    /// graph.update_to(&fred_node);
    /// assert_eq!(graph.to.name, "fred node");
    /// assert_ne!(graph.to.id, bob_node.id);
    /// ```
    pub fn update_to(&mut self, to_node: &Node) {
        debug!("Updated Graph [{}] to Node: {:#?}", self.id, to_node);
        self.to = to_node.clone();
    }

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
        if self.from.has_attr(attr_k) || self.to.has_attr(attr_k) {
            return true;
        } else {
            return false;
        }
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
    pub fn equals_node_attr<T> (&self, attr_k: &str, attr_v: T) -> bool 
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        if self.from.equals_attr(attr_k, attr_v.clone()) || self.to.equals_attr(attr_k, attr_v.clone()) {
            return true;
        } else {
            return false;
        }
    }
}
