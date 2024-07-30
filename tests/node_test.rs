use std::collections::HashMap;

use attributes::Attributes;
use graphs_test::{prepare_graphs_test, prepare_insert_graph_test};
use gruphst::graph::Graph;
use gruphst::node::Node;
use gruphst::*;

#[path = "./graphs_test.rs"]
mod graphs_test;

fn prepare_node_test() -> (Node, String) {
    let mut node = Node::new("alice");
    node.set_attr("name", "Alice");
    node.set_attr("age", 42);
    (node.clone(), node.get_id())
}

#[test]
fn node_get_label() {
    let (node, _id) = prepare_node_test();
    assert_eq!(node.get_label(), "alice");
}

#[test]
fn node_set_label() {
    let (mut node, _id) = prepare_node_test();
    assert_eq!(node.get_label(), "alice");
    node.set_label("alice marcus");
    assert_eq!(node.get_label(), "alice marcus");
}

#[test]
fn node_get_id() {
    let (node, id) = prepare_node_test();
    assert_eq!(node.get_id(), id);
}

#[test]
fn node_attributes() {
    let (node, _id) = prepare_node_test();
    assert_eq!(node.get_attr("name").unwrap(), "Alice");
    assert_eq!(node.get_attr("age").unwrap(), "42");
}

#[test]
fn node_set_attribute() {
    let (mut node, _id) = prepare_node_test();
    node.set_attr("address", "Elm Street");
    assert_eq!(node.get_attr("name").unwrap(), "Alice");
    assert_eq!(node.get_attr("age").unwrap(), "42");
    assert_eq!(node.get_attr("address").unwrap(), "Elm Street");
}

#[test]
fn node_update_attributes() {
    let (mut node, _id) = prepare_node_test();
    node.update_attr("name", "Alice Marcus").unwrap();
    assert_eq!(node.get_attr("name").unwrap(), "Alice Marcus");
    assert_eq!(node.get_attr("age").unwrap(), "42");
}

#[test]
fn node_fail_update_attributes() {
    let (mut node, _id) = prepare_node_test();
    assert!(node.update_attr("foo", "Alice Marcus").is_err());
}

#[test]
fn node_upsert_attributes() {
    let (mut node, _id) = prepare_node_test();
    node.upsert_attr("surname", "Marcus");
    assert_eq!(node.get_attr("name").unwrap(), "Alice");
    assert_eq!(node.get_attr("age").unwrap(), "42");
    node.upsert_attr("age", 43);
    assert_eq!(node.get_attr("surname").unwrap(), "Marcus");
    assert_eq!(node.get_attr("age").unwrap(), "43");
}

#[test]
fn node_delete_attributes() {
    let (mut node, _id) = prepare_node_test();
    assert_eq!(node.get_attr("name").unwrap(), "Alice");
    assert_eq!(node.get_attr("age").unwrap(), "42");
    let _ = node.del_attr("age");
    assert_eq!(node.get_attr("name").unwrap(), "Alice");
    assert!(node.get_attr("age").is_err());
}

#[test]
fn node_attribute_keys() {
    let (node, _id) = prepare_node_test();
    let keys = node.get_attr_keys();
    assert!(keys.contains(&&"name"));
    assert!(keys.contains(&&"age"));
    assert!(!keys.contains(&&"surname"));
}

#[test]
fn node_get_attributes() {
    let (node, _id) = prepare_node_test();
    let attributes = node.get_attributes();
    assert_eq!(attributes.get_attr("name").unwrap(), "Alice");
    assert_eq!(attributes.get_attr("age").unwrap(), "42");
}

#[test]
fn node_set_attributes() {
    let (mut node, _id) = prepare_node_test();
    let attributes = node.get_attributes();
    assert_eq!(attributes.get_attr("name").unwrap(), "Alice");
    assert_eq!(attributes.get_attr("age").unwrap(), "42");
    assert_eq!(node.get_attr("name").unwrap(), "Alice");
    assert_eq!(node.get_attr("age").unwrap(), "42");
    let mut new_attributes = Attributes::new();
    new_attributes.set_attr("address", "Elm Street");
    new_attributes.set_attr("city", "Springfield");
    node.set_attributes(new_attributes);
    let update_attributes = node.get_attributes();
    assert!(update_attributes.get_attr("name").is_err());
    assert!(update_attributes.get_attr("age").is_err());
    assert_eq!(update_attributes.get_attr("address").unwrap(), "Elm Street");
    assert_eq!(update_attributes.get_attr("city").unwrap(), "Springfield");
    assert!(node.get_attr("name").is_err());
    assert!(node.get_attr("age").is_err());
    assert_eq!(node.get_attr("address").unwrap(), "Elm Street");
    assert_eq!(node.get_attr("city").unwrap(), "Springfield");
}

#[test]
fn get_node_relation_out() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);

    let find_results = graphs
        .has_relation_out("relative of", Some("my graphs"))
        .unwrap();
    assert_eq!(find_results.len(), 1);
    assert_eq!(find_results[0].get_label(), "Fred");
    let node = find_results[0].clone();
    graphs.add_graph(
        &Graph::create(&node, "relative of", &Node::new("Peter")),
        Some("my graphs"),
    );
    let relations_out: HashMap<String, Vec<Node>> = node
        .get_relations_out_on_graph(graphs.get_graphs(Some("my graphs")).unwrap())
        .unwrap();
    assert!(relations_out.contains_key("relative of"));
    assert!(relations_out.contains_key("friend of"));
    assert_eq!(relations_out.len(), 2);
    if let Some(nodes) = relations_out.get("relative of") {
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].get_label(), "Alice".to_string());
        assert_eq!(nodes[1].get_label(), "Peter".to_string());
    } else {
        assert!(false);
    }
    if let Some(nodes) = relations_out.get("friend of") {
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].get_label(), "Bob".to_string());
    } else {
        assert!(false);
    }
}

#[test]
fn get_node_relation_in() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);

    let find_results = graphs
        .has_relation_in("friend of", Some("my graphs"))
        .unwrap();
    assert_eq!(find_results.len(), 2);
    let mut node: Node = Node::new("tmp");
    for n in find_results {
        if n.get_label() == "Alice".to_string() {
            node = n.clone();
        }
    }
    let relations_in: HashMap<String, Vec<Node>> = node
        .get_relations_in_on_graph(graphs.get_graphs(Some("my graphs")).unwrap())
        .unwrap();
    assert!(relations_in.contains_key("relative of"));
    assert!(relations_in.contains_key("friend of"));
    assert_eq!(relations_in.len(), 2);
    if let Some(nodes) = relations_in.get("relative of") {
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].get_label(), "Fred".to_string());
    } else {
        assert!(false);
    }
    if let Some(nodes) = relations_in.get("friend of") {
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].get_label(), "Bob".to_string());
    } else {
        assert!(false);
    }
}
