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

/// Representation of a Vertex, relating two edges
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    /// A Vertex has an uuid
    id: String,
    /// A label for the relation
    relation: String,
    /// Origin edge
    from: Rc<RefCell<Edge_>>,
    /// Target edge
    to: Rc<RefCell<Edge_>>,
    /// Attributes for the Vertex
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
    pub fn new(label: &str) -> Self {
        Vertex {
            id: Uuid::new_v4().to_string(),
            relation: label.to_string(),
            from: Edge::create(""),
            to: Edge::create(""),
            attr: Attributes::new(),
        }
    }

    /// Adds "From" and "To" edge
    /// to a previous created Vertex 
    pub fn add_relation(&mut self, from: &Edge, relation: &str, to: &Edge) {
        self.from = Rc::clone(&from.edge);
        self.relation = String::from(relation);
        self.to = Rc::clone(&to.edge);
        debug!("Added relation to Vertex: {:#?}", self);
    }

    /// Creates a Vertex,
    /// providing "From" and "To" edges and the "relation"
    /// the id is generated
    pub fn create(from: &Edge, relation: &str, to: &Edge) -> Self {
        let mut v = Vertex::new(relation);
        v.from = Rc::clone(&from.edge);
        v.to = Rc::clone(&to.edge);
        v
    }

    /// Updates the relation for the Vertex
    pub fn update_relation(&mut self, relation: &str) {
        debug!("Updated Vertex [{}] with Relation: {}", self.id, relation);
        self.relation = relation.to_string();
    }

    /// Updates the "from" or source edge in Vertex 
    pub fn update_from(&mut self, from_edge: &Edge) {
        debug!(
            "Updated Vertex [{}] from edge: {:#?}",
            self.id, from_edge.edge
        );
        self.from = Rc::clone(&from_edge.edge);
    }

    /// Updates the "to" or target edge in Vertex 
    pub fn update_to(&mut self, to_edge: &Edge) {
        debug!("Updated Vertex [{}] to edge: {:#?}", self.id, to_edge.edge);
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

    fn delete_attr(&mut self, v: &str) -> Result<(), &'static str> {
        self.attr.delete_attr(v)
    }

    fn get_attr_keys(&self) -> Vec<&str> {
        self.attr.get_attr_keys()
    }
}
