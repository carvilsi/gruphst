use gruphst::{edge::Edge, graphs::Graphs, vertex::Vertex};

pub fn prepare_graphs_test() -> Graphs {
    let mut graphs = Graphs::init("my graphs");

    let mut alice = Vertex::new("Alice");
    alice.set_attr("phone", "555-555-555");
    alice.set_attr("address", "Elm street");

    let mut bob = Vertex::new("Bob");
    bob.set_attr("age", 42);

    let fred = Vertex::new("Fred");

    graphs.add_edge(&Edge::create(&alice, "friend of", &bob), None);
    graphs.add_edge(&Edge::create(&bob, "friend of", &alice), None);
    graphs.add_edge(&Edge::create(&fred, "relative of", &alice), None);
    graphs.add_edge(&Edge::create(&fred, "friend of", &bob), None);

    graphs
}

pub fn prepare_insert_graph_test(graphs: &mut Graphs) {
    graphs.insert("middle-earth");
    graphs.add_edge(
        &Edge::create(&Vertex::new("Gandalf"), "enemy of", &Vertex::new("Saruman")),
        Some("middle-earth"),
    );
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
        &Edge::create(
            &Vertex::new("alice"),
            "lives in",
            &Vertex::new("Springfield"),
        ),
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
    graphs.add_edge(
        &Edge::create(&Vertex::new("Earth"), "has satellite", &Vertex::new("Moon")),
        Some("solar-system"),
    );
    assert_eq!(graphs.len_graphs(), 2);
    assert_eq!(graphs.len(), 5);
}

#[test]
#[should_panic(expected = "memory usage critical, auto-persisted current graphs")]
fn insert_lot_of_edges_into_the_vault() {
    let mut graphs = prepare_graphs_test();
    graphs.update_label("big-big-big");
    for _i in 1..5500 {
        graphs.add_edge(
            &Edge::create(&Vertex::new("Earth"), "has satellite", &Vertex::new("Moon")),
            None,
        );
    }
}

#[test]
fn is_empty_graphs() {
    let mut graphs = Graphs::init("empty");
    assert!(graphs.is_empty());
    graphs = prepare_graphs_test();
    assert!(!graphs.is_empty());
}

#[test]
fn should_find_edges_by_relation() {
    let mut graphs = prepare_graphs_test();
    let vertices_found = graphs.find_edges_by_relation("friend of", None).unwrap();
    assert_eq!(vertices_found.len(), 3);
}

#[test]
fn should_not_find_edges_by_relation_since_vault_does_not_exists() {
    let mut graphs = Graphs::init("empty");
    let e = graphs.find_edges_by_relation("friend of", Some("!exists"));
    assert_eq!(e, Err("provided vault does not exists"));
}

#[test]
fn should_edges_find_by_relation_in_graphs() {
    let mut graphs = prepare_graphs_test();
    let mut vertices_found = graphs.find_edges_by_relation("friend of", None).unwrap();
    assert_eq!(vertices_found.len(), 3);
    prepare_insert_graph_test(&mut graphs);
    vertices_found = graphs
        .find_edges_by_relation("enemy of", Some("middle-earth"))
        .unwrap();
    assert_eq!(vertices_found.len(), 1);
}

#[test]
fn should_find_by_relations_name() {
    let mut graphs = prepare_graphs_test();
    let relations = vec!["friend of", "relative of"];
    let vertices_found = graphs.find_edges_by_relations(relations, None).unwrap();
    assert_eq!(vertices_found.len(), 4);
}

#[test]
fn should_not_find_by_relations_name() {
    let mut graphs = prepare_graphs_test();
    let relations = vec!["foo", "bar"];
    assert!(graphs.find_edges_by_relations(relations, None).is_err());
}

#[test]
fn should_not_find_by_relations_name_vault_does_not_exists() {
    let mut graphs = Graphs::init("void");
    let relations = vec!["foo", "bar"];
    let e = graphs.find_edges_by_relations(relations, Some("!exists"));
    assert_eq!(e, Err("provided vault does not exists"));
}

#[test]
fn should_not_find_by_relation() {
    let mut graphs = prepare_graphs_test();
    assert!(graphs.find_edges_by_relation("lol", None).is_err());
}

