use gruphst::graphs::Graphs;
use gruphst::graph::Graph;
use gruphst::node::Node;
use gruphst::*;

#[test]
fn persistence() {
    let mut gru = Graphs::init("graphs-a");
    let mut node1 = Node::new("a node");
    node1.set_attr("foo", "bar");
    let node2 = Node::new("b node");
    let graph1 = Graph::create(&node1, "relation a-b", &node2);
    gru.add_graph(&graph1, None);

    let node3 = Node::new("c node");
    let node4 = Node::new("d node");
    let graph2 = Graph::create(&node3, "relation c-d", &node4);
    gru.add_graph(&graph2, None);

    let _ = gru.persists();

    let name = gru.get_label();
    let file_name = format!("{}.grphst", name);
    let grphs = Graphs::load(&file_name);
    match grphs {
        Ok(grphs) => {
            let graphs = grphs.get_graphs(Some(name.as_str())).unwrap();
            assert_eq!(grphs.get_label(), name);
            assert_eq!(graphs[0].get_relation(), graph1.get_relation());
            assert_eq!(graphs[0].get_from_node().get_label(), "a node");
            assert_eq!(graphs[0].get_from_node().len_attr(), 1);
            assert_eq!(graphs[0].get_from_node().get_attr("foo").unwrap(), "bar");
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

