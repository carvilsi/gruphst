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
use log::{ debug, info, error };
use std::collections::HashMap;

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
    attr: HashMap<String, String>,
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
    pub fn new(name: &str) -> Self {
        let node = Node {
            name: String::from(name),
            id: Uuid::new_v4().to_string(),
            attr: HashMap::new(),
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

    pub fn set_attr<T>(
        &mut self,
        attr_k: &str,
        attr_v: T)
    where T: std::fmt::Display,
    {
        self.attr.insert(attr_k.to_string(), attr_v.to_string());
    }

    pub fn get_attr(
        &self,
        attr_k: &str,
    ) -> &String {
        let res = self.attr.get(attr_k);
        res.unwrap()
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
    pub fn new(from: &Node, relation: &str, to: &Node) -> Self {
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

/// Represents stats data from the Graphs
pub struct GraphsStats <'a> {
    /// memory used by Graphs in bytes
    pub mem: usize,
    /// length of the Grpahs
    pub len: usize,
    /// name of the Graph
    pub name: &'a str,
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
    pub fn new(name: &str) -> Self {
        let graphs = Graphs {
            name: String::from(name), 
            id: Uuid::new_v4().to_string(),
            graphs: vec![],
        };
        debug!("Created new Graphs: {:#?}", graphs);
        graphs
    }

    /// Retrieves the length of the Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{ Graphs, Node, Graph };
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
        debug!("Requested length for graphs, current length: {}",
            self.graphs.len());
        self.graphs.len()
    }

    /// Checks if the Graphs is empty
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{ Graphs, Node, Graph };
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
    /// use gruphst::{ Graphs, Node, Graph };
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
            self.len());
        self.graphs.push(graph.clone());
    }
    
    /// Returns a collection of Graps elements that matches the relation 
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{ Graphs, Node, Graph };
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
    pub fn find_by_relation(
        &mut self,
        relation_name: &str
    ) -> Result<Vec<&Graph>, &'static str> {
        let graphs = self.graphs
            .iter()
            .filter(|grph| grph.relation == relation_name)
            .collect::<Vec<&Graph>>();
        if !graphs.is_empty() {
            debug!(
                "Founded {} graphs with '{}' relation name",
               graphs.len(), relation_name);
            Ok(graphs)
        } else {
            error!("Any graph found for relation: {}", relation_name);
            Err("Any graph found for relation")
        }
    }

    /// Returns a Graph that provided id matches with Graph, or From, To Nodes 
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{ Graphs, Node, Graph };
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
    pub fn find_by_id(
        &mut self,
        id: &str
    ) -> Result<&mut Graph, &'static str> {
        let graph = self.graphs
            .iter_mut()
            .find(|graph| graph.id == id ||
                   graph.from.id == id ||
                   graph.to.id == id);
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
    /// use gruphst::{ Graphs, Node, Graph };
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
    /// assert_eq!(my_graph.len(), 2);
    ///
    /// my_graph.delete_graph_by_id(alice_bob.id); 
    /// assert_eq!(my_graph.len(), 1);
    /// ```
    pub fn delete_graph_by_id(
        &mut self,
        id: String,
    ) -> Result<(), &'static str> {
        let index = self.graphs
            .iter()
            .position(|graph| graph.id == id);
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
    /// use gruphst::{ Graphs, Node, Graph };
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
    pub fn update_graph(
        &mut self,
        graph_to_update: &Graph
    ) -> Result<(), &'static str> {
        debug!("Going to update Graphs with {:#?}", graph_to_update);
        let index = self.graphs
            .iter()
            .position(|graph| graph.id == graph_to_update.id);
        if index.is_some() {
            let i = index.unwrap();
            self.graphs.remove(i);
            debug!("Graph to update found it at index: {i}");
            self.graphs.push(graph_to_update.clone());
            Ok(())
        } else {
            error!("Graph to update with id: [{}] not found",
                graph_to_update.id);
            Err("Graph not found")
        }
    }

    /// Saves the current Graphs into a file with the Graphs's name
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{ Graphs, Node, Graph };
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
    pub fn persists(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{}.grphst", self.name.replace(' ', "_"));
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
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
    /// use gruphst::{ Graphs, Node, Graph };
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
    /// let file_name = format!("{}.grphst", name);
    /// let loaded_graphs = Graphs::load(&file_name);
    /// match loaded_graphs {
    ///     Ok(loaded_graphs) => {
    ///         assert_eq!(loaded_graphs.name, name);
    ///         assert_eq!(loaded_graphs.graphs[0].relation, alice_bob.relation);
    ///     },
    ///     Err(_) => panic!(),
    /// }
    /// ```
    pub fn load(file_name: &str) -> Result<Graphs, Box<dyn Error>> {
        debug!("Loading persisted file {}", &file_name);
        let mut read_file = File::open(file_name)?;
        // TODO: change this fixed size buffer reading
        // to something more flexible.
        let mut buffer = [0; MAX_STACK_SIZE];
        read_file.read(&mut buffer[..])?;
        let readed_graph: Graphs = bincode::deserialize(&buffer)?;
        debug!("Loaded persisted file with {} Graphs", readed_graph.len());
        Ok(readed_graph)
    }

    /// Returns the current size of Graphs in bytes
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{ Graphs, Node, Graph };
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
    /// let stats = my_graphs.stats().unwrap();
    /// assert_eq!(stats.mem, 271);
    /// assert_eq!(stats.len, 1);
    /// assert_eq!(stats.name, "memories");
    /// ```
    pub fn stats(&self) -> Result<GraphsStats, Box<dyn Error>> {
        let bytes = bincode::serialize(self)?;
        debug!("Graphs [{}] '{} stats:'
            current size: {} bytes
            current length: {}",
            self.id, self.name,
            bytes.len(),
            self.len());
        let stats = GraphsStats {
            mem: bytes.len(),
            len: self.len(),
            name: &self.name,
        };
        Ok(stats)
    }
}

