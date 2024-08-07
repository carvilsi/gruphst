use log::warn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::edge::Edge;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod query;

// TODO: this should be private
/// Representation of a vertex
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex_ {
    /// a vertex id is an uuid as identifier
    id: String,
    /// And a name
    label: String,
    /// The attributes for a vertex
    attr: HashMap<String, String>,
}

impl Vertex_ {
    /// Creates a vertex with the given label, the id is generated
    fn new(label: &str) -> Rc<RefCell<Vertex_>> {
        let edge = Vertex_ {
            label: String::from(label),
            id: Uuid::new_v4().to_string(),
            attr: HashMap::new(),
        };
        Rc::new(RefCell::new(edge))
    }
}

// wrapper for Edge_
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    pub vrtx: Rc<RefCell<Vertex_>>,
}

impl Vertex {
    pub fn new(label: &str) -> Self {
        Vertex {
            vrtx: Vertex_::new(label),
        }
    }

    pub fn create(label: &str) -> Rc<RefCell<Vertex_>> {
        Vertex_::new(label)
    }

    pub fn get_id(&self) -> String {
        self.vrtx.borrow().id.clone()
    }

    pub fn get_label(&self) -> String {
        self.vrtx.borrow().label.clone()
    }

    pub fn set_label(&mut self, label: &str) {
        self.vrtx.borrow_mut().label = label.to_string();
    }

    /// Set attributes for a vertex
    pub fn set_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        self.vrtx
            .borrow_mut()
            .attr
            .insert(attr_k.to_string(), attr_v.to_string());
    }

    /// Get attribute for a vertex
    pub fn get_attr(&self, attr_k: &str) -> Result<String, &'static str> {
        let binding = self.vrtx.borrow();
        let res = binding.attr.get(attr_k);
        match res {
            Some(resp) => Ok(resp.clone()),
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
        if let Some(attr) = self.vrtx.borrow_mut().attr.get_mut(attr_k) {
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
        let mut binding = self.vrtx.borrow_mut();
        let attr = binding.attr.get_mut(attr_k);
        match attr {
            Some(attr) => {
                *attr = attr_v.to_string();
            }
            None => {
                binding.attr.insert(attr_k.to_string(), attr_v.to_string());
            }
        }
    }

    /// Deletes an attribute
    pub fn del_attr(&mut self, v: &str) -> Result<(), &'static str> {
        let res = self.vrtx.borrow_mut().attr.remove(v);
        match res {
            Some(_) => Ok(()),
            None => {
                warn!("attribute {} not found for remove", v);
                Err("attribute not found for remove")
            }
        }
    }

    /// Returns an collection containing all attribute keys
    pub fn get_attr_keys(&self) -> Vec<String> {
        let binding = self.vrtx.borrow();
        // FIXME: clippy has a warning here:
        // let kv: Vec<String> = binding.attr.iter().map(|(k,_v)| k.clone()).collect();
        // |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `binding.attr.keys().map(|k| k.clone())`

        let kv: Vec<String> = binding.attr.iter().map(|(k, _v)| k.clone()).collect();
        kv
    }

    /// Retrieves the vertices that has relation out for the given vertex on a collection of edges
    pub fn get_relations_out_on_edges(
        &self,
        edges: Vec<Edge>,
    ) -> Result<HashMap<String, Vec<Vertex>>, &'static str> {
        let mut relations_out: HashMap<String, Vec<Vertex>> = HashMap::new();
        for edge in edges {
            if edge.get_from_vertex().get_id() == self.get_id() {
                if let Some(vertices_out) = relations_out.get_mut(&edge.get_relation()) {
                    vertices_out.push(edge.get_to_vertex());
                } else {
                    let vertices_out = vec![edge.get_to_vertex()];
                    relations_out.insert(edge.get_relation(), vertices_out);
                }
            }
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            Err("no relations out for vertex")
        }
    }

    /// Retrieves the vertices that has relation in for the given vertex on edges
    pub fn get_relations_in_on_edges(
        &self,
        edges: Vec<Edge>,
    ) -> Result<HashMap<String, Vec<Vertex>>, &'static str> {
        let mut relations_in: HashMap<String, Vec<Vertex>> = HashMap::new();
        for edge in edges {
            if edge.get_to_vertex().get_id() == self.get_id() {
                if let Some(vertices_in) = relations_in.get_mut(&edge.get_relation()) {
                    vertices_in.push(edge.get_from_vertex());
                } else {
                    let vertices_in = vec![edge.get_from_vertex()];
                    relations_in.insert(edge.get_relation(), vertices_in);
                }
            }
        }
        if !relations_in.is_empty() {
            Ok(relations_in)
        } else {
            Err("no relations in for edge")
        }
    }
}
