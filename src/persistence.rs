use log::{debug, info};
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use dotenv::dotenv;
use std::io::prelude::*;
use std::io::BufReader;

use crate::graphs::Graphs;

const GRUPHST_MAX_MEM_USAGE: &str = "GRUPHST_MAX_MEM_USAGE";
const DEFAULT_GRUPHST_MAX_MEM_USAGE: usize = 25 * 1024 * 1024;

impl Graphs {
    /// Saves the current Graphs into a file with the Graphs's name
    ///
    /// # Examples
    /// ```rust
    /// use gruphst::node::Node;
    /// use gruphst::graph::Graph;
    /// use gruphst::graphs::Graphs;
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
        // reading limit of memory usage 
        dotenv().ok();
        let max_mem: usize = match dotenv::var(GRUPHST_MAX_MEM_USAGE) {
            Ok(value) => {
                let mut max = value.parse().unwrap();
                debug!("max_mem usage set to {} MB", max);
                max = max * 1024 * 1024;
                max
            },
            Err(_) => {
                debug!("using default max_mem usage {}", DEFAULT_GRUPHST_MAX_MEM_USAGE); 
                DEFAULT_GRUPHST_MAX_MEM_USAGE
            },
        };
        debug!("Loading persisted file {}", &file_name);
        let read_file = File::open(file_name)?;
        let mut reader = BufReader::new(read_file);
        reader.fill_buf()?;
        // checks if trying to load a file over the limit of memory
        if reader.buffer().len() > max_mem {
            return Err("Persisted file excedes max memory usage, check GRUPHST_MAX_MEM_USAGE var".into());
        }
        let readed_graph: Graphs = bincode::deserialize(reader.buffer())?;
        debug!("Loaded persisted file with {} Graphs", readed_graph.len());
        Ok(readed_graph)
    }
}