#[test]
fn should_return_the_unique_relations_labels_for_whole_graphs() {
    let mut graphs = prepare_graphs_test();
    let unique_relations_vertices = graphs.uniq_relations();
    assert_eq!(unique_relations_vertices, vec!["friend of", "relative of"]);
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
fn should_find_edges_with_attribute() {
    let mut graphs = prepare_graphs_test();
    let edges_found = graphs.edges_has_vertex_attr_key("age", None).unwrap();
    assert_eq!(edges_found.len(), 3);
    assert_eq!(edges_found[0].get_to_vertex().get_label(), "Bob");
}

#[test]
fn should_not_find_edges_with_attribute() {
    let mut graphs = prepare_graphs_test();
    assert!(graphs.edges_has_vertex_attr_key("foo", None).is_err());
}

#[test]
fn should_not_find_edges_with_attribute_since_vault_does_not_exists() {
    let mut graphs = prepare_graphs_test();
    let e = graphs.edges_has_vertex_attr_key("foo", Some("!exists"));
    assert_eq!(e, Err("provided vault does not exists"));
}

#[test]
fn should_find_graphs_with_attribute_like() {
    let mut graphs = prepare_graphs_test();
    let found_graphs = graphs.like_graph_edge_attr("Ag", None).unwrap();
    assert_eq!(found_graphs.len(), 3);
    assert_eq!(found_graphs[0].get_to_vertex().get_label(), "Bob");
}

#[test]
fn should_not_find_graphs_with_attribute_like() {
    let mut graphs = prepare_graphs_test();
    assert!(graphs.like_graph_edge_attr("fo", None).is_err());
}

#[test]
fn should_not_find_graphs_with_attribute_like_since_vault_does_not_exists() {
    let mut graphs = prepare_graphs_test();
    let e = graphs.like_graph_edge_attr("Ag", Some("!exists"));
    assert_eq!(e, Err("provided vault does not exists"));
}

#[test]
fn should_find_graphs_with_attribute_equal() {
    let graphs = prepare_graphs_test();
    let found_graphs = graphs.attr_equals_to("age", 42, None).unwrap();
    assert_eq!(found_graphs.len(), 3);
    assert_eq!(found_graphs[0].get_to_vertex().get_label(), "Bob");
}

#[test]
fn should_not_find_graphs_with_attribute_equal() {
    let graphs = prepare_graphs_test();
    assert!(graphs.attr_equals_to("age", 43, None).is_err());
}

#[test]
fn should_not_find_graphs_with_attribute_equal_since_vault_does_not_exists() {
    let graphs = prepare_graphs_test();
    let e = graphs.attr_equals_to("age", 43, Some("!exists"));
    assert_eq!(e, Err("provided vault does not exists"));
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
fn should_find_vertices_with_relation_in() {
    let graphs = prepare_graphs_test();
    let results = graphs.has_relation_in("friend of", None);
    assert_eq!(results.clone().unwrap().len(), 2);
    assert_eq!(results.clone().unwrap()[0].get_label(), "Bob");
    assert_eq!(results.unwrap()[1].get_label(), "Alice");
}

#[test]
fn should_not_find_vertices_with_relation_in() {
    let graphs = prepare_graphs_test();
    let results = graphs.has_relation_in("foobar", None);
    assert!(results.is_err());
}

#[test]
fn should_not_find_vertices_with_relation_in_since_vault_does_not_exists() {
    let graphs = prepare_graphs_test();
    let e = graphs.has_relation_in("foobar", Some("!exists"));
    assert_eq!(e, Err("provided vault does not exists"));
}

#[test]
fn should_find_vertices_with_relation_out() {
    let graphs = prepare_graphs_test();
    let results = graphs.has_relation_out("friend of", None);
    assert_eq!(results.clone().unwrap().len(), 3);
    assert_eq!(results.clone().unwrap()[0].get_label(), "Alice");
    assert_eq!(results.clone().unwrap()[1].get_label(), "Bob");
    assert_eq!(results.unwrap()[2].get_label(), "Fred");
}

#[test]
fn should_not_find_vertices_with_relation_out() {
    let graphs = prepare_graphs_test();
    let results = graphs.has_relation_out("foobar", None);
    assert!(results.is_err());
}

#[test]
fn should_not_find_vertices_with_relation_out_since_vault_does_not_exists() {
    let graphs = prepare_graphs_test();
    let e = graphs.has_relation_out("foobar", Some("!exists"));
    assert_eq!(e, Err("provided vault does not exists"));
}

#[test]
fn should_create_new_vault_and_add_graph() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.len_graphs(), 1);
    let graph = Edge::create(&Vertex::new("foo"), "before a", &Vertex::new("bar"));
    graphs.insert_with("other", &graph);
    assert_eq!(graphs.len_graphs(), 2);
}

