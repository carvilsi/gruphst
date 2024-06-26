//! Gruphst
//!
//! An in-memory graph database.
//! 
//! Possible to persists on file.

#![allow(clippy::unused_io_amount)]

use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::error::Error;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use std::fs::OpenOptions;
use log::{ debug, info };

const MAX_STACK_SIZE: usize = 10000;

// TODO: add env config for level
/// Enables logging providing a level
///
/// # Examples
/// ```rust
/// use gruphst::enable_logging;
///
/// enable_logging(log::Level::Info);
/// ```
pub fn enable_logging(level: log::Level) {
    simple_logger::init_with_level(level).unwrap();
}

/// Representation of a Node
#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Node {
    /// A Node consists on a uuid as identifier
    pub id: String,
    /// And a name
    pub name: String,
}

impl Node {
    /// Creates a Node with the given name, the id is generated
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    ///
    /// let node = Node::new("alice node");
    /// ```
    pub fn new(name: &str) -> Node {
        let node = Node {
            name: String::from(name),
            id: Uuid::new_v4().to_string()
        };
        debug!("The created node: {:#?}", &node);
        node
    }

    /// Updates the name of the Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// 
    ///
    /// let mut node = Node::new("alice node");
    /// assert_eq!(node.name, "alice node");
    /// node.update_name("just alice");
    /// assert_eq!(node.name, "just alice");
    /// ```
    pub fn update_name(&mut self, name: &str) {
        debug!("Updated Node [{}] with name: {}", self.id, name);
        self.name = name.to_string();
    }
}

/// Representation of a Graph, relating two nodes
#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Graph {
    /// A Graph has an uuid
    pub id: String,
    /// A name fot the relation 
    pub relation: String,
    /// Origin node
    pub from: Node,
    /// Target node
    pub to: Node,
}

impl Graph {
    /// Creates a Graph, the id is generated
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob_graph = 
    ///     Graph::new(&alice, "friend of", &bob);
    /// ```
    pub fn new(from: &Node, relation: &str, to: &Node) -> Graph {
        let graph = Graph { 
            relation: String::from(relation),
            id: Uuid::new_v4().to_string(),
            from: from.clone(),
            to: to.clone(),
        };
        debug!("The created Graph: {:#?}", graph);
        graph
    }

    /// Updates the relation for the Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// 
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let mut alice_bob_graph = Graph::new(&alice, "friend of", &bob);
    /// 
    /// assert_eq!(alice_bob_graph.relation, "friend of");
    ///
    /// alice_bob_graph.update_relation("best friends");
    /// assert_eq!(alice_bob_graph.relation, "best friends");
    /// ```
    pub fn update_relation(&mut self, relation: &str) {
        debug!("Updated Graph [{}] with Relation: {}", self.id, relation);
        self.relation = relation.to_string();
    }
    
    /// Updates the "from" node in Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// 
    ///
    /// let mut alice_node = Node::new("alice node");
    /// let bob_node = Node::new("bob node");
    /// let mut graph = Graph::new(&alice_node, "best friends", &bob_node);
    /// assert_eq!(graph.from.name, "alice node");
    /// assert_eq!(graph.to.name, "bob node");
    /// alice_node.update_name("alice");
    /// graph.update_from(&alice_node);
    /// assert_eq!(graph.from.name, "alice");
    /// ```
    pub fn update_from(&mut self, from_node: &Node) {
        debug!("Updated Graph [{}] from Node: {:#?}", self.id, from_node);
        self.from = from_node.clone();
    }

    /// Updates the "to" node in Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// 
    ///
    /// let alice_node = Node::new("alice node");
    /// let bob_node = Node::new("bob node");
    /// let mut graph = Graph::new(&alice_node, "best friends", &bob_node);
    /// assert_eq!(graph.from.name, "alice node");
    /// assert_eq!(graph.to.name, "bob node");
    /// let fred_node = Node::new("fred node");
    /// graph.update_to(&fred_node);
    /// assert_eq!(graph.to.name, "fred node");
    /// assert_ne!(graph.to.id, bob_node.id);
    /// ```
    pub fn update_to(&mut self, to_node: &Node) {
        debug!("Updated Graph [{}] to Node: {:#?}", self.id, to_node);
        self.to = to_node.clone();
    }
}

/// A colection of Graph 
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Graphs {
    /// The collection of Graph
    pub graphs: Vec<Graph>,
    /// Name for the collection
    pub name: String,
    /// The uuid for the collection
    pub id: String,
}

impl Graphs {
    /// Creates a new collection of Graph elements
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    ///
    /// let my_graph = Graphs::new("my_graph");
    /// ```
    pub fn new(name: &str) -> Graphs {
        let graphs = Graphs {
            name: String::from(name), 
            id: Uuid::new_v4().to_string(),
            graphs: vec![],
        };
        debug!("Created new Graphs: {:#?}", graphs);
        graphs
    }
    
    /// Updates the name of the Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// 
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

    /// Adds a Graph element to the colection
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob_graph = Graph::new(&alice, "friend of", &bob);
    /// let mut my_graph = Graphs::new("my_graph");
    /// my_graph.add(&alice_bob_graph);
    /// ```
    pub fn add(&mut self, graph: &Graph) {
        debug!("Added new graph to Graphs [{}]
            current length: {}",
            self.id,
            self.graphs.len());
        self.graphs.push(graph.clone());
    }
    
