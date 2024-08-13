use gruphst::edge::Edge;
use gruphst::graphs::Graphs;
use gruphst::vertex::Vertex;

#[test]
fn persistence() {
    let mut gru = Graphs::init("graphs-a");
    let mut edge1 = Vertex::new("a edge");
    edge1.set_attr("foo", "bar");
    let edge2 = Vertex::new("b edge");
    let graph1 = Edge::create(&edge1, "relation a-b", &edge2);
    gru.add_edge(&graph1, None);

    let edge3 = Vertex::new("c edge");
    let edge4 = Vertex::new("d edge");
    let graph2 = Edge::create(&edge3, "relation c-d", &edge4);
    gru.add_edge(&graph2, None);

    let _ = gru.persists();

    let name = gru.get_label();
    let file_name = format!("{}.grphst", name);
    let grphs = Graphs::load(&file_name);
    match grphs {
        Ok(grphs) => {
            let graphs = grphs.get_edges(Some(name.as_str())).unwrap();
            assert_eq!(grphs.get_label(), name);
            assert_eq!(graphs[0].get_relation(), graph1.get_relation());
            assert_eq!(graphs[0].get_from_vertex().get_label(), "a edge");
            assert_eq!(graphs[0].get_from_vertex().attr_len(), 1);
            assert_eq!(graphs[0].get_from_vertex().get_attr("foo").unwrap(), "bar");
            assert_eq!(graphs[1], graph2);
        }
        Err(_) => panic!(),
    }
}

#[test]
fn load_persisted_should_fail_fail_does_not_exists() {
    assert!(Graphs::load("tests/does-not-exists.grphst").is_err());
}

#[test]
fn load_persisted_should_fail_wrong_file_format() {
    assert!(Graphs::load("tests/data/wrong-persisted-file.grphst").is_err());
}

#[test]
fn load_persisted_fail_file_size_bigger_than_max_configured_memory() {
    assert!(Graphs::load("tests/data/big-big-big.grphst").is_err());
}
