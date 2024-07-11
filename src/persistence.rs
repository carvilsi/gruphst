use log::{debug, info};
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

use crate::config::get_max_mem_usage;
use crate::graphs::Graphs;

impl Graphs {
    /// Saves the current Graphs into a file with the Graphs's name
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
        info!(
            "Current Graphs persisted at {} file with {} bytes written",
            file_name,
            bytes.len()
        );
        Ok(())
    }

    /// Loads the persisted Graphs on a file
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
    /// let _ = my_graph.persists();
    ///
    /// let name = my_graph.name;
    /// let file_name = format!("{}.grphst", name);
    /// let loaded_graphs = Graphs::load(&file_name);
    /// match loaded_graphs {
    ///     Ok(loaded_graphs) => {
    ///         let graphs = loaded_graphs.get(Some(name)).unwrap();
    ///         assert_eq!(loaded_graphs.name, name);
    ///         assert_eq!(graphs[0].relation, alice_bob.relation);
    ///     },
    ///     Err(_) => panic!(),
    /// }
    /// ```
    pub fn load(file_name: &str) -> Result<Graphs, Box<dyn Error>> {
        debug!("Loading persisted file {}", &file_name);
        let read_file = File::open(file_name)?;
        let mut reader = BufReader::new(read_file);
        reader.fill_buf()?;
        let max_mem = get_max_mem_usage();
        // checks if trying to load a file over the limit of memory
        if reader.buffer().len() > max_mem {
            return Err(
                "Persisted file excedes max memory usage, check GRUPHST_MAX_MEM_USAGE var".into(),
            );
        }
        let readed_graph: Graphs = bincode::deserialize(reader.buffer())?;
        debug!("Loaded persisted file with {} Graphs", readed_graph.len());
        Ok(readed_graph)
    }
}
