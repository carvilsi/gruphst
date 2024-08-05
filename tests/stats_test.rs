use gruphst::{vertex::Vertex, graphs::Graphs, edge::Edge};

#[test]
fn graphs_stats() {
    let mut graphs = Graphs::init("friends-and-enemies");

    let mut alice = Edge::new("Alice");
    let mut bob = Edge::new("Bob");
    let fred = Edge::new("Fred");
    let john = Edge::new("John");
    let peter = Edge::new("Peter");

    alice.set_attr("address", "Elm street");
    alice.set_attr("email", "alice@mailinator.com");
    alice.set_attr("age", 34);

    bob.set_attr("address", "Talbot street");
    bob.set_attr("email", "bob@mailinator.com");
    bob.set_attr("age", 39);

    let relation_friend_of = "friend of";
    let relation_relative_of = "relative of";

    let mut graph = Vertex::create(&alice, relation_friend_of, &bob);
    graphs.add_graph(&graph, None);

    graph = Vertex::create(&alice, relation_relative_of, &fred);
    graphs.add_graph(&graph, None);

    graph = Vertex::create(&alice, relation_friend_of, &john);
    graphs.add_graph(&graph, None);

    graph = Vertex::create(&peter, relation_relative_of, &john);
    graphs.add_graph(&graph, None);

    graphs.insert("only relatives");
    graphs.add_graph(&graph, None);

    // XXX: Note that this could be arch dependent ¯\\(°_o)/¯
    let stats = graphs.stats().unwrap();
    assert_eq!(stats.get_len_graphs(), 5);
    assert_eq!(stats.get_total_edges(), 10);
    assert_eq!(stats.get_total_attr(), 12);
    assert_eq!(stats.get_mem(), 1739);
    assert_eq!(stats.get_uniq_rel(), 2);
    assert_eq!(stats.get_total_graphs(), 2);
}
