use std::collections::HashMap;

use graphs_test::{prepare_graphs_test, prepare_insert_graph_test};
use gruphst::edge::Edge;
use gruphst::vertex::Vertex;

#[path = "./graphs_test.rs"]
mod graphs_test;

fn prepare_edge_test() -> (Edge, String) {
    let mut edge = Edge::new("alice");
    edge.set_attr("name", "Alice");
    edge.set_attr("age", 42);
    (edge.clone(), edge.get_id())
}

#[test]
fn edge_get_label() {
    let (edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_label(), "alice");
}

#[test]
fn edge_set_label() {
    let (mut edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_label(), "alice");
    edge.set_label("alice marcus");
    assert_eq!(edge.get_label(), "alice marcus");
}

#[test]
fn edge_get_id() {
    let (edge, id) = prepare_edge_test();
    assert_eq!(edge.get_id(), id);
}

#[test]
fn edge_get_attribute() {
    let (edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_attr("name").unwrap(), "Alice");
    assert_eq!(edge.get_attr("age").unwrap(), "42");
}

#[test]
fn edge_set_attribute() {
    let (mut edge, _id) = prepare_edge_test();
    edge.set_attr("address", "Elm Street");
    assert_eq!(edge.get_attr("name").unwrap(), "Alice");
    assert_eq!(edge.get_attr("age").unwrap(), "42");
    assert_eq!(edge.get_attr("address").unwrap(), "Elm Street");
}

#[test]
fn edge_update_attributes() {
    let (mut edge, _id) = prepare_edge_test();
    edge.update_attr("name", "Alice Marcus").unwrap();
    assert_eq!(edge.get_attr("name").unwrap(), "Alice Marcus");
    assert_eq!(edge.get_attr("age").unwrap(), "42");
}

#[test]
fn edge_fail_update_attributes() {
    let (mut edge, _id) = prepare_edge_test();
    assert!(edge.update_attr("foo", "Alice Marcus").is_err());
}

#[test]
fn edge_upsert_attributes() {
    let (mut edge, _id) = prepare_edge_test();
    edge.upsert_attr("surname", "Marcus");
    assert_eq!(edge.get_attr("name").unwrap(), "Alice");
    assert_eq!(edge.get_attr("age").unwrap(), "42");
    edge.upsert_attr("age", 43);
    assert_eq!(edge.get_attr("surname").unwrap(), "Marcus");
    assert_eq!(edge.get_attr("age").unwrap(), "43");
}

#[test]
fn edge_delete_attributes() {
    let (mut edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_attr("name").unwrap(), "Alice");
    assert_eq!(edge.get_attr("age").unwrap(), "42");
    let _ = edge.del_attr("age");
    assert_eq!(edge.get_attr("name").unwrap(), "Alice");
    assert!(edge.get_attr("age").is_err());
}

#[test]
fn edge_attribute_keys() {
    let (edge, _id) = prepare_edge_test();
    let keys = edge.get_attr_keys();
    assert!(keys.contains(&"name".to_string()));
    assert!(keys.contains(&"age".to_string()));
    assert!(!keys.contains(&"surname".to_string()));
}

#[test]
fn get_edge_relation_out() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);

    let find_results = graphs
        .has_relation_out("relative of", Some("my graphs"))
        .unwrap();
    assert_eq!(find_results.len(), 1);
    assert_eq!(find_results[0].get_label(), "Fred");
    let edge = find_results[0].clone();
    graphs.add_graph(
        &Vertex::create(&edge, "relative of", &Edge::new("Peter")),
        Some("my graphs"),
    );
    let relations_out: HashMap<String, Vec<Edge>> = edge
        .get_relations_out_on_graph(graphs.get_graphs(Some("my graphs")).unwrap())
        .unwrap();
    assert!(relations_out.contains_key("relative of"));
    assert!(relations_out.contains_key("friend of"));
    assert_eq!(relations_out.len(), 2);
    if let Some(edges) = relations_out.get("relative of") {
        assert_eq!(edges.len(), 2);
        assert_eq!(edges[0].get_label(), "Alice".to_string());
        assert_eq!(edges[1].get_label(), "Peter".to_string());
    } else {
        assert!(false);
    }
    if let Some(edges) = relations_out.get("friend of") {
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].get_label(), "Bob".to_string());
    } else {
        assert!(false);
    }
}

#[test]
fn get_edge_relation_in() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);

    let find_results = graphs
        .has_relation_in("friend of", Some("my graphs"))
        .unwrap();
    assert_eq!(find_results.len(), 2);
    let mut edge: Edge = Edge::new("tmp");
    for n in find_results {
        if n.get_label() == "Alice".to_string() {
            edge = n.clone();
        }
    }
    let relations_in: HashMap<String, Vec<Edge>> = edge
        .get_relations_in_on_graph(graphs.get_graphs(Some("my graphs")).unwrap())
        .unwrap();
    assert!(relations_in.contains_key("relative of"));
    assert!(relations_in.contains_key("friend of"));
    assert_eq!(relations_in.len(), 2);
    if let Some(edges) = relations_in.get("relative of") {
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].get_label(), "Fred".to_string());
    } else {
        assert!(false);
    }
    if let Some(edges) = relations_in.get("friend of") {
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].get_label(), "Bob".to_string());
    } else {
        assert!(false);
    }
}
