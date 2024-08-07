use log::warn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::vertex::Vertex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod query;

// TODO: this should be private
/// Representation of a edge
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge_ {
    /// A edge id is an uuid as identifier
    id: String,
    /// And a name
    label: String,
    /// The attributes for a edge
    attr: HashMap<String, String>,
}

impl Edge_ {
    /// Creates a edge with the given label, the id is generated
    fn new(label: &str) -> Rc<RefCell<Edge_>> {
        let edge = Edge_ {
            label: String::from(label),
            id: Uuid::new_v4().to_string(),
            attr: HashMap::new(),
        };
        Rc::new(RefCell::new(edge))
    }
}

// wrapper for Edge_
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub edge: Rc<RefCell<Edge_>>,
}

impl Edge {
    pub fn new(label: &str) -> Self {
        Edge {
            edge: Edge_::new(label),
        }
    }
    
    pub fn create(label: &str) -> Rc<RefCell<Edge_>> {
        Edge_::new(label)
    }

    pub fn get_id(&self) -> String {
        self.edge.borrow().id.clone()
    }

    pub fn get_label(&self) -> String {
        self.edge.borrow().label.clone()
    }

    pub fn set_label(&mut self, label: &str) {
        self.edge.borrow_mut().label = label.to_string();
    }

    /// Set attributes for a edge
    pub fn set_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        self.edge
            .borrow_mut()
            .attr
            .insert(attr_k.to_string(), attr_v.to_string());
    }

    /// Get attribute for a edge
    pub fn get_attr(&self, attr_k: &str) -> Result<String, &'static str> {
        let binding = self.edge.borrow();
        let res = binding.attr.get(attr_k);
        match res {
            Some(resp) => {
                Ok(resp.clone())
            }
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
        if let Some(attr) = self.edge.borrow_mut().attr.get_mut(attr_k) {
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
        let mut binding = self.edge.borrow_mut();
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
        let res = self.edge.borrow_mut().attr.remove(v);
        match res {
            Some(_) => {
                Ok(())
            }
            None => {
                warn!("attribute {} not found for remove", v);
                Err("attribute not found for remove")
            }
        }
    }

    /// Returns an Array containing all attribute keys
    pub fn get_attr_keys(&self) -> Vec<String> {
        let binding = self.edge.borrow();
        // FIXME: clippy has a warning here:
        // let kv: Vec<String> = binding.attr.iter().map(|(k,_v)| k.clone()).collect();
        // |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `binding.attr.keys().map(|k| k.clone())`

        let kv: Vec<String> = binding.attr.iter().map(|(k, _v)| k.clone()).collect();
        kv
    }

    /// Retrieves the edges that has relation out for the given edge on vertices 
    pub fn get_relations_out_on_vertices(
        &self,
        vertices: Vec<Vertex>,
    ) -> Result<HashMap<String, Vec<Edge>>, &'static str> {
        let mut relations_out: HashMap<String, Vec<Edge>> = HashMap::new();
        for vertex in vertices {
            if vertex.get_from_edge().get_id() == self.get_id() {
                if let Some(edges_out) = relations_out.get_mut(&vertex.get_relation()) {
                    edges_out.push(vertex.get_to_edge());
                } else {
                    let edges_out = vec![vertex.get_to_edge()];
                    relations_out.insert(vertex.get_relation(), edges_out);
                }
            }
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            Err("no relations out for edge")
        }
    }

    /// Retrieves the edges that has relation in for the given edge on vertices 
    pub fn get_relations_in_on_vertices(
        &self,
        vertices: Vec<Vertex>,
    ) -> Result<HashMap<String, Vec<Edge>>, &'static str> {
        let mut relations_in: HashMap<String, Vec<Edge>> = HashMap::new();
        for graph in vertices {
            if graph.get_to_edge().get_id() == self.get_id() {
                if let Some(edges_in) = relations_in.get_mut(&graph.get_relation()) {
                    edges_in.push(graph.get_from_edge());
                } else {
                    let edges_in = vec![graph.get_from_edge()];
                    relations_in.insert(graph.get_relation(), edges_in);
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
