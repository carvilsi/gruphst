use serial_test::serial;

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
