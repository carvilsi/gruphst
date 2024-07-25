use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{attributes::Attributes, node::Node, CURNodeGraph, RUDAttribute};

mod query;

/// Representation of a Graph, relating two nodes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Graph {
    /// A Graph has an uuid
    id: String,
    /// A label for the relation
    relation: String,
    /// Origin node
    from: Node,
    /// Target node
    to: Node,
    /// Attributes for the Graph
    attr: Attributes,
}

impl CURNodeGraph for Graph {
    fn new(label: &str) -> Self {
        Graph {
            id: Uuid::new_v4().to_string(),
            relation: label.to_string(),
            from: Node::new(""),
            to: Node::new(""),
            attr: Attributes::new(),
        }
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_label(&self) -> String {
        self.relation.clone()
    }

    fn set_label(&mut self, label: &str) {
        self.relation = label.to_string()
    }

    fn get_attributes(&self) -> Attributes {
        self.attr.clone()
    }

    fn set_attributes(&mut self, attributes: Attributes) {
        self.attr = attributes;
    }
}

impl Graph {
    /// Adds "From" and "To" node
    /// to a previous created Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use crate::gruphst::*;
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let mut alice_bob_graph = Graph::new("");
    /// alice_bob_graph.add_relation(&alice, "friend of", &bob);
    /// assert_eq!(alice_bob_graph.get_relation(), "friend of");
    /// ```
    pub fn add_relation(&mut self, from: &Node, relation: &str, to: &Node) {
        self.relation = String::from(relation);
        self.from = from.clone();
        self.to = to.clone();
        debug!("Added relation to Graph: {:#?}", self);
    }
    /// Creates a Graph,
    /// providing "From" and "To" nodes and the "relation"
    /// the id is generated
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use crate::gruphst::*;
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob_graph =
    ///     Graph::create(&alice, "friend of", &bob);
    /// assert_eq!(alice_bob_graph.get_relation(), "friend of");
    /// ```
    pub fn create(from: &Node, relation: &str, to: &Node) -> Self {
        let mut g = Graph::new(relation);
        g.from = from.clone();
        g.to = to.clone();
        g
    }

    /// Updates the relation for the Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use crate::gruphst::*;
    ///
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let mut alice_bob_graph = Graph::create(&alice, "friend of", &bob);
    ///
    /// assert_eq!(alice_bob_graph.get_relation(), "friend of");
    ///
    /// alice_bob_graph.update_relation("best friends");
    /// assert_eq!(alice_bob_graph.get_relation(), "best friends");
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
    /// use crate::gruphst::*;
    ///
    ///
    /// let mut alice_node = Node::new("alice node");
    /// let bob_node = Node::new("bob node");
    /// let mut graph = Graph::create(&alice_node, "best friends", &bob_node);
    /// assert_eq!(graph.get_from_node().get_label(), "alice node");
    /// assert_eq!(graph.get_to_node().get_label(), "bob node");
    /// alice_node.set_label("alice");
    /// graph.update_from(&alice_node);
    /// assert_eq!(graph.get_from_node().get_label(), "alice");
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
    /// use crate::gruphst::*;
    ///
    ///
    /// let alice_node = Node::new("alice node");
    /// let bob_node = Node::new("bob node");
    /// let mut graph = Graph::create(&alice_node, "best friends", &bob_node);
    /// assert_eq!(graph.get_from_node().get_label(), "alice node");
    /// assert_eq!(graph.get_to_node().get_label(), "bob node");
    /// let fred_node = Node::new("fred node");
    /// graph.update_to(&fred_node);
    /// assert_eq!(graph.get_to_node().get_label(), "fred node");
    /// assert_ne!(graph.get_to_node().get_id(), bob_node.get_id());
    /// ```
    pub fn update_to(&mut self, to_node: &Node) {
        debug!("Updated Graph [{}] to Node: {:#?}", self.id, to_node);
        self.to = to_node.clone();
    }

    pub fn get_from_node(&self) -> Node {
        self.from.clone()
    }

    pub fn get_to_node(&self) -> Node {
        self.to.clone()
    }

    pub fn get_relation(&self) -> String {
        self.relation.clone()
    }

    pub fn set_relation(&mut self, relation_label: &str) {
        self.relation = relation_label.to_string();
    }
}

impl RUDAttribute for Graph {
    fn set_attr<T>(&mut self, key: &str, val: T)
    where
        T: std::fmt::Display,
    {
        self.attr.set_attr(key, val);
    }

    fn get_attr(&self, key: &str) -> Result<&String, &'static str> {
        self.attr.get_attr(key)
    }

    fn update_attr<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), &'static str>
    where
        T: std::fmt::Display,
    {
        self.attr.update_attr(attr_k, attr_v)
    }

    fn upsert_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        self.attr.upsert_attr(attr_k, attr_v)
    }

    fn del_attr(&mut self, v: &str) -> Result<(), &'static str> {
        self.attr.del_attr(v)
    }

    fn get_attr_keys(&self) -> Vec<&str> {
        self.attr.get_attr_keys()
    }
}
