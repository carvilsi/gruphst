use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::attributes::Attributes;
use crate::vertex::Vertex;
use crate::CUREdgeVertex;
use crate::RUDAttribute;
use std::collections::HashMap;

mod query;

/// Representation of a edge
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// A edge id is an uuid as identifier
    id: String,
    /// And a name
    label: String,
    /// The attributes for a edge
    attr: Attributes,
}

impl CUREdgeVertex for Edge {
    /// Creates a edge with the given label, the id is generated
    fn new(label: &str) -> Self {
        let edge = Edge {
            label: String::from(label),
            id: Uuid::new_v4().to_string(),
            attr: Attributes::new(),
        };
        debug!("The created edge: {:#?}", &edge);
        edge
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

    fn del_attr(&mut self, v: &str) -> Result<(), &'static str> {
        self.attr.del_attr(v)
    }

    fn get_attr_keys(&self) -> Vec<&str> {
        self.attr.get_attr_keys()
    }
}

impl Edge {
    /// Retrieves the edges that has relation out for the given edge on graph
    pub fn get_relations_out_on_graph(
        &self,
        graphs: Vec<Vertex>,
    ) -> Result<HashMap<String, Vec<Edge>>, &'static str> {
        let mut relations_out: HashMap<String, Vec<Edge>> = HashMap::new();
        for graph in graphs {
            if graph.get_from_edge().get_id() == self.id {
                if let Some(edges_out) = relations_out.get_mut(&graph.get_relation()) {
                    edges_out.push(graph.get_to_edge());
                } else {
                    let edges_out = vec![graph.get_to_edge()];
                    relations_out.insert(graph.get_relation(), edges_out);
                }
            }
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            Err("no relations out for edge")
        }
    }

    /// Retrieves the edges that has relation in for the given edge on graph
    pub fn get_relations_in_on_graph(
        &self,
        graphs: Vec<Vertex>,
    ) -> Result<HashMap<String, Vec<Edge>>, &'static str> {
        let mut relations_in: HashMap<String, Vec<Edge>> = HashMap::new();
        for graph in graphs {
            if graph.get_to_edge().get_id() == self.id {
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
