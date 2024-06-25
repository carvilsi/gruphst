//! Gruphst
//!
//! An in-memory graph database.
//! 
//! Possible to persists on file.

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
    id: String,
    /// And a name
    name: String,
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
}

/// Representation of a Graph, relating two nodes
#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Graph {
    /// A Graph has an uuid
    id: String,
    /// A name fot the relation 
    relation: String,
    /// Origin node
    from: Node,
    /// Target node
    to: Node,
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
    /// let alice_bob_graph = Graph::new(alice, String::from("friend of"), bob);
    /// ```
    pub fn new(from: Node, name: String, to: Node) -> Graph {
        Graph { 
            relation: name,
            id: Uuid::new_v4().to_string(),
            from,
            to,
        }
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
    /// let alice_bob_graph = Graph::new(alice, String::from("friend of"), bob);
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
    /// let alice_bob_graph = Graph::new(alice, String::from("friend of"), bob);
    /// let bob_node = alice_bob_graph.to();
    /// ```
    pub fn to(&self) -> &Node {
        &self.to
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
    /// let alice_bob_graph = Graph::new(alice, String::from("friend of"), bob);
    /// let mut my_graph = Graphs::new(String::from("my_graph"));
    /// my_graph.add(alice_bob_graph);
    /// ```
    pub fn add(&mut self, graph: Graph) {
        self.graphs.push(graph);
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
    /// let alice_bob_graph = Graph::new(alice, String::from("friend of"), bob);
    /// let mut my_graph = Graphs::new(String::from("my_graph"));
    /// my_graph.add(alice_bob_graph);
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
    /// let alice_c = alice.clone();
    /// let bob = Node::new("Bob".to_string());
    /// let bob_c = bob.clone();
    /// let alice_bob = Graph::new(alice, String::from("is friend of"), bob);
    /// my_graph.add(alice_bob.clone());
    ///
    /// let alice_fred =
    ///     Graph::new(
    ///         alice_c,
    ///         String::from("is firend of"),
    ///         Node::new("Fred".to_string())
    ///     );
    /// my_graph.add(alice_fred);
    /// 
    /// let bob_node_id = bob_c.id(); 
    /// let res = my_graph.find_by_id(&bob_node_id);
    /// assert_eq!(res.unwrap().to().id(), bob_node_id);
    /// ```
    pub fn find_by_id(&mut self, id: &str) -> Option<&mut Graph> {
        for graph in self.graphs.iter_mut() {
            if graph.id == id ||
               graph.from.id == id ||
               graph.to.id == id 
            {
                return Some(graph);
            }
        }
        None
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
    /// let alice_c = alice.clone();
    /// let bob = Node::new("Bob".to_string());
    /// let alice_bob = Graph::new(alice, String::from("is friend of"), bob);
    /// my_graph.add(alice_bob.clone());
    ///
    /// let alice_fred =
    ///     Graph::new(
    ///         alice_c,
    ///         String::from("is firend of"),
    ///         Node::new("Fred".to_string())
    ///     );
    /// my_graph.add(alice_fred);
    ///
    /// assert_eq!(my_graph.graphs.len(), 2);
    ///
    /// my_graph.delete(alice_bob.id()); 
    /// assert_eq!(my_graph.graphs.len(), 1);
    /// ```
    pub fn delete(&mut self, id: &str) {
        let index = self.graphs
            .iter()
            .position(|graph| graph.id == id)
            .unwrap();
        self.graphs.remove(index);
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
    /// let alice_c = alice.clone();
    /// let bob = Node::new("Bob".to_string());
    /// let alice_bob = Graph::new(alice, String::from("is friend of"), bob);
    /// my_graph.add(alice_bob);
    ///
    /// my_graph.persists();
    /// ```
    pub fn persists(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{}.grphst", self.name());
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)?;
        let bytes = bincode::serialize(self)?;
        #[cfg(debug_assertions)]
        println!("The size of the bytes: {}", bytes.len());            
        file.write(&bytes)?;
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
    /// let alice_c = alice.clone();
    /// let bob = Node::new("Bob".to_string());
    /// let alice_bob = Graph::new(alice, String::from("is friend of"), bob);
    /// my_graph.add(alice_bob.clone());
    ///
    /// let _ = my_graph.persists();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let n = Node::new(String::from("Node 1"));
        assert_eq!(n.name(), "Node 1");
    }

    #[test]
    fn create_graph() {
        let node1 = Node::new("a node".to_string());
        let node2 = Node::new("b node".to_string());
        let graph = Graph::new(node1, "relation a-b".to_string(), node2);
        assert_eq!(graph.relation, "relation a-b");
        assert_eq!(graph.name(), "relation a-b");
        assert_eq!(graph.from.name, "a node");
        assert_eq!(graph.to.name, "b node");
    }

    #[test]
    fn find_in_graphs() {
        let mut gru = Graphs::new("graphs-a".to_string());
        assert_eq!(gru.name(), "graphs-a");

        let node1 = Node::new("a node".to_string());
        let n1 = node1.clone();
        let node2 = Node::new("b node".to_string());
        let graph1 = Graph::new(node1, "friend of".to_string(), node2);
        gru.add(graph1);
        assert_eq!(gru.graphs.len(), 1);

        let node3 = Node::new("c node".to_string());
        let node4 = Node::new("d node".to_string());
        let graph2 = Graph::new(node3, "knows".to_string(), node4);
        gru.add(graph2);
        assert_eq!(gru.graphs.len(), 2);

        let mut res_graphs= gru.find_by_relation("knows").unwrap();
        assert_eq!(res_graphs.len(), 1);
        assert_eq!(res_graphs[0].name(), "knows");

        let node1_id = n1.id();
        let res = gru.find_by_id(&node1_id);
        assert_eq!(res.unwrap().from().id(), node1_id);

        let node5 = Node::new("e node".to_string());
        let graph3 = Graph::new(n1, "friend of".to_string(), node5);
        gru.add(graph3);

        res_graphs = gru.find_by_relation("friend of").unwrap();
        assert_eq!(res_graphs.len(), 2);
        assert_eq!(res_graphs[0].name(), "friend of");
        assert_eq!(res_graphs[1].name(), "friend of");
    }
    
    #[test]
    fn persistence() {
        let mut gru = Graphs::new("graphs-a".to_string());
        let node1 = Node::new("a node".to_string());
        let node2 = Node::new("b node".to_string());
        let graph1 = Graph::new(node1, "relation a-b".to_string(), node2);
        gru.add(graph1.clone());

        let node3 = Node::new("c node".to_string());
        let node4 = Node::new("d node".to_string());
        let graph2 = Graph::new(node3, "relation c-d".to_string(), node4);
        gru.add(graph2.clone());

        let _ = gru.persists();

        let name = gru.name();
        let grphs = Graphs::load(name);
        match grphs {
            Ok(grphs) => {
                assert_eq!(grphs.name(), name);
                assert_eq!(grphs.graphs[0].name(), graph1.name());
                assert_eq!(grphs.graphs[1], graph2);
            },
            Err(_) => panic!(),
        }
    }

    #[test]
    fn delete_from_graph() {
        let mut my_graph = Graphs::new("friends".to_string());
        let alice = Node::new("Alice".to_string());
        let alice_c = alice.clone();
        let bob = Node::new("Bob".to_string());
        let alice_bob = Graph::new(alice, String::from("is friend of"), bob);
        my_graph.add(alice_bob.clone());

        let alice_fred =
            Graph::new(
                alice_c,
                String::from("is firend of"),
                Node::new("Fred".to_string())
            );
        my_graph.add(alice_fred);

        assert_eq!(my_graph.graphs.len(), 2);

        my_graph.delete(alice_bob.id()); 
        assert_eq!(my_graph.graphs.len(), 1);
    }
}
