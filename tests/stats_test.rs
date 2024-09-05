use gruphst::{edge::Edge, graphs::Graphs, vertex::Vertex};

fn prepare_stats_test() -> Graphs {
    let mut graphs = Graphs::init("friends-and-enemies");

    let mut alice = Vertex::new("Alice");
    let mut bob = Vertex::new("Bob");
    let fred = Vertex::new("Fred");
    let john = Vertex::new("John");
    let peter = Vertex::new("Peter");

    alice.set_attr("address", "Elm street");
    alice.set_attr("email", "alice@mailinator.com");
    alice.set_attr("age", 34);

    bob.set_attr("address", "Talbot street");
    bob.set_attr("email", "bob@mailinator.com");
    bob.set_attr("age", 39);

    let v: Vec<u8> = vec![3, 1, 3, 3, 7];
    bob.set_attr_vec_u8("code", &v);

    let relation_friend_of = "friend of";
    let relation_relative_of = "relative of";

    let mut edge = Edge::create(&alice, relation_friend_of, &bob);
    edge.set_attr("foo", "bar 0");
    graphs.add_edge(&edge, None);

    edge = Edge::create(&alice, relation_relative_of, &fred);
    graphs.add_edge(&edge, None);

    edge = Edge::create(&alice, "enemy of", &john);
    graphs.add_edge(&edge, None);

    edge = Edge::create(&peter, relation_relative_of, &john);
    graphs.add_edge(&edge, None);

    graphs.insert("only relatives");
    edge.set_attr("foo one", "bar 1");
    graphs.add_edge(&edge, None);
    edge = Edge::create(&bob, "brother of", &john);
    graphs.add_edge(&edge, None);

    graphs
}

#[test]
fn graphs_stats() {
    let mut graphs = prepare_stats_test();

    // XXX: Note that this could be arch dependent ¯\\(°_o)/¯
    let stats = graphs.get_stats();
    assert_eq!(stats.get_total_edges(), 6);
    assert_eq!(stats.get_total_attr(), 19);
    assert_eq!(stats.get_mem(), 2017);
    assert_eq!(stats.get_uniq_rel(), 4);
    assert_eq!(stats.get_total_graphs(), 2);
    assert_eq!(stats.get_total_vertices(), 12);
    let max_mem = 0.1_f32 * 1024.0_f32 * 1024.0_f32;
    assert_eq!(stats.get_max_mem(), max_mem as usize);
}
