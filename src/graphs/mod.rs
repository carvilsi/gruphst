use log::{debug, error};
use serde::{Deserialize, Serialize};
use stats::GraphsStats;
use std::collections::HashMap;

use crate::{edge::Edge, vertex::Vertex};

mod persistence;
mod query;
mod stats;

/// A colection of Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graphs {
    /// The container of Vertices 
    vault: HashMap<String, Vec<Vertex>>,
    /// Name for the current vault 
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
    pub fn init_with(label: &str, vertex: &Vertex) -> Self {
        let mut graphs = Graphs::init(label);
        graphs.add_vertex(vertex, None);
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
    pub fn insert_with(&mut self, name: &str, vertex: &Vertex) {
        self.vault.insert(String::from(name), vec![]);
        self.label = String::from(name);
        self.add_vertex(vertex, Some(name));
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

    /// Adds a Vertex element to the Graphs' vault
    /// for the provided graphs vault name
    /// if does not exists it creates a new entry
    /// at vault.
    /// If None name is provided, the current one
    /// is use for the addition.
    pub fn add_vertex(&mut self, vertex: &Vertex, vault_name: Option<&str>) {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(v) = self.vault.get_mut(&current_vault) {
            v.push(vertex.clone());
            debug!(
                "Added new vertex to Graphs [{}]
                current length: {}",
                current_vault,
                self.len()
            );
        } else {
            self.insert(&current_vault);
            let v = self.vault.get_mut(&current_vault).unwrap();
            v.push(vertex.clone());
            debug!("no vertex element at vault, created one and added vertex");
        }
    }

    // TODO: create method to add a collection of vertex, AKA vertices

    /// Retrieves the collection of vertices 
    /// the default one or by name
    pub fn get_vertices(&self, vault_name: Option<&str>) -> Result<Vec<Vertex>, &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get(&current_vault) {
            Ok(vertices.clone())
        } else {
            Err("no graphs found on vault")
        }
    }

    /// Returns a collection with the unique edges on a vault
    pub fn get_uniq_edges(&self, vault_name: Option<&str>) -> Result<Vec<Edge>, &'static str> {
        let vertices = self.get_vertices(vault_name).unwrap();
        let mut edges_map: HashMap<String, Edge> = HashMap::new();
        for vertex in vertices {
            edges_map.insert(vertex.get_from_edge().get_id(), vertex.get_from_edge());
            edges_map.insert(vertex.get_to_edge().get_id(), vertex.get_to_edge());
        }
        let uniq_edges: Vec<Edge> = edges_map.into_values().collect();
        Ok(uniq_edges)
    }

    /// Updates the name of the Graphs
    pub fn update_label(&mut self, label: &str) {
        debug!("Update Graph with name: {}", label);
        self.label = label.to_string();
    }

    /// Deletes the Vertex that matches with the provided id
    pub fn delete_vertex_by_id(
        &mut self,
        id: String,
        vault_name: Option<&str>,
    ) -> Result<(), &'static str> {
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get_mut(&current_vault) {
            let index = vertices.iter().position(|vertex| vertex.get_id() == id);
            if index.is_some() {
                vertices.remove(index.unwrap());
                debug!("Deleted vertex: {}", id);
                Ok(())
            } else {
                error!("Vertex [{}] to delete not found", id);
                Err("Vertex to delete not found")
            }
        } else {
            Err("no vertices found on vault")
        }
    }

    /// Updates the Vertex on vault with the provided one
    pub fn update_graph(
        &mut self,
        vertex_to_update: &Vertex,
        vault_name: Option<&str>,
    ) -> Result<(), &'static str> {
        debug!("Going to update Graphs with {:#?}", vertex_to_update);
        let current_vault = self.select_vault_label(vault_name);
        if let Some(vertices) = self.vault.get_mut(&current_vault) {
            let index = vertices
                .iter()
                .position(|vertex| vertex.get_id() == vertex_to_update.get_id());
            if index.is_some() {
                let i = index.unwrap();
                vertices.remove(i);
                debug!("Vertex to update found it at index: {i}");
                vertices.push(vertex_to_update.clone());
                Ok(())
            } else {
                error!(
                    "Vertex to update with id: [{}] not found",
                    vertex_to_update.get_id()
                );
                Err("vertex to update not found")
            }
        } else {
            Err("no graphs in vault")
        }
    }
}

// A bundle for util functions
impl Graphs {
    /// Retrieves the current vault or returns the current one
    fn select_vault_label(&self, vault_label: Option<&str>) -> String {
        let mut current_vault = self.label.clone();
        if let Some(vlt) = vault_label {
            current_vault = vlt.to_string();
        }
        current_vault.to_string()
    }
}
