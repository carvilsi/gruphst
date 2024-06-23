use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::mem;
use uuid::Uuid;
use std::fs::OpenOptions;

static DATA_FILE: &str = "gruphst.bin";

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
struct Node {
    id: String,
    name: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
struct Graph {
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

trait Gruphst {
    fn name(&self) -> String;
}

impl Gruphst for Node {
    fn name(&self)  -> String {
        self.name.clone()
    }
}

impl Node {
    fn new(name: String) -> Node {
        Node {
            name,
            id: Uuid::new_v4().to_string()
        }
    }
}

impl Graph {
    fn new(name: String, from: Node, to: Node) -> Graph {
        Graph { 
            relation: name,
            id: Uuid::new_v4().to_string(),
            from,
            to,
        }
    }
    fn persists(&self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().create(true).append(true).open(&DATA_FILE)?;
        let bytes = bincode::serialize(self)?;
        let written = file.write(&bytes)?;
        println!("{} bytes has been written", written);
        println!("The bytes: {:?}", bytes);
        println!("The size of the bytes: {}", bytes.len());            
        Ok(())
    }

}

impl Gruphst for Graph {
    fn name(&self) -> String {
        self.relation.clone()
    }
}

fn main()-> Result<(), Box<dyn Error>> {    
   
    let node1 = Node::new(String::from("Node 1")); 
    let node2 = Node::new(String::from("Node 2")); 
    let n1 = node1.clone();
    let n2 = node2.clone();
    let graph = Graph::new(String::from("related to"), node1, node2);
    let g = graph.clone();
    
    println!("graph: {:#?}", graph);
    println!("mem of graph: {}", mem::size_of_val(&g)); 

    graph.persists()?;
    
    println!("The name of node1: {}", n1.name());
    println!("The name of node2: {}", n2.name());
    println!("The name of graph: {}", graph.name());

    println!("----------");

    let mut read_file = File::open(&DATA_FILE)?;
    let mut buffer = [0; 178];
    let _ = read_file.read(&mut buffer[..])?;
    println!("The readed bytes: {:?}", buffer);
    let readed_graph: Graph = bincode::deserialize(&buffer)?;

    println!("readed graph: {:#?}", readed_graph);
    
    Ok(())
}

