use std::{cell::RefCell, collections::HashMap, rc::Rc};

use log::warn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::vertex::{Vertex, Vertex_};

mod queries;

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
    attr: HashMap<String, String>,
}

impl Edge {
    /// Creates a new instance
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex};
    ///
    /// let mut edge = Edge::new("");
    /// edge.add_relation(
    ///     &Vertex::new("Frodo"),
    ///     "friend of",
    ///     &Vertex::new("Sam"));
    /// ```
    pub fn new(label: &str) -> Self {
        Edge {
            id: Uuid::new_v4().to_string(),
            relation: label.to_string(),
            from: Vertex::create(""),
            to: Vertex::create(""),
            attr: HashMap::new(),
        }
    }

    /// Retrieves the generated uuid for the edge
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Retrieves the label for the edge
    pub fn get_label(&self) -> String {
        self.relation.clone()
    }

    /// Sets the label for the edge
    pub fn set_label(&mut self, label: &str) {
        self.relation = label.to_string()
    }

    /// Adds "From" and "To" vertices
    /// to a previous created Edge
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex};
    ///
    /// let mut edge = Edge::new("");
    /// edge.add_relation(
    ///     &Vertex::new("Frodo"),
    ///     "friend of",
    ///     &Vertex::new("Sam"));
    /// ```
    pub fn add_relation(&mut self, from: &Vertex, relation: &str, to: &Vertex) {
        self.from = Rc::clone(&from.vrtx);
        self.relation = String::from(relation);
        self.to = Rc::clone(&to.vrtx);
    }

    /// Creates an Edge,
    /// providing "From" and "To" vertices and the "relation"
    /// the id is generated
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex};
    ///
    /// Edge::create(
    ///     &Vertex::new("Theoden"),
    ///     "king of",
    ///     &Vertex::new("Rohan"));
    /// ```
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

    /// Retrieves the "From" or source vertex of edge or the relation
    pub fn get_from_vertex(&self) -> Vertex {
        Vertex {
            vrtx: self.from.clone(),
        }
    }

    /// Retrieves the "To" or target vertex of the edge or relation
    pub fn get_to_vertex(&self) -> Vertex {
        Vertex {
            vrtx: self.to.clone(),
        }
    }

    /// Retrieves the name or label of the relation of the edge
    pub fn get_relation(&self) -> String {
        self.relation.clone()
    }

    /// Sets the name or label of the relation of the edge
    pub fn set_relation(&mut self, relation_label: &str) {
        self.relation = relation_label.to_string();
    }

    /// Set an attribute for a edge
    pub fn set_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        self.attr.insert(attr_k.to_string(), attr_v.to_string());
    }

    /// Get attribute for a edge
    pub fn get_attr(&self, attr_k: &str) -> Result<&String, &'static str> {
        let res = self.attr.get(attr_k);
        match res {
            Some(res) => Ok(res),
            None => {
                warn!("attribute '{}' not found", attr_k);
                Err("attribute not found")
            }
        }
    }

    /// Updates the value of an attribute
    pub fn update_attr<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), &'static str>
    where
        T: std::fmt::Display,
    {
        if let Some(attr) = self.attr.get_mut(attr_k) {
            *attr = attr_v.to_string();
            return Ok(());
        }
        Err("not attribute found to update")
    }

    /// Updates the value of an attribute or creates a new one if attribute key does not exists
    pub fn upsert_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        if let Some(attr) = self.attr.get_mut(attr_k) {
            *attr = attr_v.to_string();
        } else {
            self.attr.insert(attr_k.to_string(), attr_v.to_string());
        }
    }

    /// Deletes an attribute
    pub fn delete_attr(&mut self, v: &str) -> Result<(), &'static str> {
        let res = self.attr.remove(v);
        match res {
            Some(_) => Ok(()),
            None => {
                warn!("attribute {} not found for remove", v);
                Err("attribute not found for remove")
            }
        }
    }

    /// Returns an Array containing all attribute keys
    pub fn get_attr_keys(&self) -> Vec<&str> {
        let mut key_vec = Vec::new();
        for key in self.attr.keys() {
            key_vec.push(key.as_str());
        }
        key_vec
    }
}
