use std::io::Write;
use std::error::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fs::OpenOptions;

static DATA_FILE: &str = "gruphst.bin";

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Node {
    id: String,
    name: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Graph {
    id: String,
    relation: String,
    from: Node,
    to: Node,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Header {
    version: String,
}

pub trait Gruphst {
    fn name(&self) -> String;
    fn id(&self) -> String;
}

impl Gruphst for Node {
    fn name(&self)  -> String {
        self.name.clone()
    }
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name,
            id: Uuid::new_v4().to_string()
        }
    }
}

impl Graph {
    pub fn new(name: String, from: Node, to: Node) -> Graph {
        Graph { 
            relation: name,
            id: Uuid::new_v4().to_string(),
            from,
            to,
        }
    }
    pub fn persists(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().create(true).append(true).open(&DATA_FILE)?;
        let bytes = bincode::serialize(self)?;
        let written = file.write(&bytes)?;
        #[cfg(debug_assertions)]
        println!("{} bytes has been written", written);
        #[cfg(debug_assertions)]
        println!("The bytes: {:?}", bytes);
        #[cfg(debug_assertions)]
        println!("The size of the bytes: {}", bytes.len());            
        Ok(())
    }
}

impl Gruphst for Graph {
    fn name(&self) -> String {
        self.relation.clone()
    }
    fn id(&self) -> String {
        self.id.clone()
    }
}

