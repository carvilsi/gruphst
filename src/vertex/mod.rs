use log::warn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::edge::Edge;
use crate::errors::GruPHstError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

mod query;
mod cryptography;

/// Representation of a vertex.
/// A vertex or node, vertices in plural,
/// is the fundamental unit of a graph.
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Vertex_ {
    /// a vertex id is an uuid as identifier
    id: String,
    /// And a name
    label: String,
    /// The attributes for a vertex
    attr: HashMap<String, String>,
    attr_vec_u8: HashMap<String, Vec<u8>>,
}

impl Vertex_ {
    /// Creates a vertex with the given label, the id is generated
    fn new(label: &str) -> Rc<RefCell<Vertex_>> {
        let edge = Vertex_ {
            label: String::from(label),
            id: Uuid::new_v4().to_string(),
            attr: HashMap::new(),
            attr_vec_u8: HashMap::new(),
        };
        Rc::new(RefCell::new(edge))
    }
}

/// Representation of a vertex.
/// A vertex or node, vertices in plural,
/// is the fundamental unit of a graph.
// wrapper for Edge_
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    pub(crate) vrtx: Rc<RefCell<Vertex_>>,
}

impl Vertex {
    /// Creates a vertex with the given label, the id is generated
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// Vertex::new("Gandalf");
    /// ```
    pub fn new(label: &str) -> Self {
        Vertex {
            vrtx: Vertex_::new(label),
        }
    }

    pub(crate) fn create(label: &str) -> Rc<RefCell<Vertex_>> {
        Vertex_::new(label)
    }

    /// Retrieves the generated uuid for a vertex
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let vertex = Vertex::new("Gandalf");
    /// let _id: String = vertex.get_id();
    /// ```
    pub fn get_id(&self) -> String {
        self.vrtx.borrow().id.clone()
    }
    
    /// Retrieves the label for a vertex
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let vertex = Vertex::new("Gandalf");
    /// let label: String = vertex.get_label();
    /// assert_eq!(label, "Gandalf");
    /// ```
    pub fn get_label(&self) -> String {
        self.vrtx.borrow().label.clone()
    }
 
    /// Sets the label for a vertex
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut vertex = Vertex::new("Gandalf");
    /// let mut label: String = vertex.get_label();
    /// assert_eq!(label, "Gandalf");
    /// vertex.set_label("Gandalf the Gray");
    /// label = vertex.get_label();
    /// assert_eq!(label, "Gandalf the Gray");
    /// ```
    pub fn set_label(&mut self, label: &str) {
        self.vrtx.borrow_mut().label = label.to_string();
    }

    /// Set attributes for a vertex
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut gandalf = Vertex::new("Gandalf");
    /// gandalf.set_attr("known as", "The Gray");
    /// gandalf.set_attr("years old", 24000);
    /// ```
    pub fn set_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where 
        T: std::fmt::Display,
    {
        self.vrtx.borrow_mut().attr.insert(attr_k.to_string(), attr_v.to_string());
    }

    pub fn set_attr_vec_u8(&mut self, attr_k: &str, attr_v: &[u8]) {
        self.vrtx.borrow_mut().attr_vec_u8.insert(attr_k.to_string(), attr_v.to_owned());
    }

    /// Get attribute for a vertex
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut gandalf = Vertex::new("Gandalf");
    /// gandalf.set_attr("known as", "The Gray");
    /// gandalf.set_attr("years old", 24000);
    ///
    /// let gandalf_years = gandalf.get_attr("years old").unwrap();
    /// assert_eq!(gandalf_years, "24000");
    /// ```
    pub fn get_attr(&self, attr_k: &str) -> Result<String, GruPHstError> {
        let binding = self.vrtx.borrow();
        let res = binding.attr.get(attr_k);
        match res {
            Some(resp) => Ok(resp.clone()),
            None => {
                warn!("attribute '{}' not found", attr_k);
                Err(GruPHstError::AttributeNotFound)
            }
        }
    }

    /// Get attribute of type Vev<u8>
    pub fn get_attr_vec_u8(&self, attr_k: &str) -> Result<Vec<u8>, GruPHstError> {
        let binding = self.vrtx.borrow();
        let res = binding.attr_vec_u8.get(attr_k);
        match res {
            Some(resp) => Ok(resp.clone()),
            None => {
                warn!("attribute '{}' not found", attr_k);
                Err(GruPHstError::AttributeNotFound)
            }
        }
    }

