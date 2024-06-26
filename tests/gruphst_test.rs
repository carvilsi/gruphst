use gruphst::{ Graphs, Graph, Node, Gruphst };

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
        let graph = Graph::new(&node1, "relation a-b".to_string(), &node2);
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
        let node2 = Node::new("b node".to_string());
        let graph1 = Graph::new(&node1, "friend of".to_string(), &node2);
        gru.add(&graph1);
        assert_eq!(gru.graphs.len(), 1);

        let node3 = Node::new("c node".to_string());
        let node4 = Node::new("d node".to_string());
        let graph2 = Graph::new(&node3, "knows".to_string(), &node4);
        gru.add(&graph2);
        assert_eq!(gru.graphs.len(), 2);

        let mut res_graphs= gru.find_by_relation("knows").unwrap();
        assert_eq!(res_graphs.len(), 1);
        assert_eq!(res_graphs[0].name(), "knows");

        let res = gru.find_by_id(&node1.id);
        assert_eq!(res.unwrap().from().id, node1.id);

        let node5 = Node::new("e node".to_string());
        let graph3 = Graph::new(&node1, "friend of".to_string(), &node5);
        gru.add(&graph3);

        res_graphs = gru.find_by_relation("friend of").unwrap();
        assert_eq!(res_graphs.len(), 2);
        assert_eq!(res_graphs[0].name(), "friend of");
        assert_eq!(res_graphs[1].name(), "friend of");
    }
    
    #[test]
    fn persistence() {
        let mut gru = Graphs::new("graphs-a".to_string());
        let node1 = Node::new("a node".to_string());
        let node2 = Node::new("b node".to_string());
        let graph1 = Graph::new(&node1, "relation a-b".to_string(), &node2);
        gru.add(&graph1);

        let node3 = Node::new("c node".to_string());
        let node4 = Node::new("d node".to_string());
        let graph2 = Graph::new(&node3, "relation c-d".to_string(), &node4);
        gru.add(&graph2);

        let _ = gru.persists();

        let name = gru.name();
        let grphs = Graphs::load(name);
        match grphs {
            Ok(grphs) => {
                assert_eq!(grphs.name(), name);
                assert_eq!(grphs.graphs[0].name(), graph1.name());
                assert_eq!(grphs.graphs[1], graph2);
            },
            Err(_) => panic!(),
        }
    }

    #[test]
    fn delete_from_graph() {
        let mut my_graph = Graphs::new("friends".to_string());
        let alice = Node::new("Alice".to_string());
        let bob = Node::new("Bob".to_string());
        let alice_bob = Graph::new(&alice, String::from("is friend of"), &bob);
        my_graph.add(&alice_bob);

        let alice_fred =
            Graph::new(
                &alice,
                String::from("is firend of"),
                &Node::new("Fred".to_string())
            );
        my_graph.add(&alice_fred);

        assert_eq!(my_graph.graphs.len(), 2);

        my_graph.delete_graph_by_id(alice_bob.id()); 
        assert_eq!(my_graph.graphs.len(), 1);
    }

    #[test]
    fn update_node_name() {
        let mut alice_node = Node::new(String::from("alice node"));
        assert_eq!(alice_node.name(), "alice node");
        alice_node.update_name("just alice");
        assert_eq!(alice_node.name(), "just alice");
        let bob_node = Node::new("bob node".to_string());
        let mut graph = Graph::new(&alice_node, "best friends".to_string(), &bob_node);
        alice_node.update_name("alice");
        graph.update_from(&alice_node);
        assert_eq!(graph.from().name(), "alice");
    }

    #[test]
    fn update_graph_node() {
        let mut alice_node = Node::new(String::from("alice node"));
        let bob_node = Node::new("bob node".to_string());
        let mut graph = Graph::new(&alice_node, "best friends".to_string(), &bob_node);
        assert_eq!(graph.from().name(), "alice node");
        assert_eq!(graph.to().name(), "bob node");
        alice_node.update_name("alice");
        graph.update_from(&alice_node);
        assert_eq!(graph.from().name(), "alice");
        let fred_node = Node::new("fred node".to_string());
        graph.update_to(&fred_node);
        assert_eq!(graph.to().name(), "fred node");
        assert_ne!(graph.to().id, bob_node.id);
    }
}
