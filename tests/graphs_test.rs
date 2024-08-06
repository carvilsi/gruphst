use gruphst::{edge::Edge, graphs::Graphs, vertex::Vertex};

pub fn prepare_graphs_test() -> Graphs {
    let mut graphs = Graphs::init("my graphs");

    let mut alice = Edge::new("Alice");
    alice.set_attr("phone", "555-555-555");
    alice.set_attr("address", "Elm street");

    let mut bob = Edge::new("Bob");
    bob.set_attr("age", 42);

    let fred = Edge::new("Fred");

    graphs.add_graph(&Vertex::create(&alice, "friend of", &bob), None);
    graphs.add_graph(&Vertex::create(&bob, "friend of", &alice), None);
    graphs.add_graph(&Vertex::create(&fred, "relative of", &alice), None);
    graphs.add_graph(&Vertex::create(&fred, "friend of", &bob), None);

    graphs
}

// fn prepare_insert_graph_test(graphs: &mut Graphs) -> &mut Graphs {
pub fn prepare_insert_graph_test(graphs: &mut Graphs) {
    graphs.insert("middle-earth");
    graphs.add_graph(
        &Vertex::create(&Edge::new("Gandalf"), "enemy of", &Edge::new("Saruman")),
        Some("middle-earth"),
    );
    // graphs
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
        &Vertex::create(&Edge::new("alice"), "lives in", &Edge::new("Springfield")),
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
    prepare_insert_graph_test(&mut graphs);
    assert_eq!(graphs.len(), 5);
}

#[test]
fn should_insert_a_graph_into_the_vault_without_init() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.len_graphs(), 1);
    assert_eq!(graphs.len(), 4);
    graphs.add_graph(
        &Vertex::create(&Edge::new("Earth"), "has satellite", &Edge::new("Moon")),
        Some("solar-system"),
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
fn should_find_by_relation() {
    let mut graphs = prepare_graphs_test();
    let found_graph = graphs.find_by_relation("friend of", None).unwrap();
    assert_eq!(found_graph.len(), 3);
}

#[test]
fn should_find_by_relation_in_graphs() {
    let mut graphs = prepare_graphs_test();
    let mut found_graph = graphs.find_by_relation("friend of", None).unwrap();
    assert_eq!(found_graph.len(), 3);
    prepare_insert_graph_test(&mut graphs);
    found_graph = graphs
        .find_by_relation("enemy of", Some("middle-earth"))
        .unwrap();
    assert_eq!(found_graph.len(), 1);
}

#[test]
fn should_find_by_relations_name() {
    let mut graphs = prepare_graphs_test();
    let relations = vec!["friend of", "relative of"];
    let found_graphs = graphs.find_by_relations(relations, None).unwrap();
    assert_eq!(found_graphs.len(), 4);
}

#[test]
fn should_not_find_by_relation() {
    let mut graphs = prepare_graphs_test();
    assert!(graphs.find_by_relation("lol", None).is_err());
}

#[test]
fn should_return_the_unique_relations_for_whole_graphs() {
    let mut graphs = prepare_graphs_test();
    let unique_relations = graphs.uniq_relations();
    assert_eq!(unique_relations, vec!["friend of", "relative of"]);
    prepare_insert_graph_test(&mut graphs);
    assert_eq!(graphs.len_graphs(), 2);
    assert_eq!(graphs.len(), 5);
    let unique_relations_ag = graphs.uniq_relations();
    assert_eq!(
        unique_relations_ag,
        vec!["enemy of", "friend of", "relative of"]
    );
}

#[test]
fn should_find_graphs_with_attribute() {
    let mut graphs = prepare_graphs_test();
    let found_graphs = graphs.has_graph_edge_attr("age", None).unwrap();
    assert_eq!(found_graphs.len(), 3);
    assert_eq!(found_graphs[0].get_to_edge().get_label(), "Bob");
}

#[test]
fn should_find_graphs_with_attribute_like() {
    let mut graphs = prepare_graphs_test();
    let found_graphs = graphs.like_graph_edge_attr("Ag", None).unwrap();
    assert_eq!(found_graphs.len(), 3);
    assert_eq!(found_graphs[0].get_to_edge().get_label(), "Bob");
}

#[test]
fn should_find_graphs_with_attribute_equal() {
    let graphs = prepare_graphs_test();
    let found_graphs = graphs.attr_equals_to("age", 42, None).unwrap();
    assert_eq!(found_graphs.len(), 3);
    assert_eq!(found_graphs[0].get_to_edge().get_label(), "Bob");
}

#[test]
fn should_not_find_graphs_with_attribute_equal() {
    let graphs = prepare_graphs_test();
    assert!(graphs.attr_equals_to("age", 43, None).is_err());
}

#[test]
fn should_return_the_unique_relations_for_certain_graph_on_vault() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);
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
    assert_eq!(results.len(), 3);
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
    let graph = Vertex::create(&Edge::new("foo"), "before a", &Edge::new("bar"));
    graphs.insert_with("other", &graph);
    assert_eq!(graphs.len_graphs(), 2);
}

