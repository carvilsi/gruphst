use gruphst::{graph::Graph, graphs::Graphs, node::Node, *};

fn prepare_graphs_test() -> Graphs {
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
fn get_label() {
    let graphs = prepare_graphs_test();
    assert_eq!(graphs.get_label(), "my graphs");
}

#[test]
fn lengths_of_graphs() {
    let graphs = prepare_graphs_test();
    assert_eq!(graphs.len(), 4);
}

#[test]
fn should_init_adding_a_graph() {
    let graphs = Graphs::init_with(
        "grpahs0",
        &Graph::create(
            &Node::new("alice"),
            "lives in",
            &Node::new("Springfield")
        )
    );
    assert_eq!(graphs.len(), 1);
}

#[test]
fn should_insert_a_graph_into_the_vault() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.len_graphs(), 1);
    assert_eq!(graphs.len(), 4);
    graphs.insert("middle-earth");
    assert_eq!(graphs.len_graphs(), 2);
    assert_eq!(graphs.len(), 4);
    graphs.add_graph(
        &Graph::create(
            &Node::new("gandalf"),
            "enemy of",
            &Node::new("Saruman")),
       Some("middle-earth") 
    );
    assert_eq!(graphs.len(), 5);
}

#[test]
fn should_insert_a_graph_into_the_vault_without_init() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.len_graphs(), 1);
    assert_eq!(graphs.len(), 4);
    graphs.add_graph(
        &Graph::create(
            &Node::new("gandalf"),
            "enemy of",
            &Node::new("Saruman")),
       Some("middle-earth") 
    );
    assert_eq!(graphs.len_graphs(), 2);
    assert_eq!(graphs.len(), 5);
}

#[test]
fn is_empty_graphs() {
    let mut graphs = Graphs::init("empty");
    assert!(graphs.is_empty());
    graphs = prepare_graphs_test();
    assert!(!graphs.is_empty());
}

#[test]
fn find_in_graphs_failing() {
    let mut graphs = prepare_graphs_test();
    assert!(graphs.find_by_relation("lol", None).is_err());
}

#[test]
fn should_return_the_unique_relations_for_whole_graphs() {
    let mut graphs = prepare_graphs_test();
    let unique_relations = graphs.uniq_relations();
    assert_eq!(unique_relations, vec!["friend of", "relative of"]);
    graphs.insert("middle-earth");
    assert_eq!(graphs.len_graphs(), 2);
    assert_eq!(graphs.len(), 4);
    graphs.add_graph(
        &Graph::create(
            &Node::new("gandalf"),
            "enemy of",
            &Node::new("Saruman")),
       Some("middle-earth") 
    );
    let unique_relations_ag = graphs.uniq_relations();
    assert_eq!(unique_relations_ag, vec!["enemy of", "friend of", "relative of"]);
}

#[test]
fn should_return_the_unique_relations_for_certain_graph_on_vault() {
    let mut graphs = prepare_graphs_test();
    graphs.insert("middle-earth");
    graphs.add_graph(
        &Graph::create(
            &Node::new("gandalf"),
            "enemy of",
            &Node::new("Saruman")),
       Some("middle-earth") 
    );
    let unique_relations = graphs.uniq_graph_relations(Some("my graphs"));
    let unique_relations_middle_earth= graphs.uniq_graph_relations(None);
    assert_eq!(unique_relations.unwrap(), vec!["friend of", "relative of"]);
    assert_eq!(unique_relations_middle_earth.unwrap(), vec!["enemy of"]);
}

#[test]
fn should_fail_uinque_graph_relations_since_vault_does_not_exists() {
    let graphs = prepare_graphs_test();
    assert!(graphs.uniq_graph_relations(Some("foobar")).is_err());
}

#[test]
fn equals_attributes() {
    let graphs = prepare_graphs_test();
    let results = graphs.attr_equals_to("age", 42, None).unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn relation_in() {
    let graphs = prepare_graphs_test();
    let results = graphs.has_relation_in("friend of", None);
    assert_eq!(results.clone().unwrap().len(), 2);
    assert_eq!(results.clone().unwrap()[0].get_label(), "Bob");
    assert_eq!(results.unwrap()[1].get_label(), "Alice");
}

#[test]
fn relation_out() {
    let graphs = prepare_graphs_test();
    let results = graphs.has_relation_out("friend of", None);
    assert_eq!(results.clone().unwrap().len(), 3);
    assert_eq!(results.clone().unwrap()[0].get_label(), "Alice");
    assert_eq!(results.clone().unwrap()[1].get_label(), "Bob");
    assert_eq!(results.unwrap()[2].get_label(), "Fred");
}

// TODO: this tests must be refactored
#[test]
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
fn update_graph_fail() {
    let mut grphs = Graphs::init("foobar");

    let alice = Node::new("Alice");
    let bob = Node::new("Bob");
    let alice_bob = Graph::create(&alice, "friend of", &bob);
    grphs.add_graph(&alice_bob, None);

    let bob_alice = Graph::create(&bob, "friend of", &alice);
    assert!(grphs.update_graph(&bob_alice, None).is_err());
}
