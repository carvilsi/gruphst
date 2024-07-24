use serial_test::serial;

use gruphst::config::*;
use gruphst::enable_logging;
use gruphst::graph::Graph;
use gruphst::graphs::Graphs;
use gruphst::node::Node;

use gruphst::*;

// TODO: refactor the tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn logg() {
        enable_logging(log::Level::Debug);
    }

    #[test]
    fn find_in_graphs_failing() {
        let mut my_graph = Graphs::init("failing");
        let mut graph = Graph::new("");
        graph.add_relation(&Node::new("Alice"), "is friend", &Node::new("Bob"));
        my_graph.add_graph(&graph, None);
        assert!(my_graph.find_by_relation("lol", None).is_err());
    }

    #[test]
    #[serial]
    fn create_node() {
        let n = Node::new("Node 1");
        assert_eq!(n.get_label(), "Node 1");
    }

    #[test]
    #[serial]
    fn create_graph() {
        let node1 = Node::new("a node");
        let node2 = Node::new("b node");
        let graph = Graph::create(&node1, "relation a-b", &node2);
        assert_eq!(graph.get_relation(), "relation a-b");
        assert_eq!(graph.get_relation(), "relation a-b");
        assert_eq!(graph.get_from_node().get_label(), "a node");
        assert_eq!(graph.get_to_node().get_label(), "b node");
    }

    #[test]
    #[serial]
    fn find_in_graphs() {
        let mut gru = Graphs::init("graphs-a");
        assert_eq!(gru.get_label(), "graphs-a");

        let node1 = Node::new("a node");
        let node2 = Node::new("b node");
        let graph1 = Graph::create(&node1, "friend of", &node2);
        gru.add_graph(&graph1, None);
        assert_eq!(gru.len(), 1);

        let node3 = Node::new("c node");
        let node4 = Node::new("d node");
        let graph2 = Graph::create(&node3, "knows", &node4);
        gru.add_graph(&graph2, None);
        assert_eq!(gru.len(), 2);

        let mut res_graphs = gru.find_by_relation("knows", None).unwrap();
        assert_eq!(res_graphs.len(), 1);
        assert_eq!(res_graphs[0].get_relation(), "knows");

        let res = gru.find_by_id(&node1.get_id(), None);
        assert_eq!(res.unwrap().get_from_node().get_id(), node1.get_id());

        let node5 = Node::new("e node");
        let graph3 = Graph::create(&node1, "friend of", &node5);
        gru.add_graph(&graph3, None);

        res_graphs = gru.find_by_relation("friend of", None).unwrap();
        assert_eq!(res_graphs.len(), 2);
        assert_eq!(res_graphs[0].get_relation(), "friend of");
        assert_eq!(res_graphs[1].get_relation(), "friend of");
    }

    #[test]
    #[serial]
    fn persistence() {
        let mut gru = Graphs::init("graphs-a");
        let mut node1 = Node::new("a node");
        node1.set_attr("foo", "bar");
        let node2 = Node::new("b node");
        let graph1 = Graph::create(&node1, "relation a-b", &node2);
        gru.add_graph(&graph1, None);

        let node3 = Node::new("c node");
        let node4 = Node::new("d node");
        let graph2 = Graph::create(&node3, "relation c-d", &node4);
        gru.add_graph(&graph2, None);

        let _ = gru.persists();

        let name = gru.get_label();
        let file_name = format!("{}.grphst", name);
        let grphs = Graphs::load(&file_name);
        match grphs {
            Ok(grphs) => {
                let graphs = grphs.get_graphs(Some(name.as_str())).unwrap();
                assert_eq!(grphs.get_label(), name);
                assert_eq!(graphs[0].get_relation(), graph1.get_relation());
                assert_eq!(graphs[0].get_from_node().get_label(), "a node");
                assert_eq!(graphs[0].get_from_node().len_attr(), 1);
                assert_eq!(graphs[0].get_from_node().get_attr("foo").unwrap(), "bar");
                assert_eq!(graphs[1], graph2);
            }
            Err(_) => panic!(),
        }
    }

    #[test]
    #[serial]
    fn delete_from_graph() {
        let mut my_graph = Graphs::init("friends");
        let alice = Node::new("Alice");
        let bob = Node::new("Bob");
        let alice_bob = Graph::create(&alice, "is friend of", &bob);
        my_graph.add_graph(&alice_bob, None);

        let alice_fred = Graph::create(&alice, "is firend of", &Node::new("Fred"));
        my_graph.add_graph(&alice_fred, None);

        assert_eq!(my_graph.len(), 2);

        let _ = my_graph.delete_graph_by_id(alice_bob.get_id(), None);
        assert_eq!(my_graph.len(), 1);
    }

    #[test]
    #[serial]
    fn delete_from_graph_fail() {
        let mut my_graph = Graphs::init("failing");
        assert!(my_graph
            .delete_graph_by_id("foobar".to_string(), None)
            .is_err());
        my_graph.add_graph(
            &Graph::create(&Node::new("Alice"), "is friend", &Node::new("Bob")),
            None,
        );
        assert!(my_graph
            .delete_graph_by_id("foobar".to_string(), None)
            .is_err());
    }

    #[test]
    #[serial]
    fn update_node_name() {
        let mut alice_node = Node::new("alice node");
        assert_eq!(alice_node.get_label(), "alice node");
        alice_node.set_label("just alice");
        assert_eq!(alice_node.get_label(), "just alice");
        let bob_node = Node::new("bob node");
        let mut graph = Graph::create(&alice_node, "best friends", &bob_node);
        alice_node.set_label("alice");
        graph.update_from(&alice_node);
        assert_eq!(graph.get_from_node().get_label(), "alice");
    }

    #[test]
    #[serial]
    fn update_graph_node() {
        let mut alice_node = Node::new("alice node");
        let bob_node = Node::new("bob node");
        let mut graph = Graph::create(&alice_node, "best friends", &bob_node);
        assert_eq!(graph.get_from_node().get_label(), "alice node");
        assert_eq!(graph.get_to_node().get_label(), "bob node");
        alice_node.set_label("alice");
        graph.update_from(&alice_node);
        assert_eq!(graph.get_from_node().get_label(), "alice");
        let fred_node = Node::new("fred node");
        graph.update_to(&fred_node);
        assert_eq!(graph.get_to_node().get_label(), "fred node");
        assert_ne!(graph.get_to_node().get_id(), bob_node.get_id());
    }

    #[test]
    #[serial]
    fn graphs_stats() {
        let mut graphs = Graphs::init("friends-and-enemies");

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

        let relation_friend_of = "friend of";
        let relation_relative_of = "relative of";
        let mut graph = Graph::create(&alice, relation_friend_of, &bob);
        graphs.add_graph(&graph, None);

        graph = Graph::create(&alice, relation_relative_of, &fred);
        graphs.add_graph(&graph, None);

        graph = Graph::create(&alice, relation_friend_of, &john);
        graphs.add_graph(&graph, None);

        graph = Graph::create(&peter, relation_relative_of, &john);
        graphs.add_graph(&graph, None);

        graphs.insert("only relatives");
        graphs.add_graph(&graph, None);

        // XXX: Note that this could be arch dependent ¯\\(°_o)/¯
        let stats = graphs.stats().unwrap();
        assert_eq!(stats.get_len_graphs(), 5);
        assert_eq!(stats.get_total_nodes(), 10);
        assert_eq!(stats.get_total_attr(), 12);
        assert_eq!(stats.get_mem(), 2179);
        assert_eq!(stats.get_uniq_rel(), 2);
        assert_eq!(stats.get_total_graphs(), 2);
    }

    #[test]
    #[serial]
    fn update_graph_fail() {
        let mut grphs = Graphs::init("foobar");

        let alice = Node::new("Alice");
        let bob = Node::new("Bob");
        let alice_bob = Graph::create(&alice, "friend of", &bob);
        grphs.add_graph(&alice_bob, None);

        let bob_alice = Graph::create(&bob, "friend of", &alice);
        assert!(grphs.update_graph(&bob_alice, None).is_err());
    }

    #[test]
    #[serial]
    fn lengths_of_graphs() {
        let mut graphs = Graphs::init("lengths");

        assert!(graphs.is_empty());

        let alice = Node::new("Alice");
        let bob = Node::new("Bob");

        graphs.add_graph(&Graph::create(&alice, "friend", &bob), None);
        graphs.add_graph(&Graph::create(&bob, "friend", &alice), None);

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
        alice.upsert_attr("phone", "555-555-555");
        assert_eq!(alice.get_attr("phone").unwrap(), "555-555-555");
        assert_eq!(alice.len_attr(), 2);
        alice.upsert_attr("phone", "556-554-553");
        assert_eq!(alice.get_attr("phone").unwrap(), "556-554-553");
        assert_eq!(alice.len_attr(), 2);
    }

    fn do_some_networking() -> Graphs {
        let mut graphs = Graphs::init("my graphs");

        let mut alice = Node::new("Alice");
        alice.set_attr("phone", "555-555-555");
        alice.set_attr("address", "Elm street");

        let mut bob = Node::new("Bob");
        bob.set_attr("age", 42);

        let fred = Node::new("Fred");

        graphs.add_graph(&Graph::create(&alice, "friend of", &bob), None);
        graphs.add_graph(&Graph::create(&bob, "friend of", &alice), None);
        graphs.add_graph(&Graph::create(&fred, "relative of", &alice), None);
        graphs.add_graph(&Graph::create(&fred, "friend of", &alice), None);

        graphs
    }

    #[test]
    #[serial]
    fn the_unique_relations() {
        let graphs = do_some_networking();
        let unique_relations = graphs.uniq_relations();

        assert_eq!(unique_relations, vec!["friend of", "relative of"]);
    }

    #[test]
    #[serial]
    fn configuration() {
        let config_mem = get_max_mem_usage();

        assert_eq!(config_mem, 50 * 1024 * 1024);

        let config_log_level = get_log_level();

        assert_eq!(config_log_level, log::Level::Debug);
    }

    #[test]
    #[serial]
    fn equals_attributes() {
        let graphs = do_some_networking();
        let results = graphs.attr_equals_to("age", 42, None).unwrap();

        assert_eq!(results.len(), 2);
    }

    #[test]
    #[serial]
    fn relation_in() {
        let graphs = do_some_networking();
        let results = graphs.has_relation_in("friend of", None);
        assert_eq!(results.clone().unwrap().len(), 2);
        assert_eq!(results.clone().unwrap()[0].get_label(), "Bob");
        assert_eq!(results.unwrap()[1].get_label(), "Alice");
    }

    #[test]
    #[serial]
    fn relation_out() {
        let graphs = do_some_networking();
        let results = graphs.has_relation_out("friend of", None);
        assert_eq!(results.clone().unwrap().len(), 3);
        assert_eq!(results.clone().unwrap()[0].get_label(), "Alice");
        assert_eq!(results.clone().unwrap()[1].get_label(), "Bob");
        assert_eq!(results.unwrap()[2].get_label(), "Fred");
    }
}
