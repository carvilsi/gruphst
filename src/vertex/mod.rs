use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{attributes::Attributes, edge::Edge, CUREdgeVertex, RUDAttribute};

mod query;

/// Representation of a Graph, relating two edges
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    /// A Graph has an uuid
    id: String,
    /// A label for the relation
    relation: String,
    /// Origin edge
    from: Edge,
    /// Target edge
    to: Edge,
    /// Attributes for the Graph
    attr: Attributes,
}

impl CUREdgeVertex for Vertex {
    fn new(label: &str) -> Self {
        Vertex {
            id: Uuid::new_v4().to_string(),
            relation: label.to_string(),
            from: Edge::new(""),
            to: Edge::new(""),
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

impl Vertex {
    /// Adds "From" and "To" edge
    /// to a previous created Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use gruphst::vertex::Vertex;
    /// use crate::gruphst::*;
    ///
    /// let alice = Edge::new("Alice");
    /// let bob = Edge::new("Bob");
    /// let mut alice_bob_graph = Vertex::new("");
    /// alice_bob_graph.add_relation(&alice, "friend of", &bob);
    /// assert_eq!(alice_bob_graph.get_relation(), "friend of");
    /// ```
    pub fn add_relation(&mut self, from: &Edge, relation: &str, to: &Edge) {
        self.relation = String::from(relation);
        self.from = from.clone();
        self.to = to.clone();
        debug!("Added relation to Graph: {:#?}", self);
    }
    /// Creates a Graph,
    /// providing "From" and "To" edges and the "relation"
    /// the id is generated
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use gruphst::vertex::Vertex;
    /// use crate::gruphst::*;
    ///
    /// let alice = Edge::new("Alice");
    /// let bob = Edge::new("Bob");
    /// let alice_bob_graph =
    ///     Vertex::create(&alice, "friend of", &bob);
    /// assert_eq!(alice_bob_graph.get_relation(), "friend of");
    /// ```
    pub fn create(from: &Edge, relation: &str, to: &Edge) -> Self {
        let mut g = Vertex::new(relation);
        g.from = from.clone();
        g.to = to.clone();
        g
    }

    /// Updates the relation for the Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use gruphst::vertex::Vertex;
    /// use crate::gruphst::*;
    ///
    ///
    /// let alice = Edge::new("Alice");
    /// let bob = Edge::new("Bob");
    /// let mut alice_bob_graph = Vertex::create(&alice, "friend of", &bob);
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

    /// Updates the "from" edge in Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use gruphst::vertex::Vertex;
    /// use crate::gruphst::*;
    ///
    ///
    /// let mut alice_edge = Edge::new("alice edge");
    /// let bob_edge = Edge::new("bob edge");
    /// let mut graph = Vertex::create(&alice_edge, "best friends", &bob_edge);
    /// assert_eq!(graph.get_from_edge().get_label(), "alice edge");
    /// assert_eq!(graph.get_to_edge().get_label(), "bob edge");
    /// alice_edge.set_label("alice");
    /// graph.update_from(&alice_edge);
    /// assert_eq!(graph.get_from_edge().get_label(), "alice");
    /// ```
    pub fn update_from(&mut self, from_edge: &Edge) {
        debug!("Updated Graph [{}] from edge: {:#?}", self.id, from_edge);
        self.from = from_edge.clone();
    }

    /// Updates the "to" edge in Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::edge::Edge;
    /// use gruphst::vertex::Vertex;
    /// use crate::gruphst::*;
    ///
    ///
    /// let alice_edge = Edge::new("alice edge");
    /// let bob_edge = Edge::new("bob edge");
    /// let mut graph = Vertex::create(&alice_edge, "best friends", &bob_edge);
    /// assert_eq!(graph.get_from_edge().get_label(), "alice edge");
    /// assert_eq!(graph.get_to_edge().get_label(), "bob edge");
    /// let fred_edge = Edge::new("fred edge");
    /// graph.update_to(&fred_edge);
    /// assert_eq!(graph.get_to_edge().get_label(), "fred edge");
    /// assert_ne!(graph.get_to_edge().get_id(), bob_edge.get_id());
    /// ```
    pub fn update_to(&mut self, to_edge: &Edge) {
        debug!("Updated Graph [{}] to edge: {:#?}", self.id, to_edge);
        self.to = to_edge.clone();
    }

    pub fn get_from_edge(&self) -> Edge {
        self.from.clone()
    }

    pub fn get_to_edge(&self) -> Edge {
        self.to.clone()
    }

    pub fn get_relation(&self) -> String {
        self.relation.clone()
    }

    pub fn set_relation(&mut self, relation_label: &str) {
        self.relation = relation_label.to_string();
    }
}

impl RUDAttribute for Vertex {
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
