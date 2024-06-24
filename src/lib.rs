use std::io::Write;
use std::error::Error;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use std::fs::OpenOptions;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Node {
    id: String,
    name: String,
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name,
            id: Uuid::new_v4().to_string()
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Graph {
    id: String,
    relation: String,
    from: Node,
    to: Node,
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
    fn save(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{name}.grphst");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_name)?;
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
    pub fn from(&self) -> &Node {
        &self.from
    }
    pub fn to(&self) -> &Node {
        &self.to
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Graphs {
    pub graphs: Vec<Graph>,
    pub name: String,
    pub id: String,
}

impl Graphs {
    pub fn new(name: String) -> Graphs {
        Graphs {
            name, 
            id: Uuid::new_v4().to_string(),
            graphs: vec![],
        }
    }
    pub fn find_relation(&mut self, q: &str) -> Option<&mut Graph> {
        for graph in self.graphs.iter_mut() {
            if graph.relation == q {
                return Some(graph);
            }
        }
        None
    }
    pub fn find_by_id(&mut self, id: &str) -> Option<&mut Graph> {
        for graph in self.graphs.iter_mut() {
            if graph.id == id ||
               graph.from.id == id ||
               graph.to.id == id 
            {
                return Some(graph);
            }
        }
        None
    }
    pub fn add(&mut self, graph: Graph) {
        self.graphs.push(graph);
    }
    pub fn persists(&self) -> Result<(), Box<dyn Error>> {
        for graph in self.graphs.iter() {
            graph.save(self.name())?
        }
        Ok(())
    }
}

pub trait Gruphst {
    fn name(&self) -> &String;
    fn id(&self) -> &String;
}

impl Gruphst for Node {
    fn name(&self)  -> &String {
        &self.name
    }
    fn id(&self) -> &String {
        &self.id
    }
}

impl Gruphst for Graphs {
    fn name(&self)  -> &String {
        &self.name
    }
    fn id(&self) -> &String {
        &self.id
    }
}

impl Gruphst for Graph {
    fn name(&self) -> &String {
        &self.relation
    }
    fn id(&self) -> &String {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let n = Node::new(String::from("Node 1"));
        assert_eq!(n.name(), "Node 1");
    }

    #[test]
    fn create_graph() {
        let node1 = Node::new("a node".to_string());
        let node2 = Node::new("b node".to_string());
        let graph = Graph::new("relation a-b".to_string(), node1, node2);
        assert_eq!(graph.relation, "relation a-b");
        assert_eq!(graph.name(), "relation a-b");
        assert_eq!(graph.from.name, "a node");
        assert_eq!(graph.to.name, "b node");
    }

    #[test]
    fn find_in_graphs() {
        let mut gru = Graphs::new("graphs-a".to_string());
        assert_eq!(gru.name(), "graphs-a");

        let node1 = Node::new("a node".to_string());
        let n1 = node1.clone();
        let node2 = Node::new("b node".to_string());
        let graph1 = Graph::new("relation a-b".to_string(), node1, node2);
        gru.add(graph1);
        assert_eq!(gru.graphs.len(), 1);

        let node3 = Node::new("c node".to_string());
        let node4 = Node::new("d node".to_string());
        let graph2 = Graph::new("relation c-d".to_string(), node3, node4);
        gru.add(graph2);
        assert_eq!(gru.graphs.len(), 2);

        let mut result = gru.find_relation("relation c-d");
        assert_eq!(result.unwrap().name(), "relation c-d");

        let node1_id = n1.id();
        result = gru.find_by_id(&node1_id);
        assert_eq!(result.unwrap().from().id(), node1_id);
    }
    
    #[test]
    fn persistence() {
        let mut gru = Graphs::new("graphs-a".to_string());
        let node1 = Node::new("a node".to_string());
        let node2 = Node::new("b node".to_string());
        let graph1 = Graph::new("relation a-b".to_string(), node1, node2);
        gru.add(graph1.clone());

        let _ = graph1.save("data-test");

        let node3 = Node::new("c node".to_string());
        let node4 = Node::new("d node".to_string());
        let graph2 = Graph::new("relation c-d".to_string(), node3, node4);
        gru.add(graph2);

        let _ = gru.persists();
    }
}
