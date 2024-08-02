use log::{debug, error};

use crate::vertex::Vertex;
use crate::graphs::Graphs;
use crate::edge::Edge;
use crate::CUREdgeVertex;

impl Graphs {
    /// Returns a collection of Graps elements that matches the relation
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs, *};
    ///
    /// let alice = Edge::new("Alice");
    /// let bob = Edge::new("Bob");
    /// let alice_bob_graph = Vertex::create(&alice, "friend of", &bob);
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add_graph(&alice_bob_graph, None);
    ///
    /// let fred = Edge::new("Fred");
    /// my_graph.add_graph(&Vertex::create(&fred, "relative", &bob), None);
    ///
    /// let result_graph = my_graph.find_by_relation("friend of", None).unwrap();
    /// assert_eq!(result_graph.len(), 1);
    /// assert_eq!(result_graph[0].get_relation(), "friend of");
    ///
    /// let res_graph = my_graph.find_by_relation("relative", None).unwrap();
    /// assert_eq!(res_graph.len(), 1);
    /// assert_eq!(res_graph[0].get_relation(), "relative");
    /// ```
    pub fn find_by_relation(
        &mut self,
        relation_name: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.get_relation() == relation_name)
                .collect::<Vec<&Vertex>>();
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
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs, *};
    ///
    /// let alice = Edge::new("Alice");
    /// let bob = Edge::new("Bob");
    /// let alice_bob_graph = Vertex::create(&alice, "friend of", &bob);
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add_graph(&alice_bob_graph, None);
    ///
    /// let fred = Edge::new("Fred");
    /// my_graph.add_graph(&Vertex::create(&fred, "relative", &bob), None);
    ///
    /// let relations = vec!["friend of", "relative", "knows"];
    /// let result_graph = my_graph.find_by_relations(relations, None).unwrap();
    /// assert_eq!(result_graph.len(), 2);
    /// assert_eq!(result_graph[0].get_relation(), "friend of");
    /// assert_eq!(result_graph[1].get_relation(), "relative");
    /// ```
    pub fn find_by_relations(
        &mut self,
        relations: Vec<&str>,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| relations.contains(&grph.get_relation().as_str()))
                .collect::<Vec<&Vertex>>();
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

    /// Returns a collection of graphs that matches an attribute edge by key
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs, *};
    ///
    /// let mut alice = Edge::new("Alice");
    /// let mut bob = Edge::new("Bob");
    /// alice.set_attr("address", "Elm street");
    /// alice.set_attr("phone", "555-555");
    /// alice.set_attr("age", 25);
    /// bob.set_attr("age", 25);
    ///
    /// let alice_bob_graph = Vertex::create(&alice, "friend of", &bob);
    /// let bob_alice_graph = Vertex::create(&bob, "best friend", &alice);
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add_graph(&alice_bob_graph, None);
    /// my_graph.add_graph(&bob_alice_graph, None);
    ///
    /// let mut fred = Edge::new("Fred");
    /// fred.set_attr("room", 5);
    /// my_graph.add_graph(&Vertex::create(&fred, "colege", &bob), None);
    /// my_graph.add_graph(&Vertex::create(&fred, "friend of", &alice), None);
    ///
    /// let graphs_result = my_graph.has_graph_edge_attr("room", None).unwrap();
    ///
    /// assert_eq!(graphs_result.len(), 2);
    /// ```
    pub fn has_graph_edge_attr(
        &mut self,
        attr_k: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.has_edge_attr(attr_k))
                .collect::<Vec<&Vertex>>();
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

    /// Returns a collection of graphs like an attribute edge by key
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs, *};
    ///
    /// let mut alice = Edge::new("Alice");
    /// let mut bob = Edge::new("Bob");
    /// alice.set_attr("address", "Elm street");
    /// alice.set_attr("phone", "555-555");
    /// alice.set_attr("age", 25);
    /// bob.set_attr("age", 25);
    ///
    /// let alice_bob_graph = Vertex::create(&alice, "friend of", &bob);
    /// let bob_alice_graph = Vertex::create(&bob, "best friend", &alice);
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add_graph(&alice_bob_graph, None);
    /// my_graph.add_graph(&bob_alice_graph, None);
    ///
    /// let mut fred = Edge::new("Fred");
    /// fred.set_attr("room", 5);
    /// my_graph.add_graph(&Vertex::create(&fred, "colege", &bob), None);
    /// my_graph.add_graph(&Vertex::create(&fred, "friend of", &alice), None);
    ///
    /// let graphs_result = my_graph.like_graph_edge_attr("rO", None).unwrap();
    ///
    /// assert_eq!(graphs_result.len(), 2);
    /// ```
    pub fn like_graph_edge_attr(
        &mut self,
        attr_k: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.like_edge_attr(attr_k))
                .collect::<Vec<&Vertex>>();
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

    /// Returns a collection of graphs that matches an attribute
    /// and value
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs, *};
    ///
    /// let mut alice = Edge::new("Alice");
    /// let mut bob = Edge::new("Bob");
    /// alice.set_attr("address", "Elm street");
    /// alice.set_attr("phone", "555-555");
    /// alice.set_attr("age", 25);
    /// bob.set_attr("age", 42);
    ///
    /// let alice_bob_graph = Vertex::create(&alice, "friend of", &bob);
    /// let bob_alice_graph = Vertex::create(&bob, "best friend", &alice);
    /// let mut my_graph = Graphs::init("my_graph");
    /// my_graph.add_graph(&alice_bob_graph, None);
    /// my_graph.add_graph(&bob_alice_graph, None);
    ///
    /// let mut fred = Edge::new("Fred");
    /// fred.set_attr("room", 5);
    /// my_graph.add_graph(&Vertex::create(&fred, "colege", &bob), None);
    /// my_graph.add_graph(&Vertex::create(&fred, "friend of", &alice), None);
    ///
    /// let graphs_result = my_graph.attr_equals_to("age", 42, None).unwrap();
    ///
    /// assert_eq!(graphs_result.len(), 3);
    /// ```
    // XXX: add a method to find attr on all graphs????
    pub fn attr_equals_to<T>(
        &self,
        attr_k: &str,
        attr_v: T,
        graphs_name: Option<&str>,
    ) -> Result<Vec<&Vertex>, &'static str>
    where
        T: std::fmt::Display + std::clone::Clone,
    {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            let graphs = graphs
                .iter()
                .filter(|grph| grph.equals_edge_attr(attr_k, attr_v.clone()))
                .collect::<Vec<&Vertex>>();
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

    /// Returns a Vertex that provided id matches with Vertex, or From, To edges
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs, *};
    ///
    ///
    /// let mut my_graph = Graphs::init("friends");
    /// let alice = Edge::new("Alice");
    /// let bob = Edge::new("Bob");
    /// let alice_bob = Vertex::create(&alice, "is friend of", &bob);
    /// my_graph.add_graph(&alice_bob, None);
    ///
    /// let alice_fred =
    ///     Vertex::create(&alice, "is firend of", &Edge::new("Fred"));
    /// my_graph.add_graph(&alice_fred, None);
    ///
    /// let bob_edge_id = bob.get_id();
    /// let res = my_graph.find_by_id(&bob_edge_id, None);
    /// assert_eq!(res.unwrap().get_to_edge().get_id(), bob_edge_id);
    /// ```
    pub fn find_by_id(
        &mut self,
        id: &str,
        graphs_name: Option<&str>,
    ) -> Result<&mut Vertex, &'static str> {
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get_mut(&current_graph) {
            let graph = graphs.iter_mut().find(|graph| {
                graph.get_id() == id
                    || graph.get_from_edge().get_id() == id
                    || graph.get_to_edge().get_id() == id
            });
            if graph.is_some() {
                debug!("Founded Vertex by id: {:#?}", graph);
                Ok(graph.unwrap())
            } else {
                error!("Vertex with id [{}] not found", id);
                Err("Vertex not found")
            }
        } else {
            Err("no graphs found at vault")
        }
    }

    /// Find in any graph on vault by id
    pub fn find_by_id_in_graphs(&mut self, id: &str) -> Result<&mut Vertex, &'static str> {
        for (_graph_name, graphs) in self.vault.iter_mut() {
            println!("Tha name: {}", _graph_name);
            let graph = graphs.iter_mut().find(|graph| {
                graph.get_id() == id
                    || graph.get_from_edge().get_id() == id
                    || graph.get_to_edge().get_id() == id
            });
            if graph.is_some() {
                debug!("Founded Vertex by id: {:#?}", graph);
                return Ok(graph.unwrap());
            }
        }
        Err("Vertex not found")
    }

    /// Retrieves all the edges with incoming relation
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs, *};
    ///
    /// let mut my_graphs = Graphs::init("my-graphs");
    ///
    /// let alice = Edge::new("Alice");
    /// let bob = Edge::new("Bob");
    /// let fred = Edge::new("Fred");
    ///
    /// my_graphs.add_graph(&Vertex::create(&alice, "is friend of", &bob), None);
    /// my_graphs.add_graph(&Vertex::create(&bob, "is friend of", &fred), None);
    /// my_graphs.add_graph(&Vertex::create(&alice, "knows", &fred), None);
    ///
    /// let results = my_graphs.has_relation_in("is friend of", None).unwrap();
    ///
    /// assert_eq!(results.len(), 2);
    /// assert_eq!(results[0].get_label(), "Bob");
    /// assert_eq!(results[1].get_label(), "Fred");
    /// ```
    pub fn has_relation_in(
        &self,
        relation_in: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<Edge>, &'static str> {
        let mut relations_in: Vec<Edge> = Vec::new();
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            for graph in graphs {
                if graph.get_relation() == relation_in
                    && !relations_in.contains(&graph.get_to_edge())
                {
                    relations_in.push(graph.get_to_edge().clone());
                }
            }
        } else {
            return Err("no current graph in vault");
        }
        if !relations_in.is_empty() {
            Ok(relations_in)
        } else {
            Err("any edge with relation in")
        }
    }

    /// Retrieves all the edges with outcoming relation
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs, *};
    ///
    /// let mut my_graphs = Graphs::init("my-graphs");
    ///
    /// let alice = Edge::new("Alice");
    /// let bob = Edge::new("Bob");
    /// let fred = Edge::new("Fred");
    ///
    /// my_graphs.add_graph(&Vertex::create(&alice, "is friend of", &bob), None);
    /// my_graphs.add_graph(&Vertex::create(&bob, "is friend of", &fred), None);
    /// my_graphs.add_graph(&Vertex::create(&alice, "knows", &fred), None);
    ///
    /// let results = my_graphs.has_relation_out("is friend of", None).unwrap();
    ///
    /// assert_eq!(results.len(), 2);
    /// assert_eq!(results[0].get_label(), "Alice");
    /// assert_eq!(results[1].get_label(), "Bob");
    /// ```
    pub fn has_relation_out(
        &self,
        relation_out: &str,
        graphs_name: Option<&str>,
    ) -> Result<Vec<Edge>, &'static str> {
        let mut relations_out: Vec<Edge> = Vec::new();
        let current_graph = self.select_graphs_label(graphs_name);
        if let Some(graphs) = self.vault.get(&current_graph) {
            for graph in graphs {
                if graph.get_relation() == relation_out
                    && !relations_out.contains(&graph.get_from_edge())
                {
                    relations_out.push(graph.get_from_edge().clone());
                }
            }
        } else {
            return Err("no current graph in vault");
        }
        if !relations_out.is_empty() {
            Ok(relations_out)
        } else {
            Err("any edge with relation out")
        }
    }
}