#[test]
fn should_find_in_graph_by_id() {
    let mut graphs = prepare_graphs_test();
    let from_vertex = Vertex::new("Earth");
    let from_vertex_id = from_vertex.get_id();
    graphs.add_edge(
        &Edge::create(&from_vertex, "has satellite", &Vertex::new("Moon")),
        Some("solar-system"),
    );
    let mut found_graph = graphs.find_edge_by_id(&from_vertex_id, None).unwrap();
    assert_eq!(found_graph.get_label(), "has satellite");
    assert_eq!(found_graph.get_from_vertex().get_label(), "Earth");
    let default_graph_id = graphs.get_edges(Some("my graphs")).unwrap()[0].get_id();
    found_graph = graphs
        .find_edge_by_id(&default_graph_id, Some("my graphs"))
        .unwrap();
    assert_eq!(found_graph.get_label(), "friend of");
}

#[test]
fn should_find_in_graphs_by_id() {
    let mut graphs = prepare_graphs_test();
    let from_vertex = Vertex::new("Earth");
    let from_vertex_id = from_vertex.get_id();
    graphs.add_edge(
        &Edge::create(&from_vertex, "has satellite", &Vertex::new("Moon")),
        Some("solar-system"),
    );
    let default_graph_id = graphs.get_edges(Some("my graphs")).unwrap()[0].get_id();
    let mut found_graph = graphs.find_edge_by_id_in_graphs(&default_graph_id).unwrap();
    assert_eq!(found_graph.get_label(), "friend of");
    found_graph = graphs.find_edge_by_id_in_graphs(&from_vertex_id).unwrap();
    assert_eq!(found_graph.get_from_vertex().get_label(), "Earth");
}

#[test]
fn should_not_find_edges_on_graphs() {
    let mut graphs = prepare_graphs_test();
    let e = graphs.find_edge_by_id_in_graphs("000");
    assert_eq!(e, Err("edge not found"));
}

#[test]
fn delete_from_graph() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.len(), 4);

    let _ = graphs.delete_edge_by_id(graphs.get_edges(None).unwrap()[0].get_id(), None);
    assert_eq!(graphs.len(), 3);
}

#[test]
fn delete_from_graph_fail() {
    let mut graphs = prepare_graphs_test();
    assert!(graphs
        .delete_edge_by_id("foobar".to_string(), None)
        .is_err());
}

#[test]
fn delete_from_graph_fail_no_vault() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(
        graphs.delete_edge_by_id("foobar".to_string(), Some("foobar")),
        Err("no graphs in vault")
    );
}

#[test]
fn should_fail_getting_edges_since_vault_does_note_exists() {
    let graphs = prepare_graphs_test();
    assert_eq!(graphs.get_edges(Some("foobar")), Err("no graphs in vault"));
}

#[test]
fn should_update_graph() {
    let mut my_graphs = Graphs::init("my-graphs");

    let alice_edge = Vertex::new("Alice");
    let bob_edge = Vertex::new("Bob");
    let alice_bob_graph = Edge::create(&alice_edge, "best friends", &bob_edge);
    my_graphs.add_edge(&alice_bob_graph, None);

    let fred_edge = Vertex::new("Fred");
    let mut alice_fred_graph = Edge::create(&alice_edge, "super friends", &fred_edge);
    my_graphs.add_edge(&alice_fred_graph, None);

    assert_eq!(my_graphs.len(), 2);

    let graphs = my_graphs.get_edges(Some(&my_graphs.get_label())).unwrap();
    assert_eq!(graphs[1].get_relation(), "super friends");

    alice_fred_graph.update_relation("besties");
    let _ = my_graphs.update_graph(&alice_fred_graph, None);

    assert_eq!(my_graphs.len(), 2);
    let updated_graph = my_graphs.find_edge_by_id(&alice_fred_graph.get_id(), None);
    assert_eq!(updated_graph.unwrap().get_relation(), "besties");
}

#[test]
fn should_not_find_by_non_existing_id() {
    let mut graphs = prepare_graphs_test();
    assert!(graphs.find_edge_by_id("000", None).is_err());
}

