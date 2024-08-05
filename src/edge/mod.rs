use log::debug;
use log::warn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::attributes::Attributes;
use crate::vertex::Vertex;
use crate::CUREdgeVertex;
use crate::RUDAttribute;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod query;

#[derive(Debug, Clone, PartialEq,  Serialize, Deserialize)]
pub struct Edge {
    pub edge: Rc<RefCell<Edge_>>,
}

impl Edge {
    pub fn new(label: &str) -> Self {
        Edge {
            edge: Edge_::new(label),
        }
    }
}

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

impl Edge {
    pub fn get_id(&self) -> String {
        self.edge.borrow().id.clone()
    }

    pub fn get_label(&self) -> String {
        self.edge.borrow().label.clone()
    }

    pub fn set_label(&mut self, label: &str) {
        self.edge.borrow_mut().label = label.to_string();
    }

    // fn get_attributes(&self) -> Attributes {
    //     self.edge.borrow().attr.clone()
    // }

    // fn set_attributes(&mut self, attributes: Attributes) {
    //     self.edge.borrow_mut().attr = attributes;
    // }
}

impl Edge {
    /// Set attributes for a edge
    pub fn set_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        self.edge.borrow_mut().attr.insert(attr_k.to_string(), attr_v.to_string());
        debug!(
            "added attribute key: {} with value {} for edge {}",
            attr_k, attr_v, self.get_id()
        );
    }
    
    /// Get attribute for a edge
    pub fn get_attr(&self, attr_k: &str) -> Result<String, &'static str> {
        let binding = self.edge.borrow();
        let res = binding.attr.get(attr_k);
        match res {
            Some(resp) => {
                debug!(
                    "retrieved attribute value '{}' for '{}' for edge [{}]",
                    resp, attr_k, self.get_id()
                );
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
        debug!(
            "updated attribute key: {} with value {} for edge {}",
            attr_k, attr_v, self.get_id()
        );
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
        match self.edge.borrow_mut().attr.get_mut(attr_k) {
            Some(attr) => {
                *attr = attr_v.to_string();
                debug!(
                    "updated (upsert) attribute key: {} with value {} for edge {}",
                    attr_k, attr_v, self.get_id()
                );
            }
            None => {
                self.edge.borrow_mut().attr.insert(attr_k.to_string(), attr_v.to_string());
                debug!(
                    "added (upsert) attribute key: {} with value {} for edge {}",
                    attr_k, attr_v, self.get_id()
                );
            }
        }
    }

    /// Deletes an attribute
    pub fn del_attr(&mut self, v: &str) -> Result<(), &'static str> {
        let res = self.edge.borrow_mut().attr.remove(v);
        match res {
            Some(_) => {
                debug!("Removed '{}' attribute for {}", v, self.get_id());
                Ok(())
            }
            None => {
                warn!("attribute {} not found for remove", v);
                Err("attribute not found for remove")
            }
        }
    }

    // /// Returns an Array containing all attribute keys
    // fn get_attr_keys(&self) -> Vec<&str> {
    //     let mut key_vec = Vec::new();
    //     for key in self.edge.borrow().attr.keys() {
    //         key_vec.push(key.as_str());
    //     }
    //     debug!(
    //         "requested array of attributes for {} edge {:#?}",
    //         self.get_id(), key_vec
    //     );
    //     key_vec
    // }
    
    // fn set_attr<T>(&mut self, key: &str, val: T)
    // where
    //     T: std::fmt::Display,
    // {
    //     self.edge.borrow_mut().attr.set_attr(key, val);
    // }

    // // fn get_attr(&self, key: &str) -> Result<&String, &'static str> {
    // //     self.edge.borrow().attr.get_attr(key).clone()
    // // }

    // fn update_attr<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), &'static str>
    // where
    //     T: std::fmt::Display,
    // {
    //     self.edge.borrow_mut().attr.update_attr(attr_k, attr_v)
    // }

    // fn upsert_attr<T>(&mut self, attr_k: &str, attr_v: T)
    // where
    //     T: std::fmt::Display,
    // {
    //     self.edge.borrow_mut().attr.upsert_attr(attr_k, attr_v)
    // }

    // fn del_attr(&mut self, v: &str) -> Result<(), &'static str> {
    //     self.edge.borrow_mut().attr.del_attr(v)
    // }

    // fn get_attr_keys(&self) -> Vec<&str> {
    //     // self.edge.borrow_mut().attr.get_attr_keys()
    //     vec!["foo", "bar"]
    // }
}

impl Edge_ {
    /// Creates a edge with the given label, the id is generated
    pub fn new(label: &str) -> Rc<RefCell<Edge_>> {
        let edge = Edge_ {
            label: String::from(label),
            id: Uuid::new_v4().to_string(),
            attr: HashMap::new(),
        };
        debug!("The created edge: {:#?}", &edge);
        Rc::new(RefCell::new(edge))
    }

    // Retrieves the edges that has relation out for the given edge on graph
    // pub fn get_relations_out_on_graph(
        // &self,
        // graph: Vec<Vertex>,
    // ) -> Result<HashMap<String, Vec<Rc<RefCell<Edge_>>>>, &'static str> {
        // let mut relations_out: HashMap<String, Vec<Rc<RefCell<Edge_>>>> = HashMap::new();
        // for vertex in graph {
            // if vertex.get_from_edge().borrow().get_id() == self.id {
                // if let Some(edges_out) = relations_out.get_mut(&vertex.get_relation()) {
                    // edges_out.push(vertex.get_to_edge());
                // } else {
                    // let edges_out = vec![vertex.get_to_edge()];
                    // relations_out.insert(vertex.get_relation(), edges_out);
                // }
            // }
        // }
        // if !relations_out.is_empty() {
            // Ok(relations_out)
        // } else {
            // Err("no relations out for edge")
        // }
    // }
// 
    // Retrieves the edges that has relation in for the given edge on graph
    // pub fn get_relations_in_on_graph(
        // &self,
        // graphs: Vec<Vertex>,
    // ) -> Result<HashMap<String, Vec<Rc<RefCell<Edge_>>>>, &'static str> {
        // let mut relations_in: HashMap<String, Vec<Rc<RefCell<Edge_>>>> = HashMap::new();
        // for graph in graphs {
            // if graph.get_to_edge().borrow().get_id() == self.id {
                // if let Some(edges_in) = relations_in.get_mut(&graph.get_relation()) {
                    // edges_in.push(graph.get_from_edge());
                // } else {
                    // let edges_in = vec![graph.get_from_edge()];
                    // relations_in.insert(graph.get_relation(), edges_in);
                // }
            // }
        // }
        // if !relations_in.is_empty() {
            // Ok(relations_in)
        // } else {
            // Err("no relations in for edge")
        // }
    // }
}