#[test]
fn should_find_in_graph_by_id() {
    let mut graphs = prepare_graphs_test();
    let from_edge = Edge::new("Earth");
    let from_edge_id = from_edge.get_id();
    graphs.add_graph(
        &Vertex::create(&from_edge, "has satellite", &Edge::new("Moon")),
        Some("solar-system"),
    );
    let mut found_graph = graphs.find_by_id(&from_edge_id, None).unwrap();
    assert_eq!(found_graph.get_label(), "has satellite");
    assert_eq!(found_graph.get_from_edge().get_label(), "Earth");
    let default_graph_id = graphs.get_graphs(Some("my graphs")).unwrap()[0].get_id();
    found_graph = graphs
        .find_by_id(&default_graph_id, Some("my graphs"))
        .unwrap();
    assert_eq!(found_graph.get_label(), "friend of");
}

#[test]
fn should_find_in_graphs_by_id() {
    let mut graphs = prepare_graphs_test();
    let from_edge = Edge::new("Earth");
    let from_edge_id = from_edge.get_id();
    graphs.add_graph(
        &Vertex::create(&from_edge, "has satellite", &Edge::new("Moon")),
        Some("solar-system"),
    );
    let default_graph_id = graphs.get_graphs(Some("my graphs")).unwrap()[0].get_id();
    let mut found_graph = graphs.find_by_id_in_graphs(&default_graph_id).unwrap();
    assert_eq!(found_graph.get_label(), "friend of");
    found_graph = graphs.find_by_id_in_graphs(&from_edge_id).unwrap();
    assert_eq!(found_graph.get_from_edge().get_label(), "Earth");
}

#[test]
fn delete_from_graph() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.len(), 4);

    let _ = graphs.delete_graph_by_id(graphs.get_graphs(None).unwrap()[0].get_id(), None);
    assert_eq!(graphs.len(), 3);
}

#[test]
fn delete_from_graph_fail() {
    let mut graphs = prepare_graphs_test();
    assert!(graphs
        .delete_graph_by_id("foobar".to_string(), None)
        .is_err());
}

#[test]
fn should_update_graph() {
    let mut my_graphs = Graphs::init("my-graphs");

    let alice_edge = Edge::new("Alice");
    let bob_edge = Edge::new("Bob");
    let alice_bob_graph = Vertex::create(&alice_edge, "best friends", &bob_edge);
    my_graphs.add_graph(&alice_bob_graph, None);

    let fred_edge = Edge::new("Fred");
    let mut alice_fred_graph = Vertex::create(&alice_edge, "super friends", &fred_edge);
    my_graphs.add_graph(&alice_fred_graph, None);

    assert_eq!(my_graphs.len(), 2);

    let graphs = my_graphs.get_graphs(Some(&my_graphs.get_label())).unwrap();
    assert_eq!(graphs[1].get_relation(), "super friends");

    alice_fred_graph.update_relation("besties");
    let _ = my_graphs.update_graph(&alice_fred_graph, None);

    assert_eq!(my_graphs.len(), 2);
    let updated_graph = my_graphs.find_by_id(&alice_fred_graph.get_id(), None);
    assert_eq!(updated_graph.unwrap().get_relation(), "besties");
}

#[test]
fn should_fail_on_updating_graph() {
    let mut grphs = Graphs::init("foobar");

    let alice = Edge::new("Alice");
    let bob = Edge::new("Bob");
    let alice_bob = Vertex::create(&alice, "friend of", &bob);
    grphs.add_graph(&alice_bob, None);

    let bob_alice = Vertex::create(&bob, "friend of", &alice);
    assert!(grphs.update_graph(&bob_alice, None).is_err());
}

#[test]
fn should_return_uniq_edges_from_graph() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);
    let mut uniq_edges = graphs.get_uniq_edges(None).unwrap();
    assert_eq!(uniq_edges.len(), 2);
    let mut labels: Vec<String> = Vec::new();
    for edge in uniq_edges {
        labels.push(edge.get_label());
    }

    assert!(labels.contains(&"Saruman".to_string()));
    assert!(labels.contains(&"Gandalf".to_string()));

    uniq_edges = graphs.get_uniq_edges(Some("my graphs")).unwrap();
    assert_eq!(uniq_edges.len(), 3);
    labels.clear();
    for edge in uniq_edges {
        labels.push(edge.get_label());
    }

    assert!(labels.contains(&"Alice".to_string()));
    assert!(labels.contains(&"Bob".to_string()));
    assert!(labels.contains(&"Fred".to_string()));
}
