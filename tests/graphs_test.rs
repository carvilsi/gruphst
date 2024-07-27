use std::char::from_u32_unchecked;

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
        &Graph::create(&Node::new("alice"), "lives in", &Node::new("Springfield")),
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
        &Graph::create(&Node::new("gandalf"), "enemy of", &Node::new("Saruman")),
        Some("middle-earth"),
    );
    assert_eq!(graphs.len(), 5);
}

#[test]
fn should_insert_a_graph_into_the_vault_without_init() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.len_graphs(), 1);
    assert_eq!(graphs.len(), 4);
    graphs.add_graph(
        &Graph::create(&Node::new("gandalf"), "enemy of", &Node::new("Saruman")),
        Some("middle-earth"),
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
        &Graph::create(&Node::new("gandalf"), "enemy of", &Node::new("Saruman")),
        Some("middle-earth"),
    );
    let unique_relations_ag = graphs.uniq_relations();
    assert_eq!(
        unique_relations_ag,
        vec!["enemy of", "friend of", "relative of"]
    );
}

#[test]
fn should_return_the_unique_relations_for_certain_graph_on_vault() {
    let mut graphs = prepare_graphs_test();
    graphs.insert("middle-earth");
    graphs.add_graph(
        &Graph::create(&Node::new("gandalf"), "enemy of", &Node::new("Saruman")),
        Some("middle-earth"),
    );
    let unique_relations = graphs.uniq_graph_relations(Some("my graphs"));
    let unique_relations_middle_earth = graphs.uniq_graph_relations(None);
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

#[test]
fn should_create_new_vault_and_add_graph() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.len_graphs(), 1);
    let graph = Graph::create(
        &Node::new("foo"), 
        "before a", 
        &Node::new("bar"));
    graphs.insert_with("other", &graph);
    assert_eq!(graphs.len_graphs(), 2);
}

#[test]
fn find_in_graph() {
    let mut graphs = prepare_graphs_test();
    let from_node = Node::new("gandalf");
    let from_node_id = from_node.get_id();
    graphs.add_graph(
        &Graph::create(&from_node, "enemy of", &Node::new("Saruman")),
        Some("middle-earth"),
    );
    let mut found_graph = graphs.find_by_id(&from_node_id, None).unwrap();
    assert_eq!(found_graph.get_label(), "enemy of");
    assert_eq!(found_graph.get_from_node().get_label(), "gandalf");
    let default_graph_id = graphs.get_graphs(Some("my graphs")).unwrap()[0].get_id(); 
    found_graph = graphs.find_by_id(&default_graph_id, Some("my graphs")).unwrap();
    assert_eq!(found_graph.get_label(), "friend of");
}

#[test]
fn find_in_graphs() {
    let mut graphs = prepare_graphs_test();
    let from_node = Node::new("gandalf");
    let from_node_id = from_node.get_id();
    graphs.add_graph(
        &Graph::create(&from_node, "enemy of", &Node::new("Saruman")),
        Some("middle-earth"),
    );
    let default_graph_id = graphs.get_graphs(Some("my graphs")).unwrap()[0].get_id(); 
    let mut found_graph = graphs.find_by_id_in_graphs(&default_graph_id).unwrap();
    assert_eq!(found_graph.get_label(), "friend of");
    found_graph = graphs.find_by_id_in_graphs(&from_node_id).unwrap();
    assert_eq!(found_graph.get_from_node().get_label(), "gandalf");
}

// TODO: these tests must be refactored
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
