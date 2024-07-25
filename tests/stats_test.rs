use gruphst::{
    node::Node,
    graph::Graph,
    graphs::Graphs,
    CURNodeGraph,
    RUDAttribute,
};


#[test]
fn graphs_stats() {
    let mut graphs = Graphs::init("friends-and-enemies");

    let mut alice = Node::new("Alice");
    let mut bob = Node::new("Bob");
    let fred = Node::new("Fred");
    let john = Node::new("John");
    let peter = Node::new("Peter");

    alice.set_attr("address", "Elm street");
    alice.set_attr("email", "alice@mailinator.com");
    alice.set_attr("age", 34);

    bob.set_attr("address", "Talbot street");
    bob.set_attr("email", "bob@mailinator.com");
    bob.set_attr("age", 39);

    let relation_friend_of = "friend of";
    let relation_relative_of = "relative of";

    let mut graph = Graph::create(&alice, relation_friend_of, &bob);
    graphs.add_graph(&graph, None);

    graph = Graph::create(&alice, relation_relative_of, &fred);
    graphs.add_graph(&graph, None);

    graph = Graph::create(&alice, relation_friend_of, &john);
    graphs.add_graph(&graph, None);

    graph = Graph::create(&peter, relation_relative_of, &john);
    graphs.add_graph(&graph, None);

    graphs.insert("only relatives");
    graphs.add_graph(&graph, None);

    // XXX: Note that this could be arch dependent ¯\\(°_o)/¯
    let stats = graphs.stats().unwrap();
    assert_eq!(stats.get_len_graphs(), 5);
    assert_eq!(stats.get_total_nodes(), 10);
    assert_eq!(stats.get_total_attr(), 12);
    assert_eq!(stats.get_mem(), 2179);
    assert_eq!(stats.get_uniq_rel(), 2);
    assert_eq!(stats.get_total_graphs(), 2);
}