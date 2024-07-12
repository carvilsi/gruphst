use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::graph::Graph;
use crate::node::Node;
use crate::util::graphs_memory_watcher;

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

    // TODO: add test and documentation
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
    /// my_graph.add(&alice_bob_graph);
    /// ```
    // TODO: add possible to add to graphs
    pub fn add(&mut self, graph: &Graph) {
        debug!(
            "Added new graph to Graphs [{}, {}]
            current length: {}",
            self.id,
            self.name,
            self.len()
        );
        if let Some(v) = self.vault.get_mut(self.name.as_str()) {
            v.push(graph.clone());
            graphs_memory_watcher(self);
        } else {
            error!("no graph element {} at vault", self.name);
        }
    }

    /// Retrieves the collection of graphs
    /// the default one or by name
    // TODO: Add a test and documentation
    pub fn get(&self, graphs_name: Option<&str>) -> Result<Vec<Graph>, &'static str> {
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            Ok(graphs.clone())
        } else {
            Err("no graphs found on vault")
        }
    }

    /// Retrieves the length of the Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut graphs = Graphs::init("lengths");
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    ///
    /// graphs.add(&Graph::new(&alice, "friend", &bob));
    /// graphs.add(&Graph::new(&bob, "friend", &alice));
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
    // TODO: add a method to deal with total amount of different Graphs

    /// Checks if the Graphs is empty
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut graphs = Graphs::init("lengths");
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
    /// let mut my_graph = Graphs::init("my_graph");
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
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add(&alice_bob_graph);
    ///
    /// let fred = Node::new("Fred");
    /// my_graph.add(&Graph::new(&fred, "relative", &bob));
    ///
    /// let result_graph = my_graph.find_by_relation("friend of", None).unwrap();
    /// assert_eq!(result_graph.len(), 1);
    /// assert_eq!(result_graph[0].relation, "friend of");
    ///
    /// let res_graph = my_graph.find_by_relation("relative", None).unwrap();
    /// assert_eq!(res_graph.len(), 1);
    /// assert_eq!(res_graph[0].relation, "relative");
    /// ```
    pub fn find_by_relation(
        &mut self,
        relation_name: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Graph>, &'static str> {
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
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
        } else {
            Err("no graphs found on vault")
        }
    }

    /// Returns a collection of Graps elements that matches the relations
    /// in the array
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
    /// my_graph.add(&alice_bob_graph);
    ///
    /// let fred = Node::new("Fred");
    /// my_graph.add(&Graph::new(&fred, "relative", &bob));
    ///
    /// let relations = vec!["friend of", "relative", "knows"];
    /// let result_graph = my_graph.find_by_relations(relations, None).unwrap();
    /// assert_eq!(result_graph.len(), 2);
    /// assert_eq!(result_graph[0].relation, "friend of");
    /// assert_eq!(result_graph[1].relation, "relative");
    /// ```
    pub fn find_by_relations(
        &mut self,
        relations: Vec<&str>,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Graph>, &'static str> {
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| relations.contains(&grph.relation.as_str()))
                .collect::<Vec<&Graph>>();
            if !graphs.is_empty() {
                debug!(
                    "Founded {} graphs with '{:#?}' relations",
                    graphs.len(),
                    relations
                );
                Ok(graphs)
            } else {
                error!("Any graph found for relations: {:#?}", relations);
                Err("Any graph found for relation")
            }
        } else {
            Err("graphs not found on vault")
        }
    }

    /// Returns a collection of graphs that matches an attribute node by key
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut alice = Node::new("Alice");
    /// let mut bob = Node::new("Bob");
    /// alice.set_attr("address", "Elm street");
    /// alice.set_attr("phone", "555-555");
    /// alice.set_attr("age", 25);
    /// bob.set_attr("age", 25);
    ///
    /// let alice_bob_graph = Graph::new(&alice, "friend of", &bob);
    /// let bob_alice_graph = Graph::new(&bob, "best friend", &alice);
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add(&alice_bob_graph);
    /// my_graph.add(&bob_alice_graph);
    ///
    /// let mut fred = Node::new("Fred");
    /// fred.set_attr("room", 5);
    /// my_graph.add(&Graph::new(&fred, "colege", &bob));
    /// my_graph.add(&Graph::new(&fred, "friend of", &alice));
    ///
    /// let graphs_result = my_graph.has_graph_node_attr("room", None).unwrap();
    ///
    /// assert_eq!(graphs_result.len(), 2);
    /// ```
    pub fn has_graph_node_attr(
        &mut self,
        attr_k: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Graph>, &'static str> {
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.has_node_attr(attr_k))
                .collect::<Vec<&Graph>>();
            if !graphs.is_empty() {
                debug!(
                    "Founded {} graphs where an attribute key is '{}'",
                    graphs.len(),
                    attr_k
                );
                Ok(graphs)
            } else {
                error!("Any graph found for attribute: {}", attr_k);
                Err("Any graph found for attribute")
            }
        } else {
            Err("no graphs found on vault")
        }
    }

    /// Returns a collection of graphs like an attribute node by key
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut alice = Node::new("Alice");
    /// let mut bob = Node::new("Bob");
    /// alice.set_attr("address", "Elm street");
    /// alice.set_attr("phone", "555-555");
    /// alice.set_attr("age", 25);
    /// bob.set_attr("age", 25);
    ///
    /// let alice_bob_graph = Graph::new(&alice, "friend of", &bob);
    /// let bob_alice_graph = Graph::new(&bob, "best friend", &alice);
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add(&alice_bob_graph);
    /// my_graph.add(&bob_alice_graph);
    ///
    /// let mut fred = Node::new("Fred");
    /// fred.set_attr("room", 5);
    /// my_graph.add(&Graph::new(&fred, "colege", &bob));
    /// my_graph.add(&Graph::new(&fred, "friend of", &alice));
    ///
    /// let graphs_result = my_graph.like_graph_node_attr("rO", None).unwrap();
    ///
    /// assert_eq!(graphs_result.len(), 2);
    /// ```
    pub fn like_graph_node_attr(
        &mut self,
        attr_k: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Graph>, &'static str> {
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.like_node_attr(attr_k))
                .collect::<Vec<&Graph>>();
            if !graphs.is_empty() {
                debug!(
                    "Founded {} graphs where an attribute key is '{}'",
                    graphs.len(),
                    attr_k
                );
                Ok(graphs)
            } else {
                error!("Any graph found for attribute: {}", attr_k);
                Err("Any graph found for attribute")
            }
        } else {
            Err("no graphs on vault")
        }
    }

    /// Returns a collection of graphs that matches an attribute node by key
    /// and value
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut alice = Node::new("Alice");
    /// let mut bob = Node::new("Bob");
    /// alice.set_attr("address", "Elm street");
    /// alice.set_attr("phone", "555-555");
    /// alice.set_attr("age", 25);
    /// bob.set_attr("age", 42);
    ///
    /// let alice_bob_graph = Graph::new(&alice, "friend of", &bob);
    /// let bob_alice_graph = Graph::new(&bob, "best friend", &alice);
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add(&alice_bob_graph);
    /// my_graph.add(&bob_alice_graph);
    ///
    /// let mut fred = Node::new("Fred");
    /// fred.set_attr("room", 5);
    /// my_graph.add(&Graph::new(&fred, "colege", &bob));
    /// my_graph.add(&Graph::new(&fred, "friend of", &alice));
    ///
    /// let graphs_result = my_graph.attr_equals_to("age", 42, None).unwrap();
    ///
    /// assert_eq!(graphs_result.len(), 3);
    /// ```
    // TODO: add a method to find attr on all graphs
    pub fn attr_equals_to<T>(
        &self,
        attr_k: &str,
        attr_v: T,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Graph>, &'static str>
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.equals_node_attr(attr_k, attr_v.clone()))
                .collect::<Vec<&Graph>>();
            if !graphs.is_empty() {
                debug!(
                    "Founded {} graphs where an attribute key is '{}'",
                    graphs.len(),
                    attr_k
                );
                Ok(graphs)
            } else {
                error!("Any graph found for attribute: {}", attr_k);
                Err("Any graph found for attribute")
            }
        } else {
            Err("no graphs on vault")
        }
    }

    // TODO: add uniq relations for all the graphs doc-test
    pub fn uniq_graph_relations(&self, graphs_name: Option<&str>) -> Vec<&String> {
        let mut uniq_rel = Vec::new();
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            for graph in graphs.iter() {
                uniq_rel.push(&graph.relation);
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

    /// Returns an array with the unique relations in the Graphs
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut my_graph = Graphs::init("my graph");
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
        for graphs in self.vault.values() {
            for graph in graphs.iter() {
                uniq_rel.push(&graph.relation);
            }
            uniq_rel.sort();
            uniq_rel.dedup();
        }
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
    /// let mut my_graph = Graphs::init("friends");
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
    /// let res = my_graph.find_by_id(&bob_node_id, None);
    /// assert_eq!(res.unwrap().to.id, bob_node_id);
    /// ```
    pub fn find_by_id(
        &mut self,
        id: &str,
        graphs_name: Option<&str>,
    ) -> Result<&mut Graph, &'static str> {
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get_mut(&current_graph) {
            let graph = graphs
                .iter_mut()
                .find(|graph| graph.id == id || graph.from.id == id || graph.to.id == id);
            if graph.is_some() {
                debug!("Founded Graph by id: {:#?}", graph);
                Ok(graph.unwrap())
            } else {
                error!("Graph with id [{}] not found", id);
                Err("Graph not found")
            }
        } else {
            Err("no graphs found at vault")
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
    /// let mut my_graph = Graphs::init("friends");
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
    /// my_graphs.add(&alice_bob_graph);
    ///
    /// let fred_node = Node::new("Fred");
    /// let mut alice_fred_graph =
    ///     Graph::new(&alice_node, "super friends", &fred_node);
    /// my_graphs.add(&alice_fred_graph);
    ///
    /// assert_eq!(my_graphs.len(), 2);
    ///
    /// let graphs = my_graphs.get(Some(&my_graphs.name)).unwrap();
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

    /// Retrieves all the nodes with incoming relation
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut my_graphs = Graphs::init("my-graphs");
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let fred = Node::new("Fred");
    ///
    /// my_graphs.add(&Graph::new(&alice, "is friend of", &bob));
    /// my_graphs.add(&Graph::new(&bob, "is friend of", &fred));
    /// my_graphs.add(&Graph::new(&alice, "knows", &fred));
    ///
    /// let results = my_graphs.has_relation_in("is friend of", None).unwrap();
    ///
    /// assert_eq!(results.len(), 2);
    /// assert_eq!(results[0].name, "Bob");
    /// assert_eq!(results[1].name, "Fred");
    /// ```
    pub fn has_relation_in(
        &self,
        relation_in: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<Node>, &'static str> {
        let mut relations_in: Vec<Node> = Vec::new();
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            for graph in graphs {
                if graph.relation == relation_in && !relations_in.contains(&graph.to) {
                    relations_in.push(graph.to.clone());
                }
            }
        } else {
            return Err("no current graph in vault");
        }
        if !relations_in.is_empty() {
            Ok(relations_in)
        } else {
            Err("any node with relation in")
        }
    }

    /// Retrieves all the nodes with outcoming relation
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut my_graphs = Graphs::init("my-graphs");
    ///
    /// let alice = Node::new("Alice");
    /// let bob = Node::new("Bob");
    /// let fred = Node::new("Fred");
    ///
    /// my_graphs.add(&Graph::new(&alice, "is friend of", &bob));
    /// my_graphs.add(&Graph::new(&bob, "is friend of", &fred));
    /// my_graphs.add(&Graph::new(&alice, "knows", &fred));
    ///
    /// let results = my_graphs.has_relation_out("is friend of", None).unwrap();
    ///
    /// assert_eq!(results.len(), 2);
    /// assert_eq!(results[0].name, "Alice");
    /// assert_eq!(results[1].name, "Bob");
    /// ```
    pub fn has_relation_out(
        &self,
        relation_out: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<Node>, &'static str> {
        let mut relations_out: Vec<Node> = Vec::new();
        let current_graph = self.select_graphs_name(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            for graph in graphs {
                if graph.relation == relation_out && !relations_out.contains(&graph.from) {
                    relations_out.push(graph.from.clone());
                }
            }
        } else {
            return Err("no current graph in vault");
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            Err("any node with relation out")
        }
    }

    /// Retrieves the current graphs or returns the option one
    fn select_graphs_name(&self, graphs_name: Option<&str>) -> String {
        let mut current_graph = self.name.clone();
        if let Some(gn) = graphs_name {
            current_graph = gn.to_string();
        }
        current_graph.to_string()
    }
}
