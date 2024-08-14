use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::graphs::Graphs;

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
    /// Initializes, creating a new instance of GraphsStats
    pub(crate) fn init() -> Self {
        GraphsStats {
            mem: 64,
            total_edges: 0,
            total_graphs: 0,
            total_attr: 0,
            total_vertices: 0,
            uniq_rel: 0,
        }
    }

    /// Retrieves the amount of memory of the Graphs
    pub fn get_mem(&self) -> usize {
        self.mem
    }

    /// Retrieves the length of elements in the vault
    pub fn get_total_graphs(&self) -> usize {
        self.total_graphs
    }

    /// Retrieves the amount of attributes on edges and vertex
    /// from graphs
    pub fn get_total_attr(&self) -> usize {
        self.total_attr
    }

    /// Retrieves the total amount of edges on Graphs
    pub fn get_total_edges(&self) -> usize {
        self.total_edges
    }

    /// Retrieves the amount of unique relations on Graphs
    pub fn get_uniq_rel(&self) -> usize {
        self.uniq_rel
    }

    /// Retrieves the total amount of vertices on Graphs
    pub fn get_total_vertices(&self) -> usize {
        self.total_vertices
    }

    /// Returns a GraphsStats object
    pub(crate) fn generate_stats(graphs: &Graphs) -> Self {
        get_stats(graphs).unwrap()
    }
}

/// private function to generate stats
fn get_stats(grphs: &Graphs) -> Result<GraphsStats, Box<dyn Error>> {
    // lets count the amount of attributes in the graph
    let mut attr_counter = 0;
    for (_name, edges) in grphs.get_vaults().iter() {
        for edge in edges {
            attr_counter += edge.get_from_vertex().attr_len();
            attr_counter += edge.get_to_vertex().attr_len();
            attr_counter += edge.attr_len();
        }
    }

    let stats = GraphsStats {
        mem: grphs.get_mem().unwrap(),
        total_edges: grphs.len(),
        total_attr: attr_counter,
        total_vertices: grphs.len() * 2,
        uniq_rel: grphs.uniq_relations().len(),
        total_graphs: grphs.get_vaults().len(),
    };
    Ok(stats)
}
