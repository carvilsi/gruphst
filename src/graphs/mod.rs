use log::{debug, error};
use serde::{Deserialize, Serialize};
use stats::GraphsStats;
use std::collections::HashMap;

use crate::graph::Graph;
use crate::util::graphs_memory_watcher;
use crate::CURNodeGraph;

mod persistence;
mod query;
mod stats;

/// A colection of Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graphs {
    /// The collections of Graph
    vault: HashMap<String, Vec<Graph>>,
    /// Name for the current collection
    label: String,
    /// Some attributes to handle metada for Graphs
    stats: GraphsStats,
}

impl Graphs {
    /// Initializes a new Graphs element
    pub fn init(label: &str) -> Self {
        let mut vault: HashMap<String, Vec<Graph>> = HashMap::new();
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
    pub fn init_with(label: &str, graph: &Graph) -> Self {
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
    pub fn insert_with(&mut self, name: &str, graph: &Graph) {
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
}

impl Graphs {
    /// Adds a Graph element to the Graphs' vault
    /// for the provided graphs vault name
    /// if does not exists it creates a new entry
    /// at vault.
    /// If None name is provided, the current one
    /// is use for the addition.
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{
    ///     node::Node,
    ///     graph::Graph,
    ///     graphs::Graphs,
    ///     *,
    /// };
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob_graph = Graph::create(&alice, "friend of", &bob);
    /// let mut my_graphs = Graphs::init("my_graph");
    /// my_graphs.add_graph(&alice_bob_graph, None);
    /// assert_eq!(my_graphs.len_graphs(), 1);
    /// my_graphs.add_graph(
    ///     &Graph::create(&bob, "best friend", &alice),
    ///     Some("other graph"));
    /// assert_eq!(my_graphs.len_graphs(), 2);
    /// ```
    pub fn add_graph(&mut self, graph: &Graph, graphs_name: Option<&str>) {
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
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    /// use gruphst::graph::Graph;
    /// use gruphst::node::Node;
    /// use crate::gruphst::*;
    ///
    /// let mut the_graphs = Graphs::init("init graph");
    ///
    /// let graph = Graph::create(
    ///     &Node::new("alice"),
    ///     "knows",
    ///     &Node::new("bob"));
    /// the_graphs.add_graph(&graph, None);
    ///
    /// assert_eq!(the_graphs.get_label(), "init graph");
    /// let default_graph = the_graphs.get_graphs(None).unwrap();
    /// assert_eq!(default_graph[0].get_id(), graph.get_id());
    ///
    /// the_graphs.insert("new one");
    /// let graph1 = Graph::create(
    ///     &Node::new("bilbo"),
    ///     "relative",
    ///     &Node::new("frodo")
    /// );
    /// the_graphs.add_graph(&graph1, Some("new one"));
    /// assert_eq!(the_graphs.get_label(), "new one");
    /// let other_graph = the_graphs.get_graphs(Some("new one")).unwrap();
    /// assert_eq!(other_graph[0].get_id(), graph1.get_id());
    /// ```
    pub fn get_graphs(&self, graphs_name: Option<&str>) -> Result<Vec<Graph>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            Ok(graphs.clone())
        } else {
            Err("no graphs found on vault")
        }
    }

    /// Updates the name of the Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    /// use crate::gruphst::*;
    ///
    /// let mut my_graph = Graphs::init("my_graph");
    /// assert_eq!(my_graph.get_label(), "my_graph");
    ///
    /// my_graph.update_label("graphy");
    /// assert_eq!(my_graph.get_label(), "graphy");
    /// ```
    pub fn update_label(&mut self, label: &str) {
        debug!("Update Graph with name: {}", label);
        self.label = label.to_string();
    }

    /// Deletes the Graph that matches with the provided id
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    /// use crate::gruphst::*;
    ///
    /// let mut my_graph = Graphs::init("friends");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob = Graph::create(&alice, "is friend of", &bob);
    /// my_graph.add_graph(&alice_bob, None);
    ///
    /// let alice_fred =
    ///     Graph::create(&alice, "is firend of", &Node::new("Fred"));
    /// my_graph.add_graph(&alice_fred, None);
    ///
    /// assert_eq!(my_graph.len(), 2);
    ///
    /// my_graph.delete_graph_by_id(alice_bob.get_id(), None);
    /// assert_eq!(my_graph.len(), 1);
    /// ```
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
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    /// use crate::gruphst::*;
    ///
    ///
    /// let mut my_graphs = Graphs::init("my-graphs");
    ///
    /// let alice_node = Node::new("Alice");
    /// let bob_node = Node::new("Bob");
    /// let alice_bob_graph =
    ///     Graph::create(&alice_node, "best friends", &bob_node);
    /// my_graphs.add_graph(&alice_bob_graph, None);
    ///
    /// let fred_node = Node::new("Fred");
    /// let mut alice_fred_graph =
    ///     Graph::create(&alice_node, "super friends", &fred_node);
    /// my_graphs.add_graph(&alice_fred_graph, None);
    ///
    /// assert_eq!(my_graphs.len(), 2);
    ///
    /// let graphs = my_graphs.get_graphs(Some(&my_graphs.get_label())).unwrap();
    /// assert_eq!(graphs[1].get_relation(), "super friends");
    ///
    /// alice_fred_graph.update_relation("besties");
    /// my_graphs.update_graph(&alice_fred_graph, None);
    ///
    /// assert_eq!(my_graphs.len(), 2);
    /// let updated_graph = my_graphs.find_by_id(&alice_fred_graph.get_id(), None);
    /// assert_eq!(updated_graph.unwrap().get_relation(), "besties");
    /// ```
    pub fn update_graph(
        &mut self,
        graph_to_update: &Graph,
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
