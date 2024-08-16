use std::collections::HashMap;

use graphs_test::{prepare_graphs_test, prepare_insert_graph_test};
use gruphst::{edge::Edge, vertex::Vertex};

#[path = "./graphs_test.rs"]
mod graphs_test;

fn prepare_vertex_test() -> (Vertex, String) {
    let mut vertex = Vertex::new("alice");
    vertex.set_attr("name", "Alice");
    vertex.set_attr("age", 42);
    (vertex.clone(), vertex.get_id())
}

#[test]
fn vertex_get_label() {
    let (vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_label(), "alice");
}

#[test]
fn vertex_set_label() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_label(), "alice");
    vertex.set_label("alice marcus");
    assert_eq!(vertex.get_label(), "alice marcus");
}

#[test]
fn vertex_get_id() {
    let (vertex, id) = prepare_vertex_test();
    assert_eq!(vertex.get_id(), id);
}

#[test]
fn vertex_attribute_len() {
    let (vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.attr_len(), 2);
}

#[test]
fn vertex_attribute_emptiness() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(!vertex.attr_is_empty());
    let ed = Vertex::new("Ed");
    assert!(ed.attr_is_empty());
}

#[test]
fn vertex_get_attribute() {
    let (vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_attr("name").unwrap(), "Alice");
    assert_eq!(vertex.get_attr("age").unwrap(), "42");
}

#[test]
fn vertex_set_attribute() {
    let (mut vertex, _id) = prepare_vertex_test();
    vertex.set_attr("address", "Elm Street");
    assert_eq!(vertex.get_attr("name").unwrap(), "Alice");
    assert_eq!(vertex.get_attr("age").unwrap(), "42");
    assert_eq!(vertex.get_attr("address").unwrap(), "Elm Street");
}

#[test]
fn vertex_update_attributes() {
    let (mut vertex, _id) = prepare_vertex_test();
    vertex.update_attr("name", "Alice Marcus").unwrap();
    assert_eq!(vertex.get_attr("name").unwrap(), "Alice Marcus");
    assert_eq!(vertex.get_attr("age").unwrap(), "42");
}

#[test]
fn vertex_fail_update_attributes() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert!(vertex.update_attr("foo", "Alice Marcus").is_err());
}

#[test]
fn vertex_upsert_attributes() {
    let (mut vertex, _id) = prepare_vertex_test();
    vertex.upsert_attr("surname", "Marcus");
    assert_eq!(vertex.get_attr("name").unwrap(), "Alice");
    assert_eq!(vertex.get_attr("age").unwrap(), "42");
    vertex.upsert_attr("age", 43);
    assert_eq!(vertex.get_attr("surname").unwrap(), "Marcus");
    assert_eq!(vertex.get_attr("age").unwrap(), "43");
}

#[test]
fn vertex_delete_attributes() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert_eq!(vertex.get_attr("name").unwrap(), "Alice");
    assert_eq!(vertex.get_attr("age").unwrap(), "42");
    let _ = vertex.del_attr("age");
    assert_eq!(vertex.get_attr("name").unwrap(), "Alice");
    assert!(vertex.get_attr("age").is_err());
}

#[test]
fn vertex_delete_attributes_fail_since_attribute_does_not_exists() {
    let (mut vertex, _id) = prepare_vertex_test();
    assert!(vertex.del_attr("foobar").is_err());
}

#[test]
fn vertex_attribute_keys() {
    let (vertex, _id) = prepare_vertex_test();
    let keys = vertex.get_attr_keys();
    assert!(keys.contains(&"name".to_string()));
    assert!(keys.contains(&"age".to_string()));
    assert!(!keys.contains(&"surname".to_string()));
}

#[test]
fn get_vertex_relation_out() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);

    let find_results = graphs
        .has_relation_out("relative of", Some("my graphs"))
        .unwrap();
    assert_eq!(find_results.len(), 1);
    assert_eq!(find_results[0].get_label(), "Fred");
    let vertex = find_results[0].clone();
    graphs.add_edge(
        &Edge::create(&vertex, "relative of", &Vertex::new("Peter")),
        Some("my graphs"),
    );
    let relations_out: HashMap<String, Vec<Vertex>> = vertex
        .get_relations_out_on_edges(graphs.get_edges(Some("my graphs")).unwrap())
        .unwrap();
    assert!(relations_out.contains_key("relative of"));
    assert!(relations_out.contains_key("friend of"));
    assert_eq!(relations_out.len(), 2);
    if let Some(vertexs) = relations_out.get("relative of") {
        assert_eq!(vertexs.len(), 2);
        assert_eq!(vertexs[0].get_label(), "Alice".to_string());
        assert_eq!(vertexs[1].get_label(), "Peter".to_string());
    } else {
        assert!(false);
    }
    if let Some(vertexs) = relations_out.get("friend of") {
        assert_eq!(vertexs.len(), 1);
        assert_eq!(vertexs[0].get_label(), "Bob".to_string());
    } else {
        assert!(false);
    }
}

#[test]
fn not_relations_out_in_on_vertices() {
    let graphs = prepare_graphs_test();
    let vertex = Vertex::new("solo");
    assert!(vertex
        .get_relations_out_on_edges(graphs.get_edges(Some("my graphs")).unwrap())
        .is_err());
    assert!(vertex
        .get_relations_in_on_edges(graphs.get_edges(Some("my graphs")).unwrap())
        .is_err());
}

#[test]
fn get_vertex_relation_in() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);

    let find_results = graphs
        .has_relation_in("friend of", Some("my graphs"))
        .unwrap();
    assert_eq!(find_results.len(), 2);
    let mut vertex: Vertex = Vertex::new("tmp");
    for n in find_results {
        if n.get_label() == "Alice".to_string() {
            vertex = n.clone();
        }
    }

    graphs.add_edge(
        &Edge::create(&Vertex::new("Peter"), "friend of", &vertex),
        Some("my graphs"),
    );

    let relations_in: HashMap<String, Vec<Vertex>> = vertex
        .get_relations_in_on_edges(graphs.get_edges(Some("my graphs")).unwrap())
        .unwrap();
    assert!(relations_in.contains_key("relative of"));
    assert!(relations_in.contains_key("friend of"));
    assert_eq!(relations_in.len(), 2);
    if let Some(vertexs) = relations_in.get("relative of") {
        assert_eq!(vertexs.len(), 1);
        assert_eq!(vertexs[0].get_label(), "Fred".to_string());
    } else {
        assert!(false);
    }
    if let Some(vertexs) = relations_in.get("friend of") {
        assert_eq!(vertexs.len(), 2);
        assert_eq!(vertexs[0].get_label(), "Bob".to_string());
    } else {
        assert!(false);
    }
}

#[test]
fn vertex_vec_u8_attr() {
    let (mut vertex, _id) = prepare_vertex_test();
    let vector: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    vertex.set_attr_vec_u8("vector_u8", &vector); 
    assert_eq!(vertex.get_attr_vec_u8("vector_u8").unwrap(), vector);
}