#[test]
fn should_not_find_by_id_since_vault_does_not_exists() {
    let mut graphs = prepare_graphs_test();
    let e = graphs.find_edge_by_id("000", Some("!exists"));
    assert_eq!(e, Err("provided vault does not exists"));
}

#[test]
fn should_fail_on_updating_graph() {
    let mut grphs = Graphs::init("foobar");

    let alice = Vertex::new("Alice");
    let bob = Vertex::new("Bob");
    let alice_bob = Edge::create(&alice, "friend of", &bob);
    grphs.add_edge(&alice_bob, None);

    let bob_alice = Edge::create(&bob, "friend of", &alice);
    assert!(grphs.update_graph(&bob_alice, None).is_err());
}

#[test]
fn should_fail_on_updating_graph_vault_does_not_exists() {
    let mut graphs = prepare_graphs_test();
    let alice = Vertex::new("Alice");
    let bob = Vertex::new("Bob");
    let alice_bob = Edge::create(&alice, "friend of", &bob);
    assert_eq!(
        graphs.update_graph(&alice_bob, Some("foobar")),
        Err("no graphs in vault")
    );
}

#[test]
fn should_return_uniq_vertices_from_graph() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);
    let mut uniq_vertices = graphs.get_uniq_vertices(None).unwrap();
    assert_eq!(uniq_vertices.len(), 2);
    let mut labels: Vec<String> = Vec::new();
    for edge in uniq_vertices {
        labels.push(edge.get_label());
    }

    assert!(labels.contains(&"Saruman".to_string()));
    assert!(labels.contains(&"Gandalf".to_string()));

    uniq_vertices = graphs.get_uniq_vertices(Some("my graphs")).unwrap();
    assert_eq!(uniq_vertices.len(), 3);
    labels.clear();
    for edge in uniq_vertices {
        labels.push(edge.get_label());
    }

    assert!(labels.contains(&"Alice".to_string()));
    assert!(labels.contains(&"Bob".to_string()));
    assert!(labels.contains(&"Fred".to_string()));
}

#[test]
fn should_return_stats_for_graphs() {
    let mut graphs = prepare_graphs_test();
    let graphs_stats = graphs.get_stats();
    assert_eq!(graphs_stats.get_mem(), 1154);
    assert_eq!(graphs_stats.get_total_edges(), 4);
    assert_eq!(graphs_stats.get_total_graphs(), 1);
    assert_eq!(graphs_stats.get_total_attr(), 9);
    assert_eq!(graphs_stats.get_total_vertices(), 8);
    assert_eq!(graphs_stats.get_uniq_rel(), 2);
}

#[test]
fn should_retrieve_memory_used_by_graphs() {
    let graphs = prepare_graphs_test();
    let mem_usage = graphs.get_mem().unwrap();
    assert_eq!(mem_usage, 1154);
}

#[test]
fn should_set_label_for_graphs() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.get_label(), "my graphs");
    graphs.set_label("foobar");
    assert_eq!(graphs.get_label(), "foobar");
}

#[test]
fn should_update_label_for_graphs() {
    let mut graphs = prepare_graphs_test();
    assert_eq!(graphs.get_label(), "my graphs");
    graphs.update_label("foobar");
    assert_eq!(graphs.get_label(), "foobar");
}

#[test]
fn should_find_vertex_by_id() {
    let mut graphs = prepare_graphs_test();
    let a_vertex = graphs.get_edges(None).unwrap()[0].get_from_vertex();
    assert_eq!(a_vertex.get_label(), "Alice".to_string());
    let found_vertex = graphs.find_vertex_by_id(a_vertex.get_id().as_str(), None).unwrap();
    assert_eq!(found_vertex.get_id(), a_vertex.get_id());
}

#[test]
fn should_not_find_vertex_by_id_that_does_not_exists() {
    let mut graphs = prepare_graphs_test();
    let an_edge_id= graphs.get_edges(None).unwrap()[0].get_id();
    let e = graphs.find_vertex_by_id(an_edge_id.as_str(), None);
    assert_eq!(e, Err("Vertex not found"));
                    
}

#[test]
fn should_not_find_vertex_by_id_vault_does_not_exists() {
    let mut graphs = prepare_graphs_test();
    let e = graphs.find_vertex_by_id("foobar", Some("!Exists"));
    assert_eq!(e, Err("provided vault does not exists"));
}
