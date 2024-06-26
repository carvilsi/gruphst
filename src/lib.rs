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

const MAX_STACK_SIZE: usize = 10000;

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
    /// let node = Node::new(String::from("alice node"));
    /// ```
    pub fn new(name: String) -> Node {
        Node {
            name,
            id: Uuid::new_v4().to_string()
        }
    }

    /// Updates the name of the Node
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use crate::gruphst::Gruphst;
    ///
    /// let mut node = Node::new(String::from("alice node"));
    /// assert_eq!(node.name(), "alice node");
    /// node.update_name("just alice");
    /// assert_eq!(node.name(), "just alice");
    /// ```
    pub fn update_name(&mut self, name: &str) {
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
    /// let alice = Node::new(String::from("Alice"));
    /// let bob = Node::new(String::from("Bob"));
    /// let alice_bob_graph = Graph::new(&alice, String::from("friend of"), &bob);
    /// ```
    pub fn new(from: &Node, name: String, to: &Node) -> Graph {
        Graph { 
            relation: name,
            id: Uuid::new_v4().to_string(),
            from: from.clone(),
            to: to.clone(),
        }
    }
    
    /// Retrieves the relation value for the Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// use crate::gruphst::Gruphst;
    ///
    /// let alice = Node::new(String::from("Alice"));
    /// let bob = Node::new(String::from("Bob"));
    /// let mut alice_bob_graph = Graph::new(&alice, String::from("friend of"), &bob);
    /// 
    /// assert_eq!(alice_bob_graph.relation(), "friend of");
    /// ```
    pub fn relation(&self) -> &String {
        self.name()
    }

    /// Updates the relation for the Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// use crate::gruphst::Gruphst;
    ///
    /// let alice = Node::new(String::from("Alice"));
    /// let bob = Node::new(String::from("Bob"));
    /// let mut alice_bob_graph = Graph::new(&alice, String::from("friend of"), &bob);
    /// 
    /// assert_eq!(alice_bob_graph.name(), "friend of");
    ///
    /// alice_bob_graph.update_relation("best friends");
    /// assert_eq!(alice_bob_graph.name(), "best friends");
    /// ```
    pub fn update_relation(&mut self, relation: &str) {
        self.relation = relation.to_string();
    }
    
    /// Gets the "from" Node from Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    ///
    /// let alice = Node::new(String::from("Alice"));
    /// let bob = Node::new(String::from("Bob"));
    /// let alice_bob_graph = Graph::new(&alice, String::from("friend of"), &bob);
    /// let alice_node = alice_bob_graph.from();
    /// ```
    pub fn from(&self) -> &Node {
        &self.from
    }

    /// Gets the "to" Node from Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    ///
    /// let alice = Node::new(String::from("Alice"));
    /// let bob = Node::new(String::from("Bob"));
    /// let alice_bob_graph = Graph::new(&alice, String::from("friend of"), &bob);
    /// let bob_node = alice_bob_graph.to();
    /// ```
    pub fn to(&self) -> &Node {
        &self.to
    }

    /// Updates the "from" node in Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// use crate::gruphst::Gruphst;
    ///
    /// let mut alice_node = Node::new(String::from("alice node"));
    /// let bob_node = Node::new("bob node".to_string());
    /// let mut graph = Graph::new(&alice_node, "best friends".to_string(), &bob_node);
    /// assert_eq!(graph.from().name(), "alice node");
    /// assert_eq!(graph.to().name(), "bob node");
    /// alice_node.update_name("alice");
    /// graph.update_from(&alice_node);
    /// assert_eq!(graph.from().name(), "alice");
    /// ```
    pub fn update_from(&mut self, from_node: &Node) {
        self.from = from_node.clone();
    }

    /// Updates the "to" node in Graph
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// use crate::gruphst::Gruphst;
    ///
    /// let alice_node = Node::new(String::from("alice node"));
    /// let bob_node = Node::new("bob node".to_string());
    /// let mut graph = Graph::new(
    ///     &alice_node,
    ///     "best friends".to_string(),
    ///     &bob_node);
    /// assert_eq!(graph.from().name(), "alice node");
    /// assert_eq!(graph.to().name(), "bob node");
    /// let fred_node = Node::new("fred node".to_string());
    /// graph.update_to(&fred_node);
    /// assert_eq!(graph.to().name(), "fred node");
    /// assert_ne!(graph.to().id(), bob_node.id());
    /// ```
    pub fn update_to(&mut self, to_node: &Node) {
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
    /// let my_graph = Graphs::new(String::from("my_graph"));
    /// ```
    pub fn new(name: String) -> Graphs {
        Graphs {
            name, 
            id: Uuid::new_v4().to_string(),
            graphs: vec![],
        }
    }
    
    /// Updates the name of the Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use crate::gruphst::Gruphst;
    ///
    /// let mut my_graph = Graphs::new(String::from("my_graph"));
    /// assert_eq!(my_graph.name(), "my_graph");
    ///
    /// my_graph.update_name("graphy");
    /// assert_eq!(my_graph.name(), "graphy");
    /// ```
    pub fn update_name(&mut self, name: &str) {
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
    /// let alice = Node::new(String::from("Alice"));
    /// let bob = Node::new(String::from("Bob"));
    /// let alice_bob_graph = Graph::new(&alice, String::from("friend of"), &bob);
    /// let mut my_graph = Graphs::new(String::from("my_graph"));
    /// my_graph.add(&alice_bob_graph);
    /// ```
    pub fn add(&mut self, graph: &Graph) {
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
    /// let alice = Node::new(String::from("Alice"));
    /// let bob = Node::new(String::from("Bob"));
    /// let alice_bob_graph = Graph::new(&alice, String::from("friend of"), &bob);
    /// let mut my_graph = Graphs::new(String::from("my_graph"));
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
        Some(graphs)
    }

    /// Returns a Graph that matches with the provided id
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// use crate::gruphst::Gruphst;
    ///
    /// let mut my_graph = Graphs::new("friends".to_string());
    /// let alice = Node::new("Alice".to_string());
    /// let bob = Node::new("Bob".to_string());
    /// let alice_bob = Graph::new(&alice, String::from("is friend of"), &bob);
    /// my_graph.add(&alice_bob);
    ///
    /// let alice_fred =
    ///     Graph::new(
    ///         &alice,
    ///         String::from("is firend of"),
    ///         &Node::new("Fred".to_string())
    ///     );
    /// my_graph.add(&alice_fred);
    /// 
    /// let bob_node_id = bob.id(); 
    /// let res = my_graph.find_by_id(&bob_node_id);
    /// assert_eq!(res.unwrap().to().id(), bob_node_id);
    /// ```
    pub fn find_by_id(&mut self, id: &str) -> Option<&mut Graph> {
        self.graphs
            .iter_mut()
            .find(|graph| graph.id == id ||
                   graph.from.id == id ||
                   graph.to.id == id)
    }

    /// Deletes the Graph that matches with the provided id
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// use crate::gruphst::Gruphst;
    ///
    /// let mut my_graph = Graphs::new("friends".to_string());
    /// let alice = Node::new("Alice".to_string());
    /// let bob = Node::new("Bob".to_string());
    /// let alice_bob = Graph::new(&alice, String::from("is friend of"), &bob);
    /// my_graph.add(&alice_bob);
    ///
    /// let alice_fred =
    ///     Graph::new(
    ///         &alice,
    ///         String::from("is firend of"),
    ///         &Node::new("Fred".to_string())
    ///     );
    /// my_graph.add(&alice_fred);
    ///
    /// assert_eq!(my_graph.graphs.len(), 2);
    ///
    /// my_graph.delete_graph_by_id(alice_bob.id()); 
    /// assert_eq!(my_graph.graphs.len(), 1);
    /// ```
    pub fn delete_graph_by_id(&mut self, id: &str) {
        let index = self.graphs
            .iter()
            .position(|graph| graph.id == id)
            .unwrap();
        self.graphs.remove(index);
    }
    
    /// Updates the Graphs with the provided one
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::Graphs;
    /// use gruphst::Node;
    /// use gruphst::Graph;
    /// use crate::gruphst::Gruphst;
    ///
    /// let mut my_graphs = Graphs::new(String::from("my-graphs"));
    ///
    /// let alice_node = Node::new(String::from("Alice"));
    /// let bob_node = Node::new(String::from("Bob"));
    /// let alice_bob_graph =
    ///     Graph::new(&alice_node, "best friends".to_string(), &bob_node);
    /// my_graphs.add(&alice_bob_graph);
    ///
    /// let fred_node = Node::new(String::from("Fred"));
    /// let mut alice_fred_graph = 
    ///     Graph::new(&alice_node, "super friends".to_string(), &fred_node);
    /// my_graphs.add(&alice_fred_graph);
    ///
    /// assert_eq!(my_graphs.graphs.len(), 2);
    /// assert_eq!(my_graphs.graphs[1].relation(), "super friends");
    ///
    /// alice_fred_graph.update_relation("besties");
    /// my_graphs.update_graph(&alice_fred_graph);
    /// 
    /// assert_eq!(my_graphs.graphs.len(), 2);
    /// let updated_graph = my_graphs.find_by_id(alice_fred_graph.id());
    /// assert_eq!(updated_graph.unwrap().relation(), "besties");
    /// ```
    pub fn update_graph(&mut self, graph_to_update: &Graph) {
        let index = self.graphs
            .iter()
            .position(|graph| graph.id == graph_to_update.id)
            .unwrap();
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
    /// use crate::gruphst::Gruphst;
    ///
    /// let mut my_graph = Graphs::new("friends".to_string());
    /// let alice = Node::new("Alice".to_string());
    /// let bob = Node::new("Bob".to_string());
    /// let alice_bob = Graph::new(&alice, String::from("is friend of"), &bob);
    /// my_graph.add(&alice_bob);
    ///
    /// my_graph.persists();
    /// ```
    pub fn persists(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{}.grphst", self.name().replace(' ', "_"));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name.clone())?;
        let bytes = bincode::serialize(self)?;
        #[cfg(debug_assertions)]
        println!("The size of the bytes: {}", bytes.len());            
        file.write_all(&bytes)?;
        println!("Current Graphs persisted at {} file with {} bytes written",
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
    /// use crate::gruphst::Gruphst;
    ///
    /// let mut my_graph = Graphs::new("friends".to_string());
    /// let alice = Node::new("Alice".to_string());
    /// let bob = Node::new("Bob".to_string());
    /// let alice_bob = Graph::new(&alice, String::from("is friend of"), &bob);
    /// my_graph.add(&alice_bob);
    ///
    /// let _ = my_graph.persists();
    ///
    /// let name = my_graph.name();
    /// let loaded_graphs = Graphs::load(name);
    /// match loaded_graphs {
    ///     Ok(loaded_graphs) => {
    ///         assert_eq!(loaded_graphs.name(), name);
    ///         assert_eq!(loaded_graphs.graphs[0].name(), alice_bob.name());
    ///     },
    ///     Err(_) => panic!(),
    /// }
    /// ```
    pub fn load(name: &str) -> Result<Graphs, Box<dyn Error>> {
        let file_name = format!("{}.grphst", name);
        let mut read_file = File::open(file_name)?;
        let mut buffer = [0; MAX_STACK_SIZE];
        read_file.read(&mut buffer[..])?;
        let readed_graph: Graphs = bincode::deserialize(&buffer)?;
        Ok(readed_graph)
    }
}

pub trait Gruphst {
    fn name(&self) -> &String;
    fn id(&self) -> &String;
}

impl Gruphst for Node {
    fn name(&self)  -> &String {
        &self.name
    }
    fn id(&self) -> &String {
        &self.id
    }
}

impl Gruphst for Graph {
    fn name(&self) -> &String {
        &self.relation
    }
    fn id(&self) -> &String {
        &self.id
    }
}

impl Gruphst for Graphs {
    fn name(&self)  -> &String {
        &self.name
    }
    fn id(&self) -> &String {
        &self.id
    }
}


