use log::debug;
use std::error::Error;
use crate::graphs::Graphs;

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
