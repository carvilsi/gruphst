use log::info;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

use crate::config::get_max_mem_usage;
use crate::errors::GruPHstError;
use crate::graphs::Graphs;

// TODO: improve this; there is no need to deal with extension and add the possibility to use a custom filename

impl Graphs {
    /// Saves the current Graphs into a file with the Graphs's name
    /// <div class="warning">Deprecated method; will be removed on ver. 1.0.0</div>
    /// 
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs};
    ///  
    /// let edge = Edge::create(
    ///     &Vertex::new("Sauron"),
    ///     "created",
    ///     &Vertex::new("One Ring"));
    /// let mut graphs = Graphs::init_with("Middle-earth", &edge);
    ///
    /// // will write a file called 'Middle-earth.grphst' with
    /// // the content of the graphs
    /// graphs.persists();
    /// ```
    #[deprecated(since = "0.15.0", note = "please, for good, use `save` method instead")]
    pub fn persists(&self) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{}.grphst", self.get_label().replace(' ', "_"));
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_name.clone())?;
        let bytes = bincode::serialize(self)?;
        file.write_all(&bytes)?;
        info!("Current Graphs persisted at {} file with {} bytes written", file_name, bytes.len());
        Ok(())
    }

    /// Saves the current Graphs into a file with the Graphs's name
    /// or in the provided path and failename
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs};
    ///  
    /// let edge = Edge::create(
    ///     &Vertex::new("Sauron"),
    ///     "created",
    ///     &Vertex::new("One Ring"));
    /// let mut graphs = Graphs::init_with("Middle-earth", &edge);
    ///
    /// // will write a file called 'Middle-earth.grphst' with
    /// // the content of the graphs
    /// graphs.save(None);
    /// ```
    pub fn save(&self, file_path: Option<&str>) -> Result<(), Box<dyn Error>> {
        let file_name = match file_path {
            Some(fp) => format!("{}{}.grphst", fp, self.get_label().replace(' ', "_")),
            None => format!("{}.grphst", self.get_label().replace(' ', "_")),
        };
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_name.clone())?;
        let bytes = bincode::serialize(self)?;
        file.write_all(&bytes)?;
        info!("Current Graphs persisted at {} file with {} bytes written", file_name, bytes.len());
        Ok(())
    }

    /// Loads the persisted Graphs on a file
    /// # Examples
    /// ```rust
    /// use gruphst::{edge::Edge, vertex::Vertex, graphs::Graphs};
    ///
    /// let edge = Edge::create(
    ///     &Vertex::new("Sauron"),
    ///     "created",
    ///     &Vertex::new("One Ring"));
    /// let mut graphs = Graphs::init_with("Middle-earth", &edge);
    /// graphs.save(None);
    ///
    /// let loaded_graphs = Graphs::load("Middle-earth.grphst").unwrap();
    /// ```
    pub fn load(file_name: &str) -> Result<Graphs, Box<dyn Error>> {
        let read_file = File::open(file_name)?;
        let file_size: usize = read_file.metadata().unwrap().len().try_into().unwrap();
        let mut reader = BufReader::with_capacity(file_size, read_file);
        reader.fill_buf()?;
        let max_mem = get_max_mem_usage();
        // checks if trying to load a file over the limit of memory
        if reader.buffer().len() > max_mem {
            return Err(GruPHstError::PersistenceFile.into());
        }
        let readed_graph: Graphs = bincode::deserialize(reader.buffer())?;
        Ok(readed_graph)
    }
}
