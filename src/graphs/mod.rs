use log::{debug, error};
use serde::{Deserialize, Serialize};
use stats::GraphsStats;
use std::collections::HashMap;

use crate::{vertex::Vertex, edge::Edge, util::graphs_memory_watcher, CUREdgeVertex};

mod persistence;
mod query;
mod stats;

/// A colection of Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graphs {
    /// The collections of Graph
    vault: HashMap<String, Vec<Vertex>>,
    /// Name for the current collection
    label: String,
    /// Some attributes to handle metada for Graphs
    stats: GraphsStats,
}

impl Graphs {
    /// Initializes a new Graphs element
    pub fn init(label: &str) -> Self {
        let mut vault: HashMap<String, Vec<Vertex>> = HashMap::new();
        vault.insert(String::from(label), vec![]);
        let graphs = Graphs {
            label: String::from(label),
            vault,
            stats: GraphsStats::init(),
        };
        debug!("Created new Graphs: {:#?}", graphs);
        graphs
    }

    /// Initializes a new Graphs element adding a Graph to new vault
    pub fn init_with(label: &str, graph: &Vertex) -> Self {
        let mut graphs = Graphs::init(label);
        graphs.add_graph(graph, None);
        graphs
    }

    /// Creates a new entry on Graphs valut
    pub fn insert(&mut self, name: &str) {
        self.vault.insert(String::from(name), vec![]);
        self.label = String::from(name);
        self.stats = self.stats().unwrap();
        debug!("Insertered new entry to Graphs valut: {:#?}", self);
    }

    /// Creates a new entry on Graphs valut with a Graph
    pub fn insert_with(&mut self, name: &str, graph: &Vertex) {
        self.vault.insert(String::from(name), vec![]);
        self.label = String::from(name);
        self.add_graph(graph, Some(name));
        self.stats = self.stats().unwrap();
        debug!("Insertered new entry to Graphs valut: {:#?}", self);
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string()
    }

    pub fn get_stats(&self) -> GraphsStats {
        self.stats.clone()
    }

    /// Adds a Graph element to the Graphs' vault
    /// for the provided graphs vault name
    /// if does not exists it creates a new entry
    /// at vault.
    /// If None name is provided, the current one
    /// is use for the addition.
    pub fn add_graph(&mut self, graph: &Vertex, graphs_name: Option<&str>) {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(v) = self.vault.get_mut(&current_graph) {
            v.push(graph.clone());
            debug!(
                "Added new graph to Graphs [{}]
                current length: {}",
                current_graph,
                self.len()
            );
        } else {
            self.insert(&current_graph);
            let v = self.vault.get_mut(&current_graph).unwrap();
            v.push(graph.clone());
            debug!("no graph element at vault, created one and added graph");
        }
        graphs_memory_watcher(self);
    }

    /// Retrieves the collection of graphs
    /// the default one or by name
    pub fn get_graphs(&self, graphs_name: Option<&str>) -> Result<Vec<Vertex>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            Ok(graphs.clone())
        } else {
            Err("no graphs found on vault")
        }
    }

    pub fn get_uniq_edges(&self, graphs_name: Option<&str>) -> Result<Vec<Edge>, &'static str> {
        let graphs = self.get_graphs(graphs_name).unwrap();
        let mut edges_map: HashMap<String, Edge> = HashMap::new();
        for graph in graphs {
            edges_map.insert(graph.get_from_edge().get_id(), graph.get_from_edge());
            edges_map.insert(graph.get_to_edge().get_id(), graph.get_to_edge());
        }
        let uniq_edges: Vec<Edge> = edges_map.into_values().collect();
        Ok(uniq_edges)
    }

    /// Updates the name of the Graphs
    pub fn update_label(&mut self, label: &str) {
        debug!("Update Graph with name: {}", label);
        self.label = label.to_string();
    }

    /// Deletes the Graph that matches with the provided id
    pub fn delete_graph_by_id(
        &mut self,
        id: String,
        graphs_name: Option<&str>,
    ) -> Result<(), &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get_mut(&current_graph) {
            let index = graphs.iter().position(|graph| graph.get_id() == id);
            if index.is_some() {
                debug!("Delete graph: {}", id);
                graphs.remove(index.unwrap());
                Ok(())
            } else {
                error!("Graph [{}] to delete not found", id);
                Err("Graph to delete not found")
            }
        } else {
            Err("no graphs found at vault")
        }
    }

    /// Updates the Graphs with the provided one
    pub fn update_graph(
        &mut self,
        graph_to_update: &Vertex,
        graphs_name: Option<&str>,
    ) -> Result<(), &'static str> {
        debug!("Going to update Graphs with {:#?}", graph_to_update);
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get_mut(&current_graph) {
            let index = graphs
                .iter()
                .position(|graph| graph.get_id() == graph_to_update.get_id());
            if index.is_some() {
                let i = index.unwrap();
                graphs.remove(i);
                debug!("Graph to update found it at index: {i}");
                graphs.push(graph_to_update.clone());
                graphs_memory_watcher(self);
                Ok(())
            } else {
                error!(
                    "Graph to update with id: [{}] not found",
                    graph_to_update.get_id()
                );
                Err("graph to update not found")
            }
        } else {
            Err("no graphs in vault")
        }
    }
}

// A bundle for util functions
impl Graphs {
    /// Retrieves the current graphs or returns the option one
    fn select_graphs_label(&self, graphs_label: Option<&str>) -> String {
        let mut current_graph = self.label.clone();
        if let Some(gn) = graphs_label {
            current_graph = gn.to_string();
        }
        current_graph.to_string()
    }
}
