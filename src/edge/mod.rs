use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    attributes::Attributes,
    vertex::{Vertex, Vertex_},
    RUDAttribute,
};

mod query;

/// Representation of a Edge, that consists on the relation of two vertices
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// A Edge has an uuid
    id: String,
    /// A label for the relation
    relation: String,
    /// Origin vertex
    from: Rc<RefCell<Vertex_>>,
    /// Target vertex
    to: Rc<RefCell<Vertex_>>,
    /// Attributes for the Edge
    attr: Attributes,
}

impl Edge {
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
        Edge {
            id: Uuid::new_v4().to_string(),
            relation: label.to_string(),
            from: Vertex::create(""),
            to: Vertex::create(""),
            attr: Attributes::new(),
        }
    }

    /// Adds "From" and "To" vertices
    /// to a previous created Edge
    pub fn add_relation(&mut self, from: &Vertex, relation: &str, to: &Vertex) {
        self.from = Rc::clone(&from.vrtx);
        self.relation = String::from(relation);
        self.to = Rc::clone(&to.vrtx);
    }

    /// Creates an Edge,
    /// providing "From" and "To" edges and the "relation"
    /// the id is generated
    pub fn create(from: &Vertex, relation: &str, to: &Vertex) -> Self {
        let mut v = Edge::new(relation);
        v.from = Rc::clone(&from.vrtx);
        v.to = Rc::clone(&to.vrtx);
        v
    }

    /// Updates the relation for the Edge
    pub fn update_relation(&mut self, relation: &str) {
        self.relation = relation.to_string();
    }

    /// Updates the "from" or source edge in Edge
    pub fn update_from(&mut self, from_vertex: &Vertex) {
        self.from = Rc::clone(&from_vertex.vrtx);
    }

    /// Updates the "to" or target edge in Edge
    pub fn update_to(&mut self, to_vertex: &Vertex) {
        self.to = Rc::clone(&to_vertex.vrtx);
    }

    pub fn get_from_vertex(&self) -> Vertex {
        Vertex {
            vrtx: self.from.clone(),
        }
    }

    pub fn get_to_vertex(&self) -> Vertex {
        Vertex {
            vrtx: self.to.clone(),
        }
    }

    pub fn get_relation(&self) -> String {
        self.relation.clone()
    }

    pub fn set_relation(&mut self, relation_label: &str) {
        self.relation = relation_label.to_string();
    }
}

impl RUDAttribute for Edge {
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