    /// Returns a collection of Graps elements that matches the relation 
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
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
    pub fn find_by_relation(&mut self, q: &str) -> Option<Vec<&Graph>> {
        let graphs = self.graphs
            .iter()
            .filter(|grph| grph.relation == q)
            .collect::<Vec<&Graph>>();
        debug!("Founded {} graphs with '{}' relation name", graphs.len(), q);
        Some(graphs)
    }

    /// Returns a Graph that matches with the provided id
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
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
    pub fn find_by_id(&mut self, id: &str) -> Option<&mut Graph> {
        let graph = self.graphs
            .iter_mut()
            .find(|graph| graph.id == id ||
                   graph.from.id == id ||
                   graph.to.id == id);
        debug!("Founded by id: {:#?}", graph);
        graph
    }

    /// Deletes the Graph that matches with the provided id
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
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
    /// assert_eq!(my_graph.graphs.len(), 2);
    ///
    /// my_graph.delete_graph_by_id(alice_bob.id); 
    /// assert_eq!(my_graph.graphs.len(), 1);
    /// ```
    pub fn delete_graph_by_id(&mut self, id: String) {
        let index = self.graphs
            .iter()
            .position(|graph| graph.id == id)
            .unwrap();
        debug!("Delete graph: {}", id);
        self.graphs.remove(index);
    }
    
    /// Updates the Graphs with the provided one
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
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
    /// assert_eq!(my_graphs.graphs.len(), 2);
    /// assert_eq!(my_graphs.graphs[1].relation, "super friends");
    ///
    /// alice_fred_graph.update_relation("besties");
    /// my_graphs.update_graph(&alice_fred_graph);
    /// 
    /// assert_eq!(my_graphs.graphs.len(), 2);
    /// let updated_graph = my_graphs.find_by_id(&alice_fred_graph.id);
    /// assert_eq!(updated_graph.unwrap().relation, "besties");
    /// ```
    pub fn update_graph(&mut self, graph_to_update: &Graph) {
        // TODO: add here something when the index is not found
        debug!("Going to update Graphs with {:#?}", graph_to_update);
        let index = self.graphs
            .iter()
            .position(|graph| graph.id == graph_to_update.id)
            .unwrap();
        debug!("Graph to update found it at index: {index}");
        self.graphs.remove(index);
        self.graphs.push(graph_to_update.clone());
    }

    /// Saves the current Graphs into a file with the Graphs's name
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// 
    ///
    /// let mut my_graph = Graphs::new("friends");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob = Graph::new(&alice, "is friend of", &bob);
    /// my_graph.add(&alice_bob);
    ///
    /// my_graph.persists();
    /// ```
    // XXX: Maybe append is not anymore the best way
    pub fn persists(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{}.grphst", self.name.replace(' ', "_"));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name.clone())?;
        let bytes = bincode::serialize(self)?;
        file.write_all(&bytes)?;
        info!("Current Graphs persisted at {} file with {} bytes written",
            file_name, bytes.len());
        Ok(())
    }

    /// Loads the persisted Graphs on a file
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
    ///
    /// let mut my_graph = Graphs::new("friends");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let alice_bob = Graph::new(&alice, "is friend of", &bob);
    /// my_graph.add(&alice_bob);
    ///
    /// let _ = my_graph.persists();
    ///
    /// let name = my_graph.name;
    /// let loaded_graphs = Graphs::load(&name);
    /// match loaded_graphs {
    ///     Ok(loaded_graphs) => {
    ///         assert_eq!(loaded_graphs.name, name);
    ///         assert_eq!(loaded_graphs.graphs[0].relation, alice_bob.relation);
    ///     },
    ///     Err(_) => panic!(),
    /// }
    /// ```
    pub fn load(name: &str) -> Result<Graphs, Box<dyn Error>> {
        let file_name = format!("{}.grphst", name);
        debug!("Loading persisted file {}", &file_name);
        let mut read_file = File::open(file_name)?;
        let mut buffer = [0; MAX_STACK_SIZE];
        read_file.read(&mut buffer[..])?;
        let readed_graph: Graphs = bincode::deserialize(&buffer)?;
        debug!("Loaded persisted file with {} Graphs", readed_graph.graphs.len());
        Ok(readed_graph)
    }

    /// Returns the current size of Graphs in bytes
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
    ///
    /// let mut my_graphs = Graphs::new("memories");
    /// my_graphs.add(
    ///     &Graph::new(
    ///         &Node::new("Alice"),
    ///         "recalls friendship with",
    ///         &Node::new("Bob")
    ///     )
    /// );
    ///
    /// assert_eq!(my_graphs.graphs.len(), 1);
    /// assert_eq!(my_graphs.memory_usage().unwrap(), 255);
    /// ```
    // TODO: add length
    pub fn memory_usage(&self) -> Result<usize, Box<dyn Error>> {
        let bytes = bincode::serialize(self)?;
        debug!("Graphs [{}] '{}' current size: {} bytes",
            self.id, self.name, bytes.len());
        Ok(bytes.len())
    }
}

