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

    let relation_friend_of = "friend of";
    let relation_relative_of = "relative of";

    let mut graph = Edge::create(&alice, relation_friend_of, &bob);
    graphs.add_edge(&graph, None);

    graph = Edge::create(&alice, relation_relative_of, &fred);
    graphs.add_edge(&graph, None);

    graph = Edge::create(&alice, relation_friend_of, &john);
    graphs.add_edge(&graph, None);

    graph = Edge::create(&peter, relation_relative_of, &john);
    graphs.add_edge(&graph, None);

    graphs.insert("only relatives");
    graphs.add_edge(&graph, None);

    graphs
}

#[test]
fn graphs_stats() {
    let graphs = prepare_stats_test();

    // XXX: Note that this could be arch dependent ¯\\(°_o)/¯
    let stats = graphs.stats().unwrap();
    assert_eq!(stats.get_len_graphs(), 5);
    assert_eq!(stats.get_total_edges(), 5);
    assert_eq!(stats.get_total_attr(), 12);
    assert_eq!(stats.get_mem(), 1519);
    assert_eq!(stats.get_uniq_rel(), 2);
    assert_eq!(stats.get_total_graphs(), 2);
}
