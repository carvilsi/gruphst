use gruphst::edge::Edge;
use gruphst::graphs::Graphs;
use gruphst::vertex::Vertex;

#[test]
fn persistence() {
    let mut gru = Graphs::init("graphs-a");
    let mut edge1 = Edge::new("a edge");
    edge1.set_attr("foo", "bar");
    let edge2 = Edge::new("b edge");
    let graph1 = Vertex::create(&edge1, "relation a-b", &edge2);
    gru.add_vertex(&graph1, None);

    let edge3 = Edge::new("c edge");
    let edge4 = Edge::new("d edge");
    let graph2 = Vertex::create(&edge3, "relation c-d", &edge4);
    gru.add_vertex(&graph2, None);

    let _ = gru.persists();

    let name = gru.get_label();
    let file_name = format!("{}.grphst", name);
    let grphs = Graphs::load(&file_name);
    match grphs {
        Ok(grphs) => {
            let graphs = grphs.get_vertices(Some(name.as_str())).unwrap();
            assert_eq!(grphs.get_label(), name);
            assert_eq!(graphs[0].get_relation(), graph1.get_relation());
            assert_eq!(graphs[0].get_from_edge().get_label(), "a edge");
            assert_eq!(graphs[0].get_from_edge().attr_len(), 1);
            assert_eq!(graphs[0].get_from_edge().get_attr("foo").unwrap(), "bar");
            assert_eq!(graphs[1], graph2);
        }
        Err(_) => panic!(),
    }
}

#[test]
fn load_persisted_fail() {
    assert!(Graphs::load("tests/does-not-exists.grphst").is_err());
    assert!(Graphs::load("tests/data/wrong-persisted-file.grphst").is_err());
}
