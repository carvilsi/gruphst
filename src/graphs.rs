use log::{debug, error, warn, info, trace};
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;
use std::thread;
use std::process;

use crate::graph::Graph;

//const DEFAULT_GRUPHST_MAX_MEM_USAGE: usize = 25 * 1024 * 1024;
const DEFAULT_GRUPHST_MAX_MEM_USAGE: usize = 5 * 1024 * 1024;

// TODO: Add a watchdog to check the amount of mem usage


fn graphs_memory_watcher(graphs: &Graphs) {
    println!("WTF!!!!!-----!!!!!!");
    let g = graphs.clone();
    let l = thread::spawn(move || {
        println!("---- WTH ---");
        let mem = g.stats().unwrap().mem;
        let mem_prss = (mem as f32 * 100_f32) / DEFAULT_GRUPHST_MAX_MEM_USAGE as f32;
        trace!("memory preassure: {:.2}", mem_prss);
        println!("memory preassure: {:.2}", mem_prss);
        match mem_prss {
            mem_prss if mem_prss < 70_f32 => debug!("memory ok: {:.2}", mem_prss),
            mem_prss if mem_prss >= 80_f32 && mem_prss < 95_f32 => info!("memory high: {:.2}", mem_prss),
            mem_prss if mem_prss >=95_f32 && mem_prss < 99_f32 => warn!("memory close to the limit: {:.2}", mem_prss),
            mem_prss if mem_prss >=99_f32 => {
                error!("memory usage critical: {:.2}", mem_prss);
                error!("auto persisting current graphs: {}, and stoping execution", g.name);
                let _ = g.persists();
                process::exit(1);
            },
            _ => todo!(), 
        }
    });
    l.join().unwrap();
}

/// A colection of Graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graphs {
    /// The collection of Graph
    pub graphs: Vec<Graph>,
    /// Name for the collection
    pub name: String,
    /// The uuid for the collection
    pub id: String,
}

/// Represents stats data from the Graphs
#[derive(Debug)]
pub struct GraphsStats<'a> {
    /// memory used by Graphs in bytes
    pub mem: usize,
    /// length of the Graphs
    pub len: usize,
    /// name of the Graph
    pub name: &'a str,
    /// total attributes
    pub total_attr: usize,
    /// total nodes
    pub total_nodes: usize,
    /// unique relations
    pub uniq_rel: usize,
}

// TODO: Graphs: return Graph(s) that matches an attribute node by key
// TODO: Graphs: return Graph(s) that matches an attribute node by key and value
// TODO: Graphs: return Graph(s) that matches an attribute node by like key
// TODO: Graphs: return Graph(s) that matches an array of relations
// TODO: Graphs: return Graph(s) that matches an array of relations on order
impl Graphs {
    /// Creates a new collection of Graph elements
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    ///
    /// let my_graph = Graphs::new("my_graph");
    /// ```
    pub fn new(name: &str) -> Self {
        let graphs = Graphs {
            name: String::from(name),
            id: Uuid::new_v4().to_string(),
            graphs: vec![],
        };
        debug!("Created new Graphs: {:#?}", graphs);
        graphs
    }

    /// Adds a Graph element to the colection
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
    /// let mut my_graph = Graphs::new("my_graph");
    /// my_graph.add(&alice_bob_graph);
    /// ```
    pub fn add(&mut self, graph: &Graph) {
        debug!(
            "Added new graph to Graphs [{}]
            current length: {}",
            self.id,
            self.len()
        );
        println!("-- 0 --");
        graphs_memory_watcher(self);
        println!("-- 1 --");
        self.graphs.push(graph.clone());
    }

