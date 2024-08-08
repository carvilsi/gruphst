use crate::graphs::Graphs;
use log::error;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Represents stats data from the Graphs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphsStats {
    /// memory used by Graphs in bytes
    mem: usize,
    /// length of the Graph's vault
    total_edges: usize,
    /// total graphs
    total_graphs: usize,
    /// total attributes
    total_attr: usize,
    /// total edges
    total_vertices: usize,
    /// unique relations
    uniq_rel: usize,
}

impl GraphsStats {
    pub fn init() -> Self {
        GraphsStats {
            mem: 64,
            total_edges: 0,
            total_graphs: 0,
            total_attr: 0,
            total_vertices: 0,
            uniq_rel: 0,
        }
    }

    pub fn get_mem(&self) -> usize {
        self.mem
    }

    pub fn get_len_graphs(&self) -> usize {
        self.total_edges
    }

    pub fn get_total_graphs(&self) -> usize {
        self.total_graphs
    }

    pub fn get_total_attr(&self) -> usize {
        self.total_attr
    }

    pub fn get_total_edges(&self) -> usize {
        self.total_edges
    }

    pub fn get_uniq_rel(&self) -> usize {
        self.uniq_rel
    }

    pub fn get_total_vertices(&self) -> usize {
        self.total_vertices
    }

    pub fn generate_stats(graphs: &Graphs) -> Self {
        match get_stats(graphs) {
            Ok(stats) => stats,
            Err(_) => panic!("not possible to generate stats for graphs"),
        }
    }
}

impl Graphs {
    /// Returns stats from Graphs; size in bytes, amount of graph, name, total number of attributes
    /// and total amount of edges
    pub fn stats(&self) -> Result<GraphsStats, Box<dyn Error>> {
        get_stats(self)
    }

    /// Returns an array with the unique relations in the current graph
    /// or the one provided
    pub fn uniq_graph_relations(
        &self,
        graphs_name: Option<&str>,
    ) -> Result<Vec<String>, &'static str> {
        let mut uniq_rel = Vec::new();
        let current_graph = self.select_vault_label(graphs_name);
        if let Some(edges) = self.vault.get(&current_graph) {
            for edge in edges.iter() {
                uniq_rel.push(edge.get_relation());
            }
            uniq_rel.sort();
            uniq_rel.dedup();
            Ok(uniq_rel)
        } else {
            error!("no graphs in vault");
            Err("vault does not exists")
        }
    }

    /// Returns an array with the unique relations in the whole Graphs
    pub fn uniq_relations(&self) -> Vec<String> {
        let mut uniq_rel = Vec::new();
        for edges in self.vault.values() {
            for edge in edges.iter() {
                uniq_rel.push(edge.get_relation());
            }
            uniq_rel.sort();
            uniq_rel.dedup();
        }
        uniq_rel
    }

    /// Retrieves the length of the Graphs for whole vault
    pub fn len(&self) -> usize {
        let mut length = 0;
        for (_graphs_name, edges) in self.vault.iter() {
            length += edges.len();
        }
        length
    }

    /// Retrieves the length of vault
    pub fn len_graphs(&self) -> usize {
        self.vault.len()
    }

    /// Checks if the Graphs vault is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// private function to generate stats
fn get_stats(grphs: &Graphs) -> Result<GraphsStats, Box<dyn Error>> {
    let bytes = bincode::serialize(grphs)?;
    // lets count the amount of attributes in the graph
    let mut attr_counter = 0;
    for (_graph_name, edges) in grphs.vault.iter() {
        for edge in edges {
            attr_counter += edge.get_from_vertex().attr_len();
            attr_counter += edge.get_to_vertex().attr_len();
        }
    }

    let stats = GraphsStats {
        mem: bytes.len(),
        total_edges: grphs.len(),
        total_attr: attr_counter,
        total_vertices: grphs.len() * 2,
        uniq_rel: grphs.uniq_relations().len(),
        total_graphs: grphs.vault.len(),
    };
    Ok(stats)
}
