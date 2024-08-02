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
    pub fn add_relation(&mut self, from: &Edge, relation: &str, to: &Edge) {
        self.relation = String::from(relation);
        self.from = from.clone();
        self.to = to.clone();
        debug!("Added relation to Graph: {:#?}", self);
    }
    /// Creates a Graph,
    /// providing "From" and "To" edges and the "relation"
    /// the id is generated
    pub fn create(from: &Edge, relation: &str, to: &Edge) -> Self {
        let mut g = Vertex::new(relation);
        g.from = from.clone();
        g.to = to.clone();
        g
    }

    /// Updates the relation for the Graph
    pub fn update_relation(&mut self, relation: &str) {
        debug!("Updated Graph [{}] with Relation: {}", self.id, relation);
        self.relation = relation.to_string();
    }

    /// Updates the "from" edge in Graph
    pub fn update_from(&mut self, from_edge: &Edge) {
        debug!("Updated Graph [{}] from edge: {:#?}", self.id, from_edge);
        self.from = from_edge.clone();
    }

    /// Updates the "to" edge in Graph
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
