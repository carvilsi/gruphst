use crate::graphs::Graphs;
use log::{debug, error};
use std::error::Error;

/// Represents stats data from the Graphs
#[derive(Debug)]
pub struct GraphsStats<'a> {
    /// memory used by Graphs in bytes
    pub mem: usize,
    /// length of the Graph's vault
    pub len_graphs: usize,
    /// total graphs
    pub total_graphs: usize,
    /// name of the Graph
    pub name: &'a str,
    /// total attributes
    pub total_attr: usize,
    /// total nodes
    pub total_nodes: usize,
    /// unique relations
    pub uniq_rel: usize,
}

impl Graphs {
    /// Returns stats from Graphs; size in bytes, amount of graph, name, total number of attributes
    /// and total amount of Nodes
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut my_graphs = Graphs::init("memories");
    /// my_graphs.add_graph(
    ///     &Graph::new(
    ///         &Node::new("Alice"),
    ///         "recalls friendship with",
    ///         &Node::new("Bob")
    ///     ), None
    /// );
    /// let mut fred = Node::new("Fred");
    /// fred.set_attr("address", "Elm street");
    /// fred.set_attr("phone", "555-555-555");
    /// fred.set_attr("age", "25");
    ///
    /// my_graphs.add_graph(
    ///     &Graph::new(
    ///         &fred,
    ///         "relative of",
    ///         &Node::new("Coco")
    ///     ), None
    /// );
    ///
    /// let stats = my_graphs.stats().unwrap();
    /// assert_eq!(stats.mem, 572);
    /// assert_eq!(stats.len_graphs, 2);
    /// assert_eq!(stats.name, "memories");
    /// assert_eq!(stats.total_attr, 3);
    /// assert_eq!(stats.total_nodes, 4);
    /// assert_eq!(stats.uniq_rel, 2);
    /// assert_eq!(stats.total_graphs, 1);
    /// ```
    pub fn stats(&self) -> Result<GraphsStats, Box<dyn Error>> {
        let bytes = bincode::serialize(self)?;
        // lets count the amount of attributes in the graph
        let mut attr_counter = 0;
        for (_graph_name, graphs) in self.vault.iter() {
            for graph in graphs {
                attr_counter += graph.from.len_attr();
                attr_counter += graph.to.len_attr();
            }
        }

        let stats = GraphsStats {
            mem: bytes.len(),
            len_graphs: self.len(),
            name: &self.name,
            total_attr: attr_counter,
            total_nodes: self.len() * 2,
            uniq_rel: self.uniq_relations().len(),
            total_graphs: self.vault.len(),
        };
        debug!("Graphs stats: {:#?}", stats);
        Ok(stats)
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

    /// Returns an array with the unique relations in the default Graphs
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
    /// my_graph.add_graph(&Graph::new(&alice, "friend of", &bob), None);
    /// my_graph.add_graph(&Graph::new(&alice, "relative of", &fred), None);
    /// my_graph.add_graph(&Graph::new(&fred, "friend of", &bob), None);
    /// my_graph.add_graph(&Graph::new(&bob, "friend of", &alice), None);
    /// my_graph.add_graph(&Graph::new(&fred, "relative of", &alice), None);
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

    /// Retrieves the length of the Graphs for whole vault
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
    /// graphs.add_graph(&Graph::new(&alice, "friend", &bob), None);
    /// graphs.add_graph(&Graph::new(&bob, "friend", &alice), None);
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

    /// Retrieves the length of vault
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::graphs::Graphs;
    ///
    /// let mut graphs = Graphs::init("graph 0");
    /// assert_eq!(graphs.len_graphs(), 1);
    ///
    /// graphs.new("graph 1");
    /// assert_eq!(graphs.len_graphs(), 2);
    /// ```
    pub fn len_graphs(&self) -> usize {
        self.vault.len()
    }

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
    /// graphs.add_graph(&Graph::new(&alice, "friend", &bob), None);
    /// graphs.add_graph(&Graph::new(&bob, "friend", &alice), None);
    ///
    /// assert!(!graphs.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