    /// Updates the value of an attribute
    /// # Examples
    /// ```rust
    /// use gruphst::vertex::Vertex;
    ///
    /// let mut gandalf = Vertex::new("Gandalf");
    /// gandalf.set_attr("known as", "The Gray");
    /// gandalf.set_attr("years old", 24000);
    ///
    /// let mut gandalf_years = gandalf.get_attr("years old").unwrap();
    /// assert_eq!(gandalf_years, "24000");
    ///
    /// gandalf.update_attr("years old", 24001);
    /// gandalf_years = gandalf.get_attr("years old").unwrap();
    /// assert_eq!(gandalf_years, "24001");
    /// ```
    pub fn update_attr<T>(&mut self, attr_k: &str, attr_v: T) -> Result<(), GruPHstError>
    where
        T: std::fmt::Display,
    {
        if let Some(attr) = self.vrtx.borrow_mut().attr.get_mut(attr_k) {
            *attr = attr_v.to_string();
            return Ok(());
        }
        Err(GruPHstError::AttributeNotFound)
    }

    /// Updates the value of an attribute or creates a new one if attribute key does not exists
    pub fn upsert_attr<T>(&mut self, attr_k: &str, attr_v: T)
    where
        T: std::fmt::Display,
    {
        let mut binding = self.vrtx.borrow_mut();
        if let Some(attr) = binding.attr.get_mut(attr_k) {
            *attr = attr_v.to_string();
        } else {
            binding.attr.insert(attr_k.to_string(), attr_v.to_string());
        }
    }

    /// Deletes an attribute
    pub fn del_attr(&mut self, v: &str) -> Result<(), GruPHstError> {
        let res = self.vrtx.borrow_mut().attr.remove(v);
        match res {
            Some(_) => Ok(()),
            None => {
                warn!("attribute {} not found for remove", v);
                Err(GruPHstError::AttributeNotFound)
            }
        }
    }

    /// Returns an collection containing all attribute keys
    /// of String attributes
    pub fn get_attr_str_keys(&self) -> Vec<String> {
        let binding = self.vrtx.borrow();
        // FIXME: clippy has a warning here:
        // let kv: Vec<String> = binding.attr.iter().map(|(k,_v)| k.clone()).collect();
        // |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `binding.attr.keys().map(|k| k.clone())`

        let kv: Vec<String> = binding.attr.iter().map(|(k, _v)| k.clone()).collect();
        kv
    }

    /// Returns an collection containing all attribute keys
    /// of Vec<u8> attributes
    pub fn get_attr_vec_u8_keys(&self) -> Vec<String> {
        let binding = self.vrtx.borrow();
        let kv: Vec<String> = binding.attr_vec_u8.iter().map(|(k, _v)| k.clone()).collect();
        kv
    }

    /// Returns an collection containing all attribute keys
    /// of any type of attributes
    pub fn get_attr_keys(&self) -> Vec<String> {
        let binding = self.vrtx.borrow();
        let mut kv_attr: Vec<String> = binding.attr.iter().map(|(k, _v)| k.clone()).collect();
        let mut kv_attr_vec_u8: Vec<String> = binding.attr_vec_u8.iter().map(|(k, _v)| k.clone()).collect();
        kv_attr.append(&mut kv_attr_vec_u8);
        kv_attr 
    }

    /// Retrieves the vertices that has relation out for the given vertex on a collection of edges
    pub fn get_relations_out_on_edges(
        &self,
        edges: Vec<Edge>,
    ) -> Result<HashMap<String, Vec<Vertex>>, GruPHstError> {
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
            Err(GruPHstError::EdgeNoRelations(String::from("out")))
        }
    }

    /// Retrieves the vertices that has relation in for the given vertex on edges
    pub fn get_relations_in_on_edges(
        &self,
        edges: Vec<Edge>,
    ) -> Result<HashMap<String, Vec<Vertex>>, GruPHstError> {
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
            Err(GruPHstError::EdgeNoRelations(String::from("in")))
        }
    }
}
