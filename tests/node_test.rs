use attributes::Attributes;
use gruphst::node::Node;
use gruphst::*;

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
