use criterion::{criterion_group, criterion_main, Criterion};
use gruphst::{edge::Edge, graphs::Graphs, vertex::Vertex};

pub fn prepare_graphs_bench() -> Graphs {
    let mut graphs = Graphs::init("my graphs");

    let mut alice = Vertex::new("Alice");
    alice.set_attr("phone", "555-555-555");
    alice.set_attr("address", "Elm street");

    let mut bob = Vertex::new("Bob");
    bob.set_attr("age", 42);

    let fred = Vertex::new("Fred");

    graphs.add_edge(&Edge::create(&alice, "friend of", &bob), None);
    graphs.add_edge(&Edge::create(&bob, "friend of", &alice), None);
    graphs.add_edge(&Edge::create(&fred, "relative of", &alice), None);
    graphs.add_edge(&Edge::create(&fred, "friend of", &bob), None);

    graphs
}

fn add_edge(graphs: &mut Graphs, edge: &Edge) {
    graphs.add_edge(edge, None);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut graphs = prepare_graphs_bench();
    let edge: Edge = Edge::create(&Vertex::new("foo"), "relation", &Vertex::new("bar"));
    c.bench_function("add_edge", |b| b.iter(|| add_edge(&mut graphs, &edge)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

