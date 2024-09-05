use gruphst::edge::Edge;
use gruphst::graphs::Graphs;
use gruphst::vertex::Vertex;

fn prepare_persistence_test() -> (Graphs, Edge, Edge) {
    let mut gru = Graphs::init("graphs-a");
    let mut edge1 = Vertex::new("a edge");
    edge1.set_attr("foo", "bar");
    let edge2 = Vertex::new("b edge");
    let edge1 = Edge::create(&edge1, "relation a-b", &edge2);
    gru.add_edge(&edge1, None);

    let edge3 = Vertex::new("c edge");
    let edge4 = Vertex::new("d edge");
    let edge2 = Edge::create(&edge3, "relation c-d", &edge4);
    gru.add_edge(&edge2, None);

    (gru, edge1, edge2)
}

fn assertion_persisted_graphs(grphs: Graphs, name: String, edge1: Edge, edge2: Edge) {
    let graphs = grphs.get_edges(Some(name.as_str())).unwrap();
    assert_eq!(grphs.get_label(), name);
    assert_eq!(graphs[0].get_relation(), edge1.get_relation());
    assert_eq!(graphs[0].get_from_vertex().get_label(), "a edge");
    assert_eq!(graphs[0].get_from_vertex().attrs_len(), 1);
    assert_eq!(graphs[0].get_from_vertex().get_attr("foo").unwrap(), "bar");
    assert_eq!(graphs[1], edge2);
}

#[test]
fn should_persists_on_default_path() {
    let (gru, edge1, edge2) = prepare_persistence_test();

    let _ = gru.persists(None);

    let name = gru.get_label();
    let file_name = format!("{}.grphst", name);
    let grphs = Graphs::load(&file_name);
    match grphs {
        Ok(grphs) => {
            assertion_persisted_graphs(grphs, name, edge1, edge2);            
        }
        Err(_) => panic!(),
    }
}

#[test]
fn should_persists_on_custom_path() {
    let (gru, edge1, edge2) = prepare_persistence_test();

    let path = "./tests/data/";
    let _ = gru.persists(Some(path));
    let name = gru.get_label();
    let file_name = format!("{}{}.grphst", path, name);
    let grphs = Graphs::load(file_name.as_str());
    match grphs {
        Ok(grphs) => {
            assertion_persisted_graphs(grphs, name, edge1, edge2);
        }
        Err(_err) => {
            panic!()
        },
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
