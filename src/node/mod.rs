use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::attributes::Attributes;
use crate::CURNodeGraph;
use crate::RUDAttribute;
use crate::graph::Graph;
use std::collections::HashMap;

mod query;

/// Representation of a Node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// A Node id is an uuid as identifier
    id: String,
    /// And a name
    label: String,
    /// The attributes for a node
    attr: Attributes,
}

impl CURNodeGraph for Node {
    /// Creates a Node with the given label, the id is generated
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use crate::gruphst::*;
    ///
    /// let node = Node::new("alice node");
    /// ```
    fn new(label: &str) -> Self {
        let node = Node {
            label: String::from(label),
            id: Uuid::new_v4().to_string(),
            attr: Attributes::new(),
        };
        debug!("The created node: {:#?}", &node);
        node
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_label(&self) -> String {
        self.label.clone()
    }

    fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    fn get_attributes(&self) -> Attributes {
        self.attr.clone()
    }

    fn set_attributes(&mut self, attributes: Attributes) {
        self.attr = attributes;
    }
}

impl RUDAttribute for Node {
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

impl Node {
    /// Retrieves the nodes that has relation out for the given node on graph
    pub fn get_relations_out_on_graph(&self, graphs: Vec<Graph>) -> Result<HashMap<String, Vec<Node>>, &'static str> {
        let mut relations_out: HashMap<String, Vec<Node>> = HashMap::new();
        for graph in graphs {
            if graph.get_from_node().get_id() == self.id {
                if let Some(nodes_out) = relations_out.get_mut(&graph.get_relation()) {
                    nodes_out.push(graph.get_to_node());
                } else {
                    let mut nodes_out = Vec::new();
                    nodes_out.push(graph.get_to_node());
                    relations_out.insert(graph.get_relation(), nodes_out);
                }
            }
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            Err("no relations out for node")
        }
    }
    
    /// Retrieves the nodes that has relation in for the given node on graph
    pub fn get_relations_in_on_graph(&self, graphs: Vec<Graph>) -> Result<HashMap<String, Vec<Node>>, &'static str> {
        let mut relations_in: HashMap<String, Vec<Node>> = HashMap::new();
        for graph in graphs {
            if graph.get_to_node().get_id() == self.id {
                if let Some(nodes_in) = relations_in.get_mut(&graph.get_relation()) {
                    nodes_in.push(graph.get_from_node());
                } else {
                    let mut nodes_in = Vec::new();
                    nodes_in.push(graph.get_from_node());
                    relations_in.insert(graph.get_relation(), nodes_in);
                }
            }
        }
        if !relations_in.is_empty() {
            Ok(relations_in)
        } else {
            Err("no relations in for node")
        }
    }
}