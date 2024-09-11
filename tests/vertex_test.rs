use std::collections::HashMap;

use graphs_test::{prepare_graphs_test, prepare_insert_graph_test};
use gruphst::{edge::Edge, vertex::Vertex};
use gruphst::errors::attributes::AttributeError;

#[path = "./graphs_test.rs"]
mod graphs_test;

fn prepare_vertex_test() -> (Vertex, String) {
    let mut vertex = Vertex::new("alice");
    vertex.set_attr("name", "Alice");
    vertex.set_attr("age", 42);
    let v: Vec<u8> = vec![3, 1, 3, 3, 7];
    vertex.set_attr_vec_u8("code", &v);
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
    assert_eq!(vertex.attrs_len(), 3);
}

#[test]
fn vertex_attribute_emptiness() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(!vertex.attrs_empty());
    let ed = Vertex::new("Ed");
    assert!(ed.attrs_empty());
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
fn vertex_attribute_str_keys() {
    let (vertex, _id) = prepare_vertex_test();
    let keys = vertex.get_attr_str_keys();
    assert!(keys.contains(&"name".to_string()));
    assert!(keys.contains(&"age".to_string()));
    assert!(!keys.contains(&"surname".to_string()));
    assert_eq!(keys.len(), 2);
}

#[test]
fn vertex_attribute_vec_u8_keys() {
    let (vertex, _id) = prepare_vertex_test();
    let keys = vertex.get_attr_vec_u8_keys();
    assert!(keys.contains(&"code".to_string()));
    assert_eq!(keys.len(), 1);
}

#[test]
fn vertex_attribute_keys() {
    let (vertex, _id) = prepare_vertex_test();
    let keys = vertex.get_attr_keys();
    assert!(keys.contains(&"code".to_string()));
    assert!(keys.contains(&"name".to_string()));
    assert!(keys.contains(&"age".to_string()));
    assert!(!keys.contains(&"surname".to_string()));
    assert_eq!(keys.len(), 3);
}

#[test]
fn get_vertex_relation_out() {
    let mut graphs = prepare_graphs_test();
    prepare_insert_graph_test(&mut graphs);

    let find_results = graphs
        .find_vertices_with_relation_out("relative of", Some("my graphs"))
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
        .find_vertices_with_relation_in("friend of", Some("my graphs"))
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
fn should_set_and_get_vertex_vec_u8_attr() {
    let (mut vertex, _id) = prepare_vertex_test();
    let vector: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    vertex.set_attr_vec_u8("vector_u8", &vector); 
    assert_eq!(vertex.get_attr_vec_u8("vector_u8").unwrap(), vector);
}

#[test]
fn should_not_get_vertex_vec_u8_attr() {
    let (mut vertex, _id) = prepare_vertex_test();
    let vector: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    vertex.set_attr_vec_u8("vector_u8", &vector); 
    let e = vertex.get_attr_vec_u8("not exists");
    assert!(e.is_err());
    assert_eq!(e, Err(AttributeError));
}

#[test]
fn should_check_if_vertex_has_an_attribute_value_like() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_attr_like("aLi"));
    assert!(!vertex.has_attr_like("oo"));
}

#[test]
fn should_find_attributes_keys_for_all_type_of_attributes() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_attr_key("name"));
    assert!(vertex.has_attr_key("code"));
    assert!(!vertex.has_attr_key("foo"));
}

#[test]
fn should_check_if_vertex_has_a_vector_u8_attribute_key() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_attr_vec_u8_key_equals_to("code"));
    assert!(!vertex.has_attr_vec_u8_key_equals_to("foobar"));
}

#[test]
fn should_check_if_vertex_has_a_vector_u8_attribute_key_like() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_attr_vec_u8_key_like("oDe"));
    assert!(!vertex.has_attr_vec_u8_key_like("bAr"));
}

#[test]
fn should_check_if_vertex_has_a_vector_u8_attribute_value_equals_to() {
    let (vertex, _id) = prepare_vertex_test();
    let v_ok: Vec<u8> = vec![3, 1, 3, 3, 7];
    let v_nok: Vec<u8> = vec![1, 0, 1];
    assert!(vertex.has_attr_vec_u8_equals_to("code", &v_ok));
    assert!(!vertex.has_attr_vec_u8_equals_to("code", &v_nok));
    assert!(!vertex.has_attr_vec_u8_equals_to("foobar", &v_ok));
}

#[test]
fn should_check_if_vertex_has_aany_attribute_key_like() {
    let (vertex, _id) = prepare_vertex_test();
    assert!(vertex.has_attr_key_like("oDe"));
    assert!(vertex.has_attr_key_like("AmE"));
    assert!(!vertex.has_attr_key_like("bAr"));
}

#[test]
fn should_store_and_validate_a_cryptographic_hashed_value() {
    let (mut vertex, _id) = prepare_vertex_test();
    let attr_hash_key = "hash_argon2";
    let plain_text = "1. The world is all that is the case.";
    vertex.set_hash(attr_hash_key, plain_text);
    assert!(vertex.is_hash_valid(attr_hash_key, plain_text).unwrap());
    assert!(!vertex.is_hash_valid(attr_hash_key, "foo bar plain text").unwrap());
    assert!(vertex.is_hash_valid("foo bar attr", plain_text).is_err());
}
