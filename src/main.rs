use gruphst::node::Node;
use gruphst::graph::Graph;
use gruphst::graphs::Graphs;
use gruphst::enable_logging;
fn main() {
    enable_logging(log::Level::Info);

    let alice = Node::new("Alice");
    let bob = Node::new("Bob");

    let mut my_graphs = Graphs::new("my graphs");
    my_graphs.add(&Graph::new(&alice, "friend", &bob));
    my_graphs.add(&Graph::new(&bob, "friends", &alice));
}
