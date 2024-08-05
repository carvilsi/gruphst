use gruphst::{attributes::Attributes, vertex::Vertex, edge::Edge_, *};

fn prepare_graph_test() -> (Vertex, String) {
    let mut alice = Edge_::new("alice");
    alice.set_attr("age", 42);
    let bob = Edge_::new("bob");
    let mut graph = Vertex::create(&alice, "friend of", &bob);
    graph.set_attr("type", "friendship");
    graph.set_attr("value", 2);
    (graph.clone(), graph.get_id())
}

#[test]
fn graph_get_relation() {
    let (graph, _id) = prepare_graph_test();
    assert_eq!(graph.get_relation(), "friend of");
    // an alias
    assert_eq!(graph.get_label(), "friend of");
}

#[test]
fn graph_set_relation() {
    let (mut graph, _id) = prepare_graph_test();
    assert_eq!(graph.get_relation(), "friend of");
    graph.set_relation("best friend");
    assert_eq!(graph.get_relation(), "best friend");
}

#[test]
fn graph_get_id() {
    let (graph, id) = prepare_graph_test();
    assert_eq!(graph.get_id(), id);
}

#[test]
fn graph_attributes() {
    let (graph, _id) = prepare_graph_test();
    assert_eq!(graph.get_attr("type").unwrap(), "friendship");
    assert_eq!(graph.get_attr("value").unwrap(), "2");
}

#[test]
fn graph_set_attribute() {
    let (mut graph, _id) = prepare_graph_test();
    graph.set_attr("weight", 5);
    assert_eq!(graph.get_attr("type").unwrap(), "friendship");
    assert_eq!(graph.get_attr("value").unwrap(), "2");
    assert_eq!(graph.get_attr("weight").unwrap(), "5");
}

#[test]
fn graph_update_attribute() {
    let (mut graph, _id) = prepare_graph_test();
    assert_eq!(graph.get_attr("value").unwrap(), "2");
    graph.update_attr("value", 3).unwrap();
    assert_eq!(graph.get_attr("value").unwrap(), "3");
}

#[test]
fn graph_fail_update_attribute() {
    let (mut graph, _id) = prepare_graph_test();
    assert!(graph.update_attr("foo", 3).is_err());
}

#[test]
fn graph_upsert_attribute() {
    let (mut graph, _id) = prepare_graph_test();
    assert_eq!(graph.get_attr("value").unwrap(), "2");
    graph.upsert_attr("value", 3);
    assert_eq!(graph.get_attr("value").unwrap(), "3");
    graph.upsert_attr("range", "low");
    assert_eq!(graph.get_attr("range").unwrap(), "low");
}

#[test]
fn graph_attribute_keys() {
    let (graph, _id) = prepare_graph_test();
    let keys = graph.get_attr_keys();
    assert!(keys.contains(&&"type"));
    assert!(keys.contains(&&"value"));
    assert!(!keys.contains(&&"foo"));
}

#[test]
fn graph_get_attributes() {
    let (graph, _id) = prepare_graph_test();
    let attributes = graph.get_attributes();
    assert_eq!(attributes.get_attr("type").unwrap(), "friendship");
    assert_eq!(attributes.get_attr("value").unwrap(), "2");
}

#[test]
fn graph_set_attributes() {
    let (mut graph, _id) = prepare_graph_test();
    let attributes = graph.get_attributes();
    assert_eq!(attributes.get_attr("type").unwrap(), "friendship");
    assert_eq!(attributes.get_attr("value").unwrap(), "2");
    assert_eq!(graph.get_attr("type").unwrap(), "friendship");
    assert_eq!(graph.get_attr("value").unwrap(), "2");
    let mut new_attributes = Attributes::new();
    new_attributes.set_attr("color", "black");
    new_attributes.set_attr("weight", 5);
    graph.set_attributes(new_attributes);
    let update_attributes = graph.get_attributes();
    assert!(update_attributes.get_attr("type").is_err());
    assert!(update_attributes.get_attr("value").is_err());
    assert_eq!(update_attributes.get_attr("color").unwrap(), "black");
    assert_eq!(update_attributes.get_attr("weight").unwrap(), "5");
    assert!(graph.get_attr("type").is_err());
    assert!(graph.get_attr("value").is_err());
    assert_eq!(graph.get_attr("color").unwrap(), "black");
    assert_eq!(graph.get_attr("weight").unwrap(), "5");
}

#[test]
fn graph_update_from_edge() {
    let (mut graph, _id) = prepare_graph_test();
    assert_eq!(graph.get_from_edge().get_label(), "alice");
    let edge = Edge_::new("fred");
    graph.update_from(&edge);
    assert_eq!(graph.get_from_edge().get_label(), "fred");
}

#[test]
fn graph_update_to_edge() {
    let (mut graph, _id) = prepare_graph_test();
    assert_eq!(graph.get_to_edge().get_label(), "bob");
    let edge = Edge_::new("fred");
    graph.update_to(&edge);
    assert_eq!(graph.get_to_edge().get_label(), "fred");
}

#[test]
fn should_check_if_attribute_exists_on_graph() {
    let (graph, _id) = prepare_graph_test();
    assert!(graph.has_attr("value"));
    assert!(!graph.has_attr("age"));
}

#[test]
fn should_check_if_attribute_exists_on_any_edge_on_graph() {
    let (graph, _id) = prepare_graph_test();
    assert!(graph.has_edge_attr("age"));
    assert!(!graph.has_edge_attr("foo"));
}

#[test]
fn should_check_if_attribute_like_on_graph() {
    let (graph, _id) = prepare_graph_test();
    assert!(graph.like_attr("va"));
    assert!(!graph.like_attr("ag"));
}

#[test]
fn should_check_if_attribute_like_on_any_edge_on_graph() {
    let (graph, _id) = prepare_graph_test();
    assert!(graph.like_edge_attr("Ag"));
    assert!(!graph.has_edge_attr("foo"));
}

#[test]
fn should_check_if_attribute_is_equals_to() {
    let (graph, _id) = prepare_graph_test();
    assert!(graph.equals_attr("value", 2));
    assert!(!graph.equals_attr("value", 5));
    assert!(!graph.equals_attr("foo", 25));
}

#[test]
fn should_check_in_edge_if_attribute_is_equals_to() {
    let (graph, _id) = prepare_graph_test();
    assert!(graph.equals_edge_attr("age", 42));
    assert!(!graph.equals_edge_attr("age", 43));
    assert!(!graph.equals_edge_attr("foo", 25));
}