    /// Retrieves the length of the Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut graphs = Graphs::new("lengths");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    ///
    /// graphs.add(&Graph::new(&alice, "friend", &bob));
    /// graphs.add(&Graph::new(&bob, "friend", &alice));
    ///
    /// assert_eq!(graphs.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        debug!(
            "Requested length for graphs, current length: {}",
            self.graphs.len()
        );
        self.graphs.len()
    }

    /// Checks if the Graphs is empty
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut graphs = Graphs::new("lengths");
    ///
    /// assert!(graphs.is_empty());
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    ///
    /// graphs.add(&Graph::new(&alice, "friend", &bob));
    /// graphs.add(&Graph::new(&bob, "friend", &alice));
    ///
    /// assert!(!graphs.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Updates the name of the Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut my_graph = Graphs::new("my_graph");
    /// assert_eq!(my_graph.name, "my_graph");
    ///
    /// my_graph.update_name("graphy");
    /// assert_eq!(my_graph.name, "graphy");
    /// ```
    pub fn update_name(&mut self, name: &str) {
        debug!("Update Graph [{}] with name: {}", self.id, name);
        self.name = name.to_string();
    }

    /// Returns a collection of Graps elements that matches the relation
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
    /// let mut my_graph = Graphs::new("my_graph");
    /// my_graph.add(&alice_bob_graph);
    ///
    /// let result_graph = my_graph.find_by_relation("friend of").unwrap();
    /// assert_eq!(result_graph.len(), 1);
    /// ```
    pub fn find_by_relation(&mut self, relation_name: &str) -> Result<Vec<&Graph>, &'static str> {
        let graphs = self
            .graphs
            .iter()
            .filter(|grph| grph.relation == relation_name)
            .collect::<Vec<&Graph>>();
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
    }

    /// Returns an array with the unique relations in the Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut my_graph = Graphs::new("my graph");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let fred = Node::new("Fred");
    ///
    /// my_graph.add(&Graph::new(&alice, "friend of", &bob));
    /// my_graph.add(&Graph::new(&alice, "relative of", &fred));
    /// my_graph.add(&Graph::new(&fred, "friend of", &bob));
    /// my_graph.add(&Graph::new(&bob, "friend of", &alice));
    /// my_graph.add(&Graph::new(&fred, "relative of", &alice));
    ///
    /// let relations = my_graph.uniq_relations();
    /// assert_eq!(relations, vec!["friend of", "relative of"]);
    /// ```
    pub fn uniq_relations(&self) -> Vec<&String> {
        let mut uniq_rel = Vec::new();
        for graph in self.graphs.iter() {
            uniq_rel.push(&graph.relation);
        }
        uniq_rel.sort();
        uniq_rel.dedup();
        uniq_rel
    }

    /// Returns a Graph that provided id matches with Graph, or From, To Nodes
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    ///
    /// let mut my_graph = Graphs::new("friends");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob = Graph::new(&alice, "is friend of", &bob);
    /// my_graph.add(&alice_bob);
    ///
    /// let alice_fred =
    ///     Graph::new(&alice, "is firend of", &Node::new("Fred"));
    /// my_graph.add(&alice_fred);
    ///
    /// let bob_node_id = bob.id;
    /// let res = my_graph.find_by_id(&bob_node_id);
    /// assert_eq!(res.unwrap().to.id, bob_node_id);
    /// ```
    pub fn find_by_id(&mut self, id: &str) -> Result<&mut Graph, &'static str> {
        let graph = self
            .graphs
            .iter_mut()
            .find(|graph| graph.id == id || graph.from.id == id || graph.to.id == id);
        if graph.is_some() {
            debug!("Founded Graph by id: {:#?}", graph);
            Ok(graph.unwrap())
        } else {
            error!("Graph with id [{}] not found", id);
            Err("Graph not found")
        }
    }

    /// Deletes the Graph that matches with the provided id
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut my_graph = Graphs::new("friends");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob = Graph::new(&alice, "is friend of", &bob);
    /// my_graph.add(&alice_bob);
    ///
    /// let alice_fred =
    ///     Graph::new(&alice, "is firend of", &Node::new("Fred"));
    /// my_graph.add(&alice_fred);
    ///
    /// assert_eq!(my_graph.len(), 2);
    ///
    /// my_graph.delete_graph_by_id(alice_bob.id);
    /// assert_eq!(my_graph.len(), 1);
    /// ```
    pub fn delete_graph_by_id(&mut self, id: String) -> Result<(), &'static str> {
        let index = self.graphs.iter().position(|graph| graph.id == id);
        if index.is_some() {
            debug!("Delete graph: {}", id);
            self.graphs.remove(index.unwrap());
            Ok(())
        } else {
            error!("Graph [{}] to delete not found", id);
            Err("Graph to delete not found")
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
    /// let mut my_graphs = Graphs::new("my-graphs");
    ///
    /// let alice_node = Node::new("Alice");
    /// let bob_node = Node::new("Bob");
    /// let alice_bob_graph =
    ///     Graph::new(&alice_node, "best friends", &bob_node);
    /// my_graphs.add(&alice_bob_graph);
    ///
    /// let fred_node = Node::new("Fred");
    /// let mut alice_fred_graph =
    ///     Graph::new(&alice_node, "super friends", &fred_node);
    /// my_graphs.add(&alice_fred_graph);
    ///
    /// assert_eq!(my_graphs.len(), 2);
    /// assert_eq!(my_graphs.graphs[1].relation, "super friends");
    ///
    /// alice_fred_graph.update_relation("besties");
    /// my_graphs.update_graph(&alice_fred_graph);
    ///
    /// assert_eq!(my_graphs.len(), 2);
    /// let updated_graph = my_graphs.find_by_id(&alice_fred_graph.id);
    /// assert_eq!(updated_graph.unwrap().relation, "besties");
    /// ```
    pub fn update_graph(&mut self, graph_to_update: &Graph) -> Result<(), &'static str> {
        debug!("Going to update Graphs with {:#?}", graph_to_update);
        let index = self
            .graphs
            .iter()
            .position(|graph| graph.id == graph_to_update.id);
        if index.is_some() {
            let i = index.unwrap();
            self.graphs.remove(i);
            debug!("Graph to update found it at index: {i}");
            self.graphs.push(graph_to_update.clone());
            Ok(())
        } else {
            error!(
                "Graph to update with id: [{}] not found",
                graph_to_update.id
            );
            Err("Graph not found")
        }
    }

    /// Returns stats from Graphs; size in bytes, amount of graph, name, total number of attributes
    /// and total amount of Nodes
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut my_graphs = Graphs::new("memories");
    /// my_graphs.add(
    ///     &Graph::new(
    ///         &Node::new("Alice"),
    ///         "recalls friendship with",
    ///         &Node::new("Bob")
    ///     )
    /// );
    /// let mut fred = Node::new("Fred");
    /// fred.set_attr("address", "Elm street");
    /// fred.set_attr("phone", "555-555-555");
    /// fred.set_attr("age", "25");
    ///
    /// my_graphs.add(
    ///     &Graph::new(
    ///         &fred,
    ///         "relative of",
    ///         &Node::new("Coco")
    ///     )
    /// );
    ///
    /// let stats = my_graphs.stats().unwrap();
    /// assert_eq!(stats.mem, 548);
    /// assert_eq!(stats.len, 2);
    /// assert_eq!(stats.name, "memories");
    /// assert_eq!(stats.total_attr, 3);
    /// assert_eq!(stats.total_nodes, 4);
    /// assert_eq!(stats.uniq_rel, 2);
    /// ```
    pub fn stats(&self) -> Result<GraphsStats, Box<dyn Error>> {
        let bytes = bincode::serialize(self)?;
        // lets count the amount of attributes in the graph
        let mut attr_counter = 0;
        for graph in self.graphs.iter() {
            attr_counter += graph.from.len_attr();
            attr_counter += graph.to.len_attr();
        }

        let stats = GraphsStats {
            mem: bytes.len(),
            len: self.len(),
            name: &self.name,
            total_attr: attr_counter,
            total_nodes: self.len() * 2,
            uniq_rel: self.uniq_relations().len(),
        };
        debug!("Graphs stats: {:#?}", stats);
        Ok(stats)
    }
}
