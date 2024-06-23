use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::mem;

use gruphst::Gruphst;
use gruphst::{Node, Graph};

fn main()-> Result<(), Box<dyn Error>> {    
   
    let node1 = Node::new(String::from("Node 1")); 
    let node2 = Node::new(String::from("Node 2")); 
    let n1 = node1.clone();
    let n2 = node2.clone();
    let graph = Graph::new(String::from("related to"), node1, node2);
    let g = graph.clone();
   
    #[cfg(debug_assertions)]
    println!("graph: {:#?}", graph);
    #[cfg(debug_assertions)]
    println!("mem of graph: {}", mem::size_of_val(&g)); 

    graph.persists()?;
    
    #[cfg(debug_assertions)]
    println!("The name of node1: {}", n1.name());
    #[cfg(debug_assertions)]
    println!("The name of node2: {}", n2.name());
    #[cfg(debug_assertions)]
    println!("The name of graph: {}", graph.name());

    #[cfg(debug_assertions)]
    println!("----------");

    let mut read_file = File::open("gruphst.bin")?;
    let mut buffer = [0; 178];
    let _ = read_file.read(&mut buffer[..])?;
    #[cfg(debug_assertions)]
    println!("The readed bytes: {:?}", buffer);
    let readed_graph: Graph = bincode::deserialize(&buffer)?;

    #[cfg(debug_assertions)]
    println!("readed graph: {:#?}", readed_graph);
    
    Ok(())
}

