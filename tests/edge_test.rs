use gruphst::{edge::Edge, vertex::Vertex};

fn prepare_edge_test() -> (Edge, String) {
    let mut alice = Vertex::new("alice");
    alice.set_attr("age", 42);
    let v: Vec<u8> = vec![3, 1, 3, 3, 7];
    alice.set_attr_vec_u8("code", &v);
    let bob = Vertex::new("bob");
    let mut edge = Edge::create(&alice, "friend of", &bob);
    edge.set_attr("type", "friendship");
    edge.set_attr("value", 2);
    
    (edge.clone(), edge.get_id())
}

#[test]
fn edge_add_relation_to_exisiting_edge() {
    let mut edge = Edge::new("");
    let mut alice = Vertex::new("alice");
    alice.set_attr("age", 42);
    let bob = Vertex::new("bob");
    edge.add_relation(&alice, "best friends", &bob);
    assert_eq!(edge.get_label(), "best friends");
}

#[test]
fn edge_get_relation_label() {
    let (edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_relation(), "friend of");
    // an alias
    assert_eq!(edge.get_label(), "friend of");
}

#[test]
fn edge_set_relation_label() {
    let (mut edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_label(), "friend of");
    edge.set_label("best friend of");
    assert_eq!(edge.get_label(), "best friend of");
}

#[test]
fn edge_set_relation() {
    let (mut edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_relation(), "friend of");
    edge.set_relation("best friend");
    assert_eq!(edge.get_relation(), "best friend");
}

#[test]
fn edge_get_id() {
    let (edge, id) = prepare_edge_test();
    assert_eq!(edge.get_id(), id);
}

#[test]
fn edge_attributes() {
    let (edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_attr("type").unwrap(), "friendship");
    assert_eq!(edge.get_attr("value").unwrap(), "2");
}

#[test]
fn edge_set_attribute() {
    let (mut edge, _id) = prepare_edge_test();
    edge.set_attr("weight", 5);
    assert_eq!(edge.get_attr("type").unwrap(), "friendship");
    assert_eq!(edge.get_attr("value").unwrap(), "2");
    assert_eq!(edge.get_attr("weight").unwrap(), "5");
}

#[test]
fn edge_update_attribute() {
    let (mut edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_attr("value").unwrap(), "2");
    edge.update_attr("value", 3).unwrap();
    assert_eq!(edge.get_attr("value").unwrap(), "3");
}

#[test]
fn edge_fail_update_attribute() {
    let (mut edge, _id) = prepare_edge_test();
    assert!(edge.update_attr("foo", 3).is_err());
}

#[test]
fn edge_upsert_attribute() {
    let (mut edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_attr("value").unwrap(), "2");
    edge.upsert_attr("value", 3);
    assert_eq!(edge.get_attr("value").unwrap(), "3");
    edge.upsert_attr("range", "low");
    assert_eq!(edge.get_attr("range").unwrap(), "low");
}

#[test]
fn edge_attribute_keys() {
    let (edge, _id) = prepare_edge_test();
    let keys = edge.get_attr_keys();
    assert!(keys.contains(&&"type"));
    assert!(keys.contains(&&"value"));
    assert!(!keys.contains(&&"foo"));
}

#[test]
fn edge_set_attributes() {
    let (mut edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_attr("type").unwrap(), "friendship");
    assert_eq!(edge.get_attr("value").unwrap(), "2");
    edge.set_attr("color", "black");
    edge.set_attr("weight", 5);
    assert_eq!(edge.get_attr("color").unwrap(), "black");
    assert_eq!(edge.get_attr("weight").unwrap(), "5");
}

#[test]
fn edge_get_attribute_should_fail_since_does_not_exists() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.get_attr("foobar").is_err());
}

#[test]
fn edge_update_from_vertex() {
    let (mut edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_from_vertex().get_label(), "alice");
    let vertex = Vertex::new("fred");
    edge.update_from(&vertex);
    assert_eq!(edge.get_from_vertex().get_label(), "fred");
}

#[test]
fn edge_update_to_vertex() {
    let (mut edge, _id) = prepare_edge_test();
    assert_eq!(edge.get_to_vertex().get_label(), "bob");
    let vertex = Vertex::new("fred");
    edge.update_to(&vertex);
    assert_eq!(edge.get_to_vertex().get_label(), "fred");
}

#[test]
fn should_check_if_attribute_exists_on_edge() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.has_attr_key("value"));
    assert!(!edge.has_attr_key("age"));
}

#[test]
fn should_check_if_str_attribute_key_exists_on_any_vertex_on_edge() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.has_vertex_with_attr_str_key("age"));
    assert!(!edge.has_vertex_with_attr_str_key("foo"));
}

#[test]
fn should_check_if_any_attribute_key_exists_on_any_vertex_on_edge() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.has_vertex_with_attr_key("age"));
    assert!(edge.has_vertex_with_attr_key("code"));
    assert!(!edge.has_vertex_with_attr_key("foo"));
}
#[test]
fn should_check_if_attribute_like_on_edge() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.has_attr_key_like("va"));
    assert!(!edge.has_attr_key_like("ag"));
}

#[test]
fn should_check_if_string_attribute_key_like_on_any_vertex_on_edge() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.has_vertex_with_attr_str_key_like("Ag"));
    assert!(!edge.has_vertex_with_attr_str_key_like("foo"));
}

#[test]
fn should_check_if_any_attribute_key_like_on_any_vertex_on_edge() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.has_vertex_with_attr_key_like("Ag"));
    assert!(edge.has_vertex_with_attr_key_like("OdE"));
    assert!(!edge.has_vertex_with_attr_key_like("foo"));
}

#[test]
fn should_check_if_attribute_is_equals_to() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.has_attr_equals_to("value", 2));
    assert!(!edge.has_attr_equals_to("value", 5));
    assert!(!edge.has_attr_equals_to("foo", 25));
}

#[test]
fn should_check_in_edge_if_attribute_is_equals_to() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.has_vertex_with_attr_str_value_equals_to("age", 42));
    assert!(!edge.has_vertex_with_attr_str_value_equals_to("age", 43));
    assert!(!edge.has_vertex_with_attr_str_value_equals_to("foo", 25));
}

#[test]
fn should_get_the_amount_of_attribute_for_edge() {
    let (edge, _id) = prepare_edge_test();
    assert_eq!(edge.attr_len(), 2);
}

#[test]
fn should_check_if_the_attributes_for_edge_is_empty() {
    let (edge, _id) = prepare_edge_test();
    assert!(!edge.attr_is_empty());
}

#[test]
fn should_delete_an_attribute_for_edge() {
    let (mut edge, _id) = prepare_edge_test();
    let _ = edge.delete_attr("type");
    assert_eq!(edge.attr_len(), 1);
}

#[test]
fn should_fail_to_delete_an_attribute_for_edge_if_does_not_exists() {
    let (mut edge, _id) = prepare_edge_test();
    assert!(edge.delete_attr("foobar").is_err());
}

#[test]
fn should_find_vertex_by_id() {
    let (edge, _id) = prepare_edge_test();
    let vertex_id = edge.get_to_vertex().get_id();
    let found_vertex = edge.find_vertex_by_id(vertex_id.as_str()).unwrap();
    assert_eq!(found_vertex.get_id(), vertex_id); 
}

#[test]
fn should_not_find_vertex_by_id() {
    let (edge, _id) = prepare_edge_test();
    assert!(edge.find_vertex_by_id("foobar").is_err());
}