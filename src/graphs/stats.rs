use crate::graphs::Graphs;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::QueryAttribute;

/// Represents stats data from the Graphs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphsStats {
    /// memory used by Graphs in bytes
    mem: usize,
    /// length of the Graph's vault
    len_graphs: usize,
    /// total graphs
    total_graphs: usize,
    /// total attributes
    total_attr: usize,
    /// total nodes
    total_nodes: usize,
    /// unique relations
    uniq_rel: usize,
}

impl GraphsStats {
    pub fn init() -> Self {
        GraphsStats {
            mem: 64,
            len_graphs: 0,
            total_graphs: 0,
            total_attr: 0,
            total_nodes: 0,
            uniq_rel: 0,
        }
    }

    pub fn get_mem(&self) -> usize {
        self.mem
    }

    pub fn get_len_graphs(&self) -> usize {
        self.len_graphs
    }

    pub fn get_total_graphs(&self) -> usize {
        self.total_graphs
    }

    pub fn get_total_attr(&self) -> usize {
        self.total_attr
    }

    pub fn get_total_nodes(&self) -> usize {
        self.total_nodes
    }

    pub fn get_uniq_rel(&self) -> usize {
        self.uniq_rel
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
    /// and total amount of Nodes
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    /// use crate::gruphst::*;
    ///
    /// let mut my_graphs = Graphs::init("memories");
    /// my_graphs.add_graph(
    ///     &Graph::create(
    ///         &Node::new("Alice"),
    ///         "recalls friendship with",
    ///         &Node::new("Bob")
    ///     ), None
    /// );
    /// let mut fred = Node::new("Fred");
    /// fred.set_attr("address", "Elm street");
    /// fred.set_attr("phone", "555-555-555");
    /// fred.set_attr("age", "25");
    ///
    /// my_graphs.add_graph(
    ///     &Graph::create(
    ///         &fred,
    ///         "relative of",
    ///         &Node::new("Coco")
    ///     ), None
    /// );
    ///
    /// let stats = my_graphs.stats().unwrap();
    /// assert_eq!(stats.get_mem(), 856);
    /// assert_eq!(stats.get_len_graphs(), 2);
    /// assert_eq!(stats.get_total_attr(), 3);
    /// assert_eq!(stats.get_total_nodes(), 4);
    /// assert_eq!(stats.get_uniq_rel(), 2);
    /// assert_eq!(stats.get_total_graphs(), 1);
    /// ```
    pub fn stats(&self) -> Result<GraphsStats, Box<dyn Error>> {
        get_stats(self)
    }

    // TODO: add uniq relations for all the graphs doc-test
    pub fn uniq_graph_relations(&self, graphs_name: Option<&str>) -> Vec<String> {
        let mut uniq_rel = Vec::new();
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            for graph in graphs.iter() {
                uniq_rel.push(graph.get_relation());
            }
            uniq_rel.sort();
            uniq_rel.dedup();
            uniq_rel
        } else {
            // TODO: return an error if any graph????
            error!("no graphs in vault");
            uniq_rel
        }
    }

    /// Returns an array with the unique relations in the default Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    /// use crate::gruphst::*;
    ///
    /// let mut my_graph = Graphs::init("my graph");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let fred = Node::new("Fred");
    ///
    /// my_graph.add_graph(&Graph::create(&alice, "friend of", &bob), None);
    /// my_graph.add_graph(&Graph::create(&alice, "relative of", &fred), None);
    /// my_graph.add_graph(&Graph::create(&fred, "friend of", &bob), None);
    /// my_graph.add_graph(&Graph::create(&bob, "friend of", &alice), None);
    /// my_graph.add_graph(&Graph::create(&fred, "relative of", &alice), None);
    ///
    /// let relations = my_graph.uniq_relations();
    /// assert_eq!(relations, vec!["friend of", "relative of"]);
    /// ```
    pub fn uniq_relations(&self) -> Vec<String> {
        let mut uniq_rel = Vec::new();
        for graphs in self.vault.values() {
            for graph in graphs.iter() {
                uniq_rel.push(graph.get_relation());
            }
            uniq_rel.sort();
            uniq_rel.dedup();
        }
        uniq_rel
    }

    /// Retrieves the length of the Graphs for whole vault
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    /// use crate::gruphst::*;
    ///
    /// let mut graphs = Graphs::init("lengths");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    ///
    /// graphs.add_graph(&Graph::create(&alice, "friend", &bob), None);
    /// graphs.add_graph(&Graph::create(&bob, "friend", &alice), None);
    ///
    /// assert_eq!(graphs.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        let mut length = 0;
        for (_graphs_name, graphs) in self.vault.iter() {
            length += graphs.len();
        }
        debug!("Requested length for vault, current length: {}", length);
        length
    }

    /// Retrieves the length of vault
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    /// use crate::gruphst::*;
    ///
    /// let mut graphs = Graphs::init("graph 0");
    /// assert_eq!(graphs.len_graphs(), 1);
    ///
    /// graphs.insert("graph 1");
    /// assert_eq!(graphs.len_graphs(), 2);
    /// ```
    pub fn len_graphs(&self) -> usize {
        self.vault.len()
    }

    /// Checks if the Graphs is empty
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    /// use crate::gruphst::*;
    ///
    /// let mut graphs = Graphs::init("lengths");
    ///
    /// assert!(graphs.is_empty());
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    ///
    /// graphs.add_graph(&Graph::create(&alice, "friend", &bob), None);
    /// graphs.add_graph(&Graph::create(&bob, "friend", &alice), None);
    ///
    /// assert!(!graphs.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// private function to generate stats
fn get_stats(grphs: &Graphs) -> Result<GraphsStats, Box<dyn Error>> {
    let bytes = bincode::serialize(grphs)?;
    // lets count the amount of attributes in the graph
    let mut attr_counter = 0;
    for (_graph_name, graphs) in grphs.vault.iter() {
        for graph in graphs {
            attr_counter += graph.get_from_node().len_attr();
            attr_counter += graph.get_to_node().len_attr();
        }
    }

    let stats = GraphsStats {
        mem: bytes.len(),
        len_graphs: grphs.len(),
        total_attr: attr_counter,
        total_nodes: grphs.len() * 2,
        uniq_rel: grphs.uniq_relations().len(),
        total_graphs: grphs.vault.len(),
    };
    debug!("Graphs stats: {:#?}", stats);
    Ok(stats)
}