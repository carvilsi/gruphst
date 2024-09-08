use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{edge::Edge, graphs_stats::GraphsStats, util::graphs_memory_watcher, vertex::Vertex};

mod persistence;
mod queries;
mod stats;

/// A colection of Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graphs {
    /// The container of Edges
    vault: HashMap<String, Vec<Edge>>,
    /// Name for the current vault
    label: String,
    /// Some attributes to handle metada for Graphs
    stats: GraphsStats,
}

impl Graphs {
    /// Initializes a new Graphs element
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    ///  
    /// Graphs::init("my graph");
    /// ```
    pub fn init(label: &str) -> Self {
        let mut vault: HashMap<String, Vec<Edge>> = HashMap::new();
        vault.insert(String::from(label), vec![]);
        Graphs {
            label: String::from(label),
            vault,
            stats: GraphsStats::init(),
        }
    }

    /// Initializes a new Graphs element adding a Edge to new vault
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs};
    ///  
    /// let edge = Edge::create(
    ///     &Vertex::new("Sauron"),
    ///     "created",
    ///     &Vertex::new("One Ring"));
    /// Graphs::init_with("my graph", &edge);
    /// ```
    pub fn init_with(label: &str, vertex: &Edge) -> Self {
        let mut graphs = Graphs::init(label);
        graphs.add_edge(vertex, None);
        graphs
    }

    /// Creates a new entry on Graphs valut
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    ///  
    /// let mut graphs = Graphs::init("my graphs");
    /// graphs.insert("my other graphs");
    /// ```
    pub fn insert(&mut self, name: &str) {
        self.vault.insert(String::from(name), vec![]);
        self.label = String::from(name);
        graphs_memory_watcher(self);
    }

    /// Creates a new entry on Graphs valut with a Graph
    pub fn insert_with(&mut self, name: &str, edge: &Edge) {
        self.vault.insert(String::from(name), vec![]);
        self.label = String::from(name);
        self.add_edge(edge, Some(name));
        graphs_memory_watcher(self);
    }

    /// Returns the label or name for the graphs
    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    /// Sets the label or name for the graphs
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string()
    }

    /// Returns the stats for a grpahs
    /// the stats are generated 
    pub fn get_stats(&mut self) -> GraphsStats {
        self.stats = GraphsStats::generate_stats(self);
        self.stats.clone()
    }

    /// Returns the GraphsStats object
    pub fn get_graphs_stats(&self) -> GraphsStats {
        self.stats.clone()
    }

    pub fn get_vaults(&self) -> HashMap<String, Vec<Edge>> {
        self.vault.clone()
    }

    /// Adds a Edge element to the Graphs' vault
    /// for the provided graphs vault name
    /// if does not exists it creates a new entry
    /// at vault.
    /// If None name is provided, the current one
    /// is use for the addition.
    pub fn add_edge(&mut self, edge: &Edge, vault_name: Option<&str>) {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(e) = self.vault.get_mut(&current_vault) {
            e.push(edge.clone());
        } else {
            self.insert(&current_vault);
            let v = self.vault.get_mut(&current_vault).unwrap();
            v.push(edge.clone());
        }
        graphs_memory_watcher(self);
    }

    /// Adds a collection of Edges to the Graphs' vault
    /// for the provided graphs vault name
    /// if does not exists it creates a new entry
    /// at vault.
    /// If None name is provided, the current one
    /// is use for the addition.
    pub fn add_edges(&mut self, edges: &mut Vec<Edge>, vault_name: Option<&str>) {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(e) = self.vault.get_mut(&current_vault) {
            e.append(edges);
        } else {
            self.insert(&current_vault);
            let v = self.vault.get_mut(&current_vault).unwrap();
            v.append(edges);
        }
        graphs_memory_watcher(self);
    }

    /// Retrieves the collection of edges
    /// the default one or by name
    pub fn get_edges(&self, vault_name: Option<&str>) -> Result<Vec<Edge>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get(&current_vault) {
            Ok(edges.clone())
        } else {
            Err("no graphs in vault")
        }
    }

    /// Returns a collection with the unique vertices on a vault
    pub fn get_uniq_vertices(&self, vault_name: Option<&str>) -> Result<Vec<Vertex>, &'static str> {
        let edges = self.get_edges(vault_name).unwrap();
        let mut vertices_map: HashMap<String, Vertex> = HashMap::new();
        for edge in edges {
            vertices_map.insert(edge.get_from_vertex().get_id(), edge.get_from_vertex());
            vertices_map.insert(edge.get_to_vertex().get_id(), edge.get_to_vertex());
        }
        let uniq_vertices: Vec<Vertex> = vertices_map.into_values().collect();
        Ok(uniq_vertices)
    }

    /// Updates the name of the Graphs
    pub fn update_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    /// Deletes the Edge that matches with the provided id
    pub fn delete_edge_by_id(
        &mut self,
        id: String,
        vault_name: Option<&str>,
    ) -> Result<(), &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get_mut(&current_vault) {
            if let Some(index) = edges.iter().position(|edge| edge.get_id() == id) {
                edges.remove(index);
                graphs_memory_watcher(self);
                Ok(())
            } else {
                error!("Edge [{}] to delete not found", id);
                Err("Edge to delete not found")
            }
        } else {
            Err("no graphs in vault")
        }
    }

    /// Updates the Edge on vault with the provided one
    pub fn update_graph(
        &mut self,
        edge_to_update: &Edge,
        vault_name: Option<&str>,
    ) -> Result<(), &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(edges) = self.vault.get_mut(&current_vault) {
            let index = edges
                .iter()
                .position(|vertex| vertex.get_id() == edge_to_update.get_id());
            if index.is_some() {
                let i = index.unwrap();
                edges.remove(i);
                edges.push(edge_to_update.clone());
                graphs_memory_watcher(self);
                Ok(())
            } else {
                error!("Edge to update with id: [{}] not found", edge_to_update.get_id());
                Err("edge to update not found")
            }
        } else {
            Err("no graphs in vault")
        }
    }

    /// Retrieves the current vault or returns the current one
    fn select_vault_label(&self, vault_label: Option<&str>) -> String {
        let mut current_vault = self.label.clone();
        if let Some(vlt) = vault_label {
            current_vault = vlt.to_string();
        }
        current_vault.to_string()
    }
}
