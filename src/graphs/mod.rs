use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::graph::Graph;
use crate::util::graphs_memory_watcher;

mod persistence;
mod query;
mod stats;

/// A colection of Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graphs {
    /// The collections of Graph
    pub vault: HashMap<String, Vec<Graph>>,
    /// Name for the current collection
    pub name: String,
    /// The uuid for the collection
    pub id: String,
}

impl Graphs {
    /// Creates a new collection of Graph elements
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    ///
    /// let my_graph = Graphs::init("my_graph");
    /// ```
    pub fn init(name: &str) -> Self {
        let mut vault: HashMap<String, Vec<Graph>> = HashMap::new();
        vault.insert(String::from(name), vec![]);
        let graphs = Graphs {
            name: String::from(name),
            id: Uuid::new_v4().to_string(),
            vault,
        };
        debug!("Created new Graphs: {:#?}", graphs);
        graphs
    }

    /// Creates a new element to Graphs vault
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    ///
    /// let first_graph = Graphs::init("first graph");
    /// let first_graph_id = first_graph.id;
    ///
    /// ```
    pub fn new(&mut self, name: &str) -> &mut Graphs {
        self.vault.insert(String::from(name), vec![]);
        self.name = name.to_string();
        debug!("Created new Graphs: {:#?}", self);
        self
    }

    /// Adds a Graph element to the current colection
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob_graph = Graph::new(&alice, "friend of", &bob);
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add_graph(&alice_bob_graph, None);
    /// ```
    pub fn add_graph(&mut self, graph: &Graph, graphs_name: Option<&str>) {
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(v) = self.vault.get_mut(&current_graph) {
            v.push(graph.clone());
            debug!(
                "Added new graph to Graphs [{}, {}]
                current length: {}",
                self.id,
                current_graph,
                self.len()
            );
            graphs_memory_watcher(self);
        } else {
            error!("no graph element {} at vault", current_graph);
        }
    }

    /// Retrieves the collection of graphs
    /// the default one or by name
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    /// use gruphst::graph::Graph;
    /// use gruphst::node::Node;
    ///
    /// let mut the_graphs = Graphs::init("init graph");
    ///
    /// let graph = Graph::new(
    ///     &Node::new("alice"),
    ///     "knows",
    ///     &Node::new("bob"));
    /// the_graphs.add_graph(&graph, None);
    ///
    /// assert_eq!(the_graphs.name, "init graph");
    /// let default_graph = the_graphs.get_graphs(None).unwrap();
    /// assert_eq!(default_graph[0].id, graph.id);
    ///
    /// the_graphs.new("new one");
    /// let graph1 = Graph::new(
    ///     &Node::new("bilbo"),
    ///     "relative",
    ///     &Node::new("frodo")
    /// );
    /// the_graphs.add_graph(&graph1, Some("new one"));
    /// assert_eq!(the_graphs.name, "new one");
    /// let other_graph = the_graphs.get_graphs(Some("new one")).unwrap();
    /// assert_eq!(other_graph[0].id, graph1.id);
    /// ```
    pub fn get_graphs(&self, graphs_name: Option<&str>) -> Result<Vec<Graph>, &'static str> {
        let current_graph = self.select_graphs_name(graphs_name);
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
    ///
    /// let mut my_graph = Graphs::init("my_graph");
    /// assert_eq!(my_graph.name, "my_graph");
    ///
    /// my_graph.update_name("graphy");
    /// assert_eq!(my_graph.name, "graphy");
    /// ```
    // TODO: This must deal with multiple vaults
    pub fn update_name(&mut self, name: &str) {
        debug!("Update Graph [{}] with name: {}", self.id, name);
        self.name = name.to_string();
    }

    /// Deletes the Graph that matches with the provided id
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut my_graph = Graphs::init("friends");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob = Graph::new(&alice, "is friend of", &bob);
    /// my_graph.add_graph(&alice_bob, None);
    ///
    /// let alice_fred =
    ///     Graph::new(&alice, "is firend of", &Node::new("Fred"));
    /// my_graph.add_graph(&alice_fred, None);
    ///
    /// assert_eq!(my_graph.len(), 2);
    ///
    /// my_graph.delete_graph_by_id(alice_bob.id, None);
    /// assert_eq!(my_graph.len(), 1);
    /// ```
    pub fn delete_graph_by_id(
        &mut self,
        id: String,
        graphs_name: Option<&str>,
    ) -> Result<(), &'static str> {
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get_mut(&current_graph) {
            let index = graphs.iter().position(|graph| graph.id == id);
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
    ///
    ///
    /// let mut my_graphs = Graphs::init("my-graphs");
    ///
    /// let alice_node = Node::new("Alice");
    /// let bob_node = Node::new("Bob");
    /// let alice_bob_graph =
    ///     Graph::new(&alice_node, "best friends", &bob_node);
    /// my_graphs.add_graph(&alice_bob_graph, None);
    ///
    /// let fred_node = Node::new("Fred");
    /// let mut alice_fred_graph =
    ///     Graph::new(&alice_node, "super friends", &fred_node);
    /// my_graphs.add_graph(&alice_fred_graph, None);
    ///
    /// assert_eq!(my_graphs.len(), 2);
    ///
    /// let graphs = my_graphs.get_graphs(Some(&my_graphs.name)).unwrap();
    /// assert_eq!(graphs[1].relation, "super friends");
    ///
    /// alice_fred_graph.update_relation("besties");
    /// my_graphs.update_graph(&alice_fred_graph, None);
    ///
    /// assert_eq!(my_graphs.len(), 2);
    /// let updated_graph = my_graphs.find_by_id(&alice_fred_graph.id, None);
    /// assert_eq!(updated_graph.unwrap().relation, "besties");
    /// ```
    pub fn update_graph(
        &mut self,
        graph_to_update: &Graph,
        graphs_name: Option<&str>,
    ) -> Result<(), &'static str> {
        debug!("Going to update Graphs with {:#?}", graph_to_update);
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get_mut(&current_graph) {
            let index = graphs
                .iter()
                .position(|graph| graph.id == graph_to_update.id);
            if index.is_some() {
                let i = index.unwrap();
                graphs.remove(i);
                debug!("Graph to update found it at index: {i}");
                graphs.push(graph_to_update.clone());
                graphs_memory_watcher(self);
                Ok(())
            } else {
                // TODO: reformat this!
                error!(
                    "Graph to update with id: [{}] not found",
                    graph_to_update.id
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
    fn select_graphs_name(&self, graphs_name: Option<&str>) -> String {
        let mut current_graph = self.name.clone();
        if let Some(gn) = graphs_name {
            current_graph = gn.to_string();
        }
        current_graph.to_string()
    }
}
