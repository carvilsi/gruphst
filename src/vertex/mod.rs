use std::{cell::RefCell, rc::Rc};

use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    attributes::Attributes,
    edge::{Edge, Edge_},
    RUDAttribute,
};

mod query;

/// Representation of a Graph, relating two edges
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    /// A Graph has an uuid
    id: String,
    /// A label for the relation
    relation: String,
    /// Origin edge
    from: Rc<RefCell<Edge_>>,
    /// Target edge
    to: Rc<RefCell<Edge_>>,
    /// Attributes for the Graph
    attr: Attributes,
}

impl Vertex {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_label(&self) -> String {
        self.relation.clone()
    }

    pub fn set_label(&mut self, label: &str) {
        self.relation = label.to_string()
    }

    pub fn get_attributes(&self) -> Attributes {
        self.attr.clone()
    }

    pub fn set_attributes(&mut self, attributes: Attributes) {
        self.attr = attributes;
    }

    /// Creates a new instance
    fn new(label: &str) -> Self {
        Vertex {
            id: Uuid::new_v4().to_string(),
            relation: label.to_string(),
            from: Edge_::new(""),
            to: Edge_::new(""),
            attr: Attributes::new(),
        }
    }

    /// Adds "From" and "To" edge
    /// to a previous created Graph
    pub fn add_relation(&mut self, from: &Edge, relation: &str, to: &Edge) {
        self.from = Rc::clone(&from.edge);
        self.relation = String::from(relation);
        self.to = Rc::clone(&to.edge);
        debug!("Added relation to Graph: {:#?}", self);
    }

    /// Creates a Graph,
    /// providing "From" and "To" edges and the "relation"
    /// the id is generated
    pub fn create(from: &Edge, relation: &str, to: &Edge) -> Self {
        let mut v = Vertex::new(relation);
        v.from = Rc::clone(&from.edge);
        v.to = Rc::clone(&to.edge);
        v
    }

    /// Updates the relation for the Graph
    pub fn update_relation(&mut self, relation: &str) {
        debug!("Updated Graph [{}] with Relation: {}", self.id, relation);
        self.relation = relation.to_string();
    }

    /// Updates the "from" edge in Graph
    pub fn update_from(&mut self, from_edge: &Edge) {
        debug!(
            "Updated Graph [{}] from edge: {:#?}",
            self.id, from_edge.edge
        );
        self.from = Rc::clone(&from_edge.edge);
    }

    /// Updates the "to" edge in Graph
    pub fn update_to(&mut self, to_edge: &Edge) {
        debug!("Updated Graph [{}] to edge: {:#?}", self.id, to_edge.edge);
        self.to = Rc::clone(&to_edge.edge);
    }

    pub fn get_from_edge(&self) -> Edge {
        Edge {
            edge: self.from.clone(),
        }
    }

    pub fn get_to_edge(&self) -> Edge {
        Edge {
            edge: self.to.clone(),
        }
    }

    pub fn get_relation(&self) -> String {
        self.relation.clone()
    }

    pub fn set_relation(&mut self, relation_label: &str) {
        self.relation = relation_label.to_string();
    }
}

impl RUDAttribute for Vertex {
    fn set<T>(&mut self, key: &str, val: T)
    where
        T: std::fmt::Display,
    {
        self.attr.set(key, val);
    }

    fn get(&self, key: &str) -> Result<&String, &'static str> {
        self.attr.get(key)
    }

    fn update<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), &'static str>
    where
        T: std::fmt::Display,
    {
        self.attr.update(attr_k, attr_v)
    }

    fn upsert<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        self.attr.upsert(attr_k, attr_v)
    }

    fn delete(&mut self, v: &str) -> Result<(), &'static str> {
        self.attr.delete(v)
    }

    fn get_keys(&self) -> Vec<&str> {
        self.attr.get_keys()
    }
}
