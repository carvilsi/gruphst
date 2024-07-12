use log::{debug, error};

use crate::graph::Graph;
use crate::graphs::Graphs;
use crate::node::Node;

impl Graphs {
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
    /// my_graph.add_graph(&alice_bob_graph, None);
    ///
    /// let fred = Node::new("Fred");
    /// my_graph.add_graph(&Graph::new(&fred, "relative", &bob), None);
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
    /// my_graph.add_graph(&alice_bob_graph, None);
    ///
    /// let fred = Node::new("Fred");
    /// my_graph.add_graph(&Graph::new(&fred, "relative", &bob), None);
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
    /// my_graph.add_graph(&alice_bob_graph, None);
    /// my_graph.add_graph(&bob_alice_graph, None);
    ///
    /// let mut fred = Node::new("Fred");
    /// fred.set_attr("room", 5);
    /// my_graph.add_graph(&Graph::new(&fred, "colege", &bob), None);
    /// my_graph.add_graph(&Graph::new(&fred, "friend of", &alice), None);
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
    /// my_graph.add_graph(&alice_bob_graph, None);
    /// my_graph.add_graph(&bob_alice_graph, None);
    ///
    /// let mut fred = Node::new("Fred");
    /// fred.set_attr("room", 5);
    /// my_graph.add_graph(&Graph::new(&fred, "colege", &bob), None);
    /// my_graph.add_graph(&Graph::new(&fred, "friend of", &alice), None);
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
    /// my_graph.add_graph(&alice_bob_graph, None);
    /// my_graph.add_graph(&bob_alice_graph, None);
    ///
    /// let mut fred = Node::new("Fred");
    /// fred.set_attr("room", 5);
    /// my_graph.add_graph(&Graph::new(&fred, "colege", &bob), None);
    /// my_graph.add_graph(&Graph::new(&fred, "friend of", &alice), None);
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
    /// my_graph.add_graph(&alice_bob, None);
    ///
    /// let alice_fred =
    ///     Graph::new(&alice, "is firend of", &Node::new("Fred"));
    /// my_graph.add_graph(&alice_fred, None);
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
    /// my_graphs.add_graph(&Graph::new(&alice, "is friend of", &bob), None);
    /// my_graphs.add_graph(&Graph::new(&bob, "is friend of", &fred), None);
    /// my_graphs.add_graph(&Graph::new(&alice, "knows", &fred), None);
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
    /// my_graphs.add_graph(&Graph::new(&alice, "is friend of", &bob), None);
    /// my_graphs.add_graph(&Graph::new(&bob, "is friend of", &fred), None);
    /// my_graphs.add_graph(&Graph::new(&alice, "knows", &fred), None);
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
}
