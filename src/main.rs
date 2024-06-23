use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::mem;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
struct Node {
    id: i32,
    name: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
struct Graph {
    id: i32,
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
    fn new(name: String) -> Self;
    fn name(&self) -> String;
}

impl Gruphst for Node {
    fn new(name: String) -> Node {
        Node { name, id: 1 }
    }
    fn name(&self)  -> String {
        self.name.clone()
    }
}

fn main()-> Result<(), Box<dyn Error>> {    
    let file_name = "gruphst.bin";
    let mut file = File::create(&file_name)?;
    
    let node1 = Node {
        id: 0,
        name: String::from("this is an origin node"),
    };
    
    let n = node1.clone();
 
    let node2 = Node {
        name: String::from("this is target node"),
        id: 1
    };

    let graph = Graph {
        relation: String::from("related to moar tingys"),
        id: 2,
        from: node1,
        to: node2,
    };
    let g = graph.clone();
    
    println!("graph: {:#?}", graph);
    println!("mem of node1: {}", mem::size_of_val(&n)); 
    println!("mem of graph: {}", mem::size_of_val(&g)); 

    let bytes = bincode::serialize(&graph)?;
    let written = file.write(&bytes)?;
    println!("{} bytes has been written", written);
    println!("The bytes: {:?}", bytes);
    println!("The size of the bytes: {}", bytes.len());

    println!("----------");

    let mut read_file = File::open(&file_name)?;
    let mut buffer = [0; 99];
    let _ = read_file.read(&mut buffer[..])?;
    println!("The readed bytes: {:?}", buffer);
    let readed_graph: Graph = bincode::deserialize(&buffer)?;

    println!("readed graph: {:#?}", readed_graph);
    
    Ok(())
}

