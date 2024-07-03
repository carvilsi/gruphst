use gruphst::enable_logging;
use gruphst::{Graph, Graphs, Node};
use serial_test::serial;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn logg() {
        enable_logging(log::Level::Debug);
    }

    #[test]
    #[serial]
    fn find_in_graphs_failing() {
        let mut my_graph = Graphs::new("failing");
        my_graph.add(&Graph::new(
            &Node::new("Alice"),
            "is friend",
            &Node::new("Bob"),
        ));
        assert!(my_graph.find_by_id("foobarid").is_err());
        assert!(my_graph.find_by_relation("lol").is_err());
    }

    #[test]
    #[serial]
    fn create_node() {
        let n = Node::new("Node 1");
        assert_eq!(n.name, "Node 1");
    }

    #[test]
    #[serial]
    fn create_graph() {
        let node1 = Node::new("a node");
        let node2 = Node::new("b node");
        let graph = Graph::new(&node1, "relation a-b", &node2);
        assert_eq!(graph.relation, "relation a-b");
        assert_eq!(graph.relation, "relation a-b");
        assert_eq!(graph.from.name, "a node");
        assert_eq!(graph.to.name, "b node");
    }

    #[test]
    #[serial]
    fn find_in_graphs() {
        let mut gru = Graphs::new("graphs-a");
        assert_eq!(gru.name, "graphs-a");

        let node1 = Node::new("a node");
        let node2 = Node::new("b node");
        let graph1 = Graph::new(&node1, "friend of", &node2);
        gru.add(&graph1);
        assert_eq!(gru.len(), 1);

        let node3 = Node::new("c node");
        let node4 = Node::new("d node");
        let graph2 = Graph::new(&node3, "knows", &node4);
        gru.add(&graph2);
        assert_eq!(gru.len(), 2);

        let mut res_graphs = gru.find_by_relation("knows").unwrap();
        assert_eq!(res_graphs.len(), 1);
        assert_eq!(res_graphs[0].relation, "knows");

        let res = gru.find_by_id(&node1.id);
        assert_eq!(res.unwrap().from.id, node1.id);

        let node5 = Node::new("e node");
        let graph3 = Graph::new(&node1, "friend of", &node5);
        gru.add(&graph3);

        res_graphs = gru.find_by_relation("friend of").unwrap();
        assert_eq!(res_graphs.len(), 2);
        assert_eq!(res_graphs[0].relation, "friend of");
        assert_eq!(res_graphs[1].relation, "friend of");
    }

    #[test]
    #[serial]
    fn persistence() {
        let mut gru = Graphs::new("graphs-a");
        let mut node1 = Node::new("a node");
        node1.set_attr("foo", "bar");
        let node2 = Node::new("b node");
        let graph1 = Graph::new(&node1, "relation a-b", &node2);
        gru.add(&graph1);

        let node3 = Node::new("c node");
        let node4 = Node::new("d node");
        let graph2 = Graph::new(&node3, "relation c-d", &node4);
        gru.add(&graph2);

        let _ = gru.persists();

        let name = gru.name;
        let file_name = format!("{}.grphst", name);
        let grphs = Graphs::load(&file_name);
        match grphs {
            Ok(grphs) => {
                assert_eq!(grphs.name, name);
                assert_eq!(grphs.graphs[0].relation, graph1.relation);
                assert_eq!(grphs.graphs[0].from.name, "a node");
                assert_eq!(grphs.graphs[0].from.len_attr(), 1);
                assert_eq!(grphs.graphs[0].from.get_attr("foo").unwrap(), "bar");
                assert_eq!(grphs.graphs[1], graph2);
            }
            Err(_) => panic!(),
        }
    }

    #[test]
    #[serial]
    fn delete_from_graph() {
        let mut my_graph = Graphs::new("friends");
        let alice = Node::new("Alice");
        let bob = Node::new("Bob");
        let alice_bob = Graph::new(&alice, "is friend of", &bob);
        my_graph.add(&alice_bob);

        let alice_fred = Graph::new(&alice, "is firend of", &Node::new("Fred"));
        my_graph.add(&alice_fred);

        assert_eq!(my_graph.len(), 2);

        let _ = my_graph.delete_graph_by_id(alice_bob.id);
        assert_eq!(my_graph.len(), 1);
    }

    #[test]
    #[serial]
    fn delete_from_graph_fail() {
        let mut my_graph = Graphs::new("failing");
        assert!(my_graph.delete_graph_by_id("foobar".to_string()).is_err());
        my_graph.add(&Graph::new(
            &Node::new("Alice"),
            "is friend",
            &Node::new("Bob"),
        ));
        assert!(my_graph.delete_graph_by_id("foobar".to_string()).is_err());
    }

    #[test]
    #[serial]
    fn update_node_name() {
        let mut alice_node = Node::new("alice node");
        assert_eq!(alice_node.name, "alice node");
        alice_node.update_name("just alice");
        assert_eq!(alice_node.name, "just alice");
        let bob_node = Node::new("bob node");
        let mut graph = Graph::new(&alice_node, "best friends", &bob_node);
        alice_node.update_name("alice");
        graph.update_from(&alice_node);
        assert_eq!(graph.from.name, "alice");
    }

    #[test]
    #[serial]
    fn update_graph_node() {
        let mut alice_node = Node::new("alice node");
        let bob_node = Node::new("bob node");
        let mut graph = Graph::new(&alice_node, "best friends", &bob_node);
        assert_eq!(graph.from.name, "alice node");
        assert_eq!(graph.to.name, "bob node");
        alice_node.update_name("alice");
        graph.update_from(&alice_node);
        assert_eq!(graph.from.name, "alice");
        let fred_node = Node::new("fred node");
        graph.update_to(&fred_node);
        assert_eq!(graph.to.name, "fred node");
        assert_ne!(graph.to.id, bob_node.id);
    }

    #[test]
    #[serial]
    fn graphs_stats() {
        let mut graphs = Graphs::new("friends-and-enemies");

        let mut alice = Node::new("Alice");
        let mut bob = Node::new("Bob");
        let fred = Node::new("Fred");
        let john = Node::new("John");
        let peter = Node::new("Peter");

        alice.set_attr("address", "Elm street");
        alice.set_attr("email", "alice@mailinator.com");
        alice.set_attr("age", 35);

        bob.set_attr("address", "Talbot street");
        bob.set_attr("email", "bob@mailinator.com");
        bob.set_attr("age", 40);

        let relation_friend_of = "friend_of";
        let mut graph = Graph::new(&alice, relation_friend_of, &bob);
        graphs.add(&graph);

        graph = Graph::new(&alice, relation_friend_of, &fred);
        graphs.add(&graph);

        graph = Graph::new(&alice, relation_friend_of, &john);
        graphs.add(&graph);

        graph = Graph::new(&peter, relation_friend_of, &john);
        graphs.add(&graph);

        // XXX: Note that this could be arch dependent ¯\\(°_o)/¯
        let stats = graphs.stats().unwrap();
        assert_eq!(stats.len, 4);
        assert_eq!(stats.total_nodes, 8);
        assert_eq!(stats.total_attributes, 12);
        assert_eq!(stats.mem, 1219);
        assert_eq!(stats.name, "friends-and-enemies");
    }

    #[test]
    #[serial]
    fn update_graph_fail() {
        let mut grphs = Graphs::new("foobar");

        let alice = Node::new("Alice");
        let bob = Node::new("Bob");
        let alice_bob = Graph::new(&alice, "friend of", &bob);
        grphs.add(&alice_bob);

        let bob_alice = Graph::new(&bob, "friend of", &alice);
        assert!(grphs.update_graph(&bob_alice).is_err());
    }

    #[test]
    #[serial]
    fn lengths_of_graphs() {
        let mut graphs = Graphs::new("lengths");

        assert!(graphs.is_empty());

        let alice = Node::new("Alice");
        let bob = Node::new("Bob");

        graphs.add(&Graph::new(&alice, "friend", &bob));
        graphs.add(&Graph::new(&bob, "friend", &alice));

        assert_eq!(graphs.len(), 2);
        assert!(!graphs.is_empty());
    }

    #[test]
    #[serial]
    fn load_persisted_fail() {
        assert!(Graphs::load("tests/does-not-exists.grphst").is_err());
        assert!(Graphs::load("tests/data/wrong-persisted-file.grphst").is_err());
    }

    #[test]
    #[serial]
    fn attributes() {
        let mut alice = Node::new("Alice");
        assert_eq!(alice.len_attr(), 0);
        assert!(alice.is_empty_attr());
        alice.set_attr("address", "Elm Street");
        assert_eq!(alice.get_attr("address").unwrap(), "Elm Street");
        assert_eq!(alice.len_attr(), 1);
        alice.set_attr("age", 34);
        assert_eq!(alice.get_attr("age").unwrap(), "34");
        let _ = alice.update_attr("age", 43);
        assert_eq!(alice.get_attr("age").unwrap(), "43");
        assert_eq!(alice.len_attr(), 2);
        let attrs = alice.get_attr_keys();
        assert!(attrs.contains(&&"age"));
        assert!(attrs.contains(&&"address"));
        assert!(alice.get_attr("phone").is_err());
        let _ = alice.del_attr("age");
        assert_eq!(alice.len_attr(), 1);
        assert!(!alice.is_empty_attr());
    }
}
