use gruphst::{attributes::Attributes, edge::Edge, vertex::Vertex, *};

fn prepare_vertex_test() -> (Vertex, String) {
    let mut alice = Edge::new("alice");
    alice.set_attr("age", 42);
    let bob = Edge::new("bob");
    let mut vertex = Vertex::create(&alice, "friend of", &bob);
    vertex.set_attr("type", "friendship");
    vertex.set_attr("value", 2);
    (vertex.clone(), vertex.get_id())
}

#[test]
fn vertex_add_relation_to_exisiting_vertex() {
    let mut vertex = Vertex::new("");
    let mut alice = Edge::new("alice");
    alice.set_attr("age", 42);
    let bob = Edge::new("bob");
    vertex.add_relation(&alice, "best friends", &bob);
    assert_eq!(vertex.get_label(), "best friends");
}

#[test]
fn vertex_get_relation_label() {
    let (vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_relation(), "friend of");
    // an alias
    assert_eq!(vertex.get_label(), "friend of");
}

#[test]
fn vertex_set_relation_label() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_label(), "friend of");
    vertex.set_label("best friend of");
    assert_eq!(vertex.get_label(), "best friend of");
}

#[test]
fn vertex_set_relation() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_relation(), "friend of");
    vertex.set_relation("best friend");
    assert_eq!(vertex.get_relation(), "best friend");
}

#[test]
fn vertex_get_id() {
    let (vertex, id) = prepare_vertex_test();
    assert_eq!(vertex.get_id(), id);
}

#[test]
fn vertex_attributes() {
    let (vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_attr("type").unwrap(), "friendship");
    assert_eq!(vertex.get_attr("value").unwrap(), "2");
}

#[test]
fn vertex_set_attribute() {
    let (mut vertex, _id) = prepare_vertex_test();
    vertex.set_attr("weight", 5);
    assert_eq!(vertex.get_attr("type").unwrap(), "friendship");
    assert_eq!(vertex.get_attr("value").unwrap(), "2");
    assert_eq!(vertex.get_attr("weight").unwrap(), "5");
}

#[test]
fn vertex_update_attribute() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_attr("value").unwrap(), "2");
    vertex.update_attr("value", 3).unwrap();
    assert_eq!(vertex.get_attr("value").unwrap(), "3");
}

#[test]
fn vertex_fail_update_attribute() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert!(vertex.update_attr("foo", 3).is_err());
}

#[test]
fn vertex_upsert_attribute() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_attr("value").unwrap(), "2");
    vertex.upsert_attr("value", 3);
    assert_eq!(vertex.get_attr("value").unwrap(), "3");
    vertex.upsert_attr("range", "low");
    assert_eq!(vertex.get_attr("range").unwrap(), "low");
}

#[test]
fn vertex_attribute_keys() {
    let (vertex, _id) = prepare_vertex_test();
    let keys = vertex.get_attr_keys();
    assert!(keys.contains(&&"type"));
    assert!(keys.contains(&&"value"));
    assert!(!keys.contains(&&"foo"));
}

#[test]
fn vertex_get_attributes() {
    let (vertex, _id) = prepare_vertex_test();
    let attributes = vertex.get_attributes();
    assert_eq!(attributes.get_attr("type").unwrap(), "friendship");
    assert_eq!(attributes.get_attr("value").unwrap(), "2");
}

#[test]
fn vertex_set_attributes() {
    let (mut vertex, _id) = prepare_vertex_test();
    let attributes = vertex.get_attributes();
    assert_eq!(attributes.get_attr("type").unwrap(), "friendship");
    assert_eq!(attributes.get_attr("value").unwrap(), "2");
    assert_eq!(vertex.get_attr("type").unwrap(), "friendship");
    assert_eq!(vertex.get_attr("value").unwrap(), "2");
    let mut new_attributes = Attributes::new();
    new_attributes.set_attr("color", "black");
    new_attributes.set_attr("weight", 5);
    vertex.set_attributes(new_attributes);
    let update_attributes = vertex.get_attributes();
    assert!(update_attributes.get_attr("type").is_err());
    assert!(update_attributes.get_attr("value").is_err());
    assert_eq!(update_attributes.get_attr("color").unwrap(), "black");
    assert_eq!(update_attributes.get_attr("weight").unwrap(), "5");
    assert!(vertex.get_attr("type").is_err());
    assert!(vertex.get_attr("value").is_err());
    assert_eq!(vertex.get_attr("color").unwrap(), "black");
    assert_eq!(vertex.get_attr("weight").unwrap(), "5");
}

#[test]
fn vertex_update_from_edge() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_from_edge().get_label(), "alice");
    let edge = Edge::new("fred");
    vertex.update_from(&edge);
    assert_eq!(vertex.get_from_edge().get_label(), "fred");
}

#[test]
fn vertex_update_to_edge() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_to_edge().get_label(), "bob");
    let edge = Edge::new("fred");
    vertex.update_to(&edge);
    assert_eq!(vertex.get_to_edge().get_label(), "fred");
}

#[test]
fn should_check_if_attribute_exists_on_vertex() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_attr_key("value"));
    assert!(!vertex.has_attr_key("age"));
}

#[test]
fn should_check_if_attribute_exists_on_any_edge_on_vertex() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_edge_with_attr_key("age"));
    assert!(!vertex.has_edge_with_attr_key("foo"));
}

#[test]
fn should_check_if_attribute_like_on_vertex() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_attr_key_like("va"));
    assert!(!vertex.has_attr_key_like("ag"));
}

#[test]
fn should_check_if_attribute_like_on_any_edge_on_vertex() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_edge_with_attr_key_like("Ag"));
    assert!(!vertex.has_edge_with_attr_key("foo"));
}

#[test]
fn should_check_if_attribute_is_equals_to() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_attr_equals_to("value", 2));
    assert!(!vertex.has_attr_equals_to("value", 5));
    assert!(!vertex.has_attr_equals_to("foo", 25));
}

#[test]
fn should_check_in_edge_if_attribute_is_equals_to() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_edge_with_attr_value_equal("age", 42));
    assert!(!vertex.has_edge_with_attr_value_equal("age", 43));
    assert!(!vertex.has_edge_with_attr_value_equal("foo", 25));
}

#[test]
fn should_get_the_amount_of_attribute_for_vertex() {
    let (vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.attr_len(), 2);
}

#[test]
fn should_check_if_the_attributes_for_vertex_is_empty() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(!vertex.attr_is_empty());
}

#[test]
fn should_delete_an_attribute_for_vertex() {
    let (mut vertex, _id) = prepare_vertex_test();
    let _ = vertex.delete_attr("type");
    assert_eq!(vertex.attr_len(), 1);
}

#[test]
fn should_fail_to_delete_an_attribute_for_vertex_if_does_not_exists() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert!(vertex.delete_attr("foobar").is_err());
}
