use std::cell::RefCell;
use std::rc::Rc;

use log::{debug, error};

use crate::vertex::Vertex;
use crate::graphs::Graphs;
use crate::edge::Edge;
use crate::CUREdgeVertex;

impl Graphs {
    /// Returns a collection of Graps elements that matches the relation
    pub fn find_by_relation(
        &mut self,
        relation_name: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.get_relation() == relation_name)
                .collect::<Vec<&Vertex>>();
            if !graphs.is_empty() {
                debug!(
                    "Founded {} graphs with '{}' relation name",
                    graphs.len(),
                    relation_name
                );
                Ok(graphs)
            } else {
                error!("Any graph found for relation: {}", relation_name);
                Err("Any graph found for relation")
            }
        } else {
            Err("no graphs found on vault")
        }
    }

    /// Returns a collection of Graps elements that matches the relations
    /// in the array
    pub fn find_by_relations(
        &mut self,
        relations: Vec<&str>,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| relations.contains(&grph.get_relation().as_str()))
                .collect::<Vec<&Vertex>>();
            if !graphs.is_empty() {
                debug!(
                    "Founded {} graphs with '{:#?}' relations",
                    graphs.len(),
                    relations
                );
                Ok(graphs)
            } else {
                error!("Any graph found for relations: {:#?}", relations);
                Err("Any graph found for relation")
            }
        } else {
            Err("graphs not found on vault")
        }
    }

    /// Returns a collection of graphs that matches an attribute edge by key
    pub fn has_graph_edge_attr(
        &mut self,
        attr_k: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.has_edge_attr(attr_k))
                .collect::<Vec<&Vertex>>();
            if !graphs.is_empty() {
                debug!(
                    "Founded {} graphs where an attribute key is '{}'",
                    graphs.len(),
                    attr_k
                );
                Ok(graphs)
            } else {
                error!("Any graph found for attribute: {}", attr_k);
                Err("Any graph found for attribute")
            }
        } else {
            Err("no graphs found on vault")
        }
    }

    /// Returns a collection of graphs like an attribute edge by key
    pub fn like_graph_edge_attr(
        &mut self,
        attr_k: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.like_edge_attr(attr_k))
                .collect::<Vec<&Vertex>>();
            if !graphs.is_empty() {
                debug!(
                    "Founded {} graphs where an attribute key is '{}'",
                    graphs.len(),
                    attr_k
                );
                Ok(graphs)
            } else {
                error!("Any graph found for attribute: {}", attr_k);
                Err("Any graph found for attribute")
            }
        } else {
            Err("no graphs on vault")
        }
    }

    /// Returns a collection of graphs that matches an attribute
    /// and value
    // XXX: add a method to find attr on all graphs????
    pub fn attr_equals_to<T>(
        &self,
        attr_k: &str,
        attr_v: T,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str>
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.equals_edge_attr(attr_k, attr_v.clone()))
                .collect::<Vec<&Vertex>>();
            if !graphs.is_empty() {
                debug!(
                    "Founded {} graphs where an attribute key is '{}'",
                    graphs.len(),
                    attr_k
                );
                Ok(graphs)
            } else {
                error!("Any graph found for attribute: {}", attr_k);
                Err("Any graph found for attribute")
            }
        } else {
            Err("no graphs on vault")
        }
    }

    /// Returns a Vertex that provided id matches with Vertex, or From, To edges
    pub fn find_by_id(
        &mut self,
        id: &str,
        graphs_name: Option<&str>,
    ) -> Result<&mut Vertex, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get_mut(&current_graph) {
            let graph = graphs.iter_mut().find(|graph| {
                graph.get_id() == id
                    || graph.get_from_edge().borrow().get_id() == id
                    || graph.get_to_edge().borrow().get_id() == id
            });
            if graph.is_some() {
                debug!("Founded Vertex by id: {:#?}", graph);
                Ok(graph.unwrap())
            } else {
                error!("Vertex with id [{}] not found", id);
                Err("Vertex not found")
            }
        } else {
            Err("no graphs found at vault")
        }
    }

    /// Find in any graph on vault by id
    pub fn find_by_id_in_graphs(&mut self, id: &str) -> Result<&mut Vertex, &'static str> {
        for (_graph_name, graphs) in self.vault.iter_mut() {
            println!("Tha name: {}", _graph_name);
            let graph = graphs.iter_mut().find(|graph| {
                graph.get_id() == id
                    || graph.get_from_edge().borrow().get_id() == id
                    || graph.get_to_edge().borrow().get_id() == id
            });
            if graph.is_some() {
                debug!("Founded Vertex by id: {:#?}", graph);
                return Ok(graph.unwrap());
            }
        }
        Err("Vertex not found")
    }

    /// Retrieves all the edges with incoming relation
    pub fn has_relation_in(
        &self,
        relation_in: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<Rc<RefCell<Edge>>>, &'static str> {
        let mut relations_in: Vec<Rc<RefCell<Edge>>> = Vec::new();
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            for graph in graphs {
                if graph.get_relation() == relation_in
                    && !relations_in.contains(&graph.get_to_edge())
                {
                    relations_in.push(graph.get_to_edge().clone());
                }
            }
        } else {
            return Err("no current graph in vault");
        }
        if !relations_in.is_empty() {
            Ok(relations_in)
        } else {
            Err("any edge with relation in")
        }
    }

    /// Retrieves all the edges with outcoming relation
    pub fn has_relation_out(
        &self,
        relation_out: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<Rc<RefCell<Edge>>>, &'static str> {
        let mut relations_out: Vec<Rc<RefCell<Edge>>> = Vec::new();
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            for graph in graphs {
                if graph.get_relation() == relation_out
                    && !relations_out.contains(&graph.get_from_edge())
                {
                    relations_out.push(graph.get_from_edge().clone());
                }
            }
        } else {
            return Err("no current graph in vault");
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            Err("any edge with relation out")
        }
    }
}
