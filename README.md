<div class="text" align="center">
    <img src="https://img.shields.io/github/actions/workflow/status/carvilsi/gruphst/test.yml?logo=github&label=tests" alt="test">
    <img src="https://img.shields.io/crates/v/gruphst.svg" alt="crates">
    <p></p>
    <p>GruPHst</p>
    <p>An in-memory graph database</p>
</div> 

---

# GruPHst

An in-memory graph database.
 
Possible to persists on file (just because is something that we always expect from an in-memory databases).

Early state of development with lot of TODOs, just doing nerdy things with Graph Databases while trying to learn some Rust.

[Documentation](https://docs.rs/gruphst/latest/gruphst/)


**To run tests**

`$ cargo test`

To run tests with debug output:

`$ cargo test -- --show-output`

**Install**

Run the following Cargo command in your project directory:

`$ cargo add gruphst`

Or add the following line to your Cargo.toml:

`gruphst = "0.3.0"`


## Nodes


Creates a Node with the given name, the id is generated


```rust
use gruphst::Node;

let node = Node::new("alice node");
```

Updates the name of the Node


```rust
use gruphst::Node;

let mut node = Node::new("alice node");
node.update_name("just alice");
```

### Node Attributes

Set attributes for a Node

```rust
use gruphst::Node;

let mut node = Node::new("Alice");
node.set_attr("Address", "Elm street");
```

Get attributes for a Node

```rust
use gruphst::Node;

let mut node = Node::new("Alice");
node.set_attr("Address", "Elm street");
let attr = node.get_attr("Address").unwrap();
```

Returns an Array containing all attribute keys

```rust
use gruphst::Node;

let mut node = Node::new("Alice");
node.set_attr("Address", "Elm street");
node.set_attr("age", 44);
let keys = node.get_attr_keys();
assert!(keys.contains(&&"age"));
```

Updates the value of an attribute

```rust
use gruphst::Node;

let mut node = Node::new("Alice");
node.set_attr("Address", "Elm street");
node.set_attr("age", 44);
node.update_attr("age", 55);
```

Updates the value of an attribute or creates a new one if attribute key does not exists

```rust
use gruphst::Node;

let mut node = Node::new("Alice");
node.set_attr("Address", "Elm street");
node.upsert_attr("age", 44);
assert_eq!(node.get_attr("age").unwrap(), "44");
node.upsert_attr("age", 55);
assert_eq!(node.get_attr("age").unwrap(), "55");
```

Retrieves the lenght of attributes for a Node

```rust
use gruphst::Node;

let mut node = Node::new("Alice");
node.set_attr("Address", "Elm street");
node.set_attr("age", 25);
assert_eq!(node.len_attr(), 2);
```

Checks if attributes for a Node is empty

```rust
use gruphst::Node;

let mut node = Node::new("Alice");
assert!(node.is_empty_attr());
node.set_attr("Address", "Elm street");
node.set_attr("age", 25);
assert!(!node.is_empty_attr());
```

Deletes an attribute for a Node

```rust
use gruphst::Node;

let mut node = Node::new("Alice");
assert!(node.is_empty_attr());
node.set_attr("Address", "Elm street");
assert!(!node.is_empty_attr());
node.del_attr("Address");
assert!(node.is_empty_attr());
```

## Graph

Representation of a Graph, relating two nodes

Creates a Graph, the id is generated

```rust
use gruphst::Node;
use gruphst::Graph;

let alice = Node::new("Alice");
let bob = Node::new("Bob");
let alice_bob_graph =
    Graph::new(&alice, "friend of", &bob);
```

Updates the relation for the Graph

```rust
use gruphst::Node;
use gruphst::Graph;

let alice = Node::new("Alice");
let bob = Node::new("Bob");
let mut alice_bob_graph = Graph::new(&alice, "friend of", &bob);

assert_eq!(alice_bob_graph.relation, "friend of");

alice_bob_graph.update_relation("best friends");
assert_eq!(alice_bob_graph.relation, "best friends");
```

Updates the "from" node in Graph

```rust
use gruphst::Node;
use gruphst::Graph;

let mut alice_node = Node::new("alice node");
let bob_node = Node::new("bob node");
let mut graph = Graph::new(&alice_node, "best friends", &bob_node);
assert_eq!(graph.from.name, "alice node");
assert_eq!(graph.to.name, "bob node");
alice_node.update_name("alice");
graph.update_from(&alice_node);
assert_eq!(graph.from.name, "alice");
```

Updates the "to" node in Graph

```rust
use gruphst::Node;
use gruphst::Graph;

let alice_node = Node::new("alice node");
let bob_node = Node::new("bob node");
let mut graph = Graph::new(&alice_node, "best friends", &bob_node);
assert_eq!(graph.from.name, "alice node");
assert_eq!(graph.to.name, "bob node");
let fred_node = Node::new("fred node");
graph.update_to(&fred_node);
assert_eq!(graph.to.name, "fred node");
assert_ne!(graph.to.id, bob_node.id);
```

### Graphs

A colection of Graph

Creates a new collection of Graph elements

```rust
use gruphst::Graphs;

let my_graph = Graphs::new("my_graph");
```

Retrieves the length of the Graphs

```rust
use gruphst::{ Graphs, Node, Graph };

let mut graphs = Graphs::new("lengths");
let alice = Node::new("Alice");
let bob = Node::new("Bob");

graphs.add(&Graph::new(&alice, "friend", &bob));
graphs.add(&Graph::new(&bob, "friend", &alice));

assert_eq!(graphs.len(), 2);
```

Checks if the Graphs is empty

```rust
use gruphst::{ Graphs, Node, Graph };

let mut graphs = Graphs::new("lengths");

assert!(graphs.is_empty());

let alice = Node::new("Alice");
let bob = Node::new("Bob");

graphs.add(&Graph::new(&alice, "friend", &bob));
graphs.add(&Graph::new(&bob, "friend", &alice));

assert!(!graphs.is_empty());
```

Updates the name of the Graphs


```rust
use gruphst::Graphs;

let mut my_graph = Graphs::new("my_graph");
assert_eq!(my_graph.name, "my_graph");

my_graph.update_name("graphy");
assert_eq!(my_graph.name, "graphy");
```

Adds a Graph element to the colection

```rust
use gruphst::{ Graphs, Node, Graph };

let alice = Node::new("Alice");
let bob = Node::new("Bob");
let alice_bob_graph = Graph::new(&alice, "friend of", &bob);
let mut my_graph = Graphs::new("my_graph");
my_graph.add(&alice_bob_graph);
```

Returns a collection of Graps elements that matches the relation

```rust
use gruphst::{ Graphs, Node, Graph };

let alice = Node::new("Alice");
let bob = Node::new("Bob");
let alice_bob_graph = Graph::new(&alice, "friend of", &bob);
let mut my_graph = Graphs::new("my_graph");
my_graph.add(&alice_bob_graph);

let result_graph = my_graph.find_by_relation("friend of").unwrap();
assert_eq!(result_graph.len(), 1);
```

Returns a Graph that provided id matches with Graph, or From, To Nodes

```rust
use gruphst::{ Graphs, Node, Graph };


let mut my_graph = Graphs::new("friends");
let alice = Node::new("Alice");
let bob = Node::new("Bob");
let alice_bob = Graph::new(&alice, "is friend of", &bob);
my_graph.add(&alice_bob);

let alice_fred =
    Graph::new(&alice, "is firend of", &Node::new("Fred"));
my_graph.add(&alice_fred);

let bob_node_id = bob.id;
let res = my_graph.find_by_id(&bob_node_id);
assert_eq!(res.unwrap().to.id, bob_node_id);
```

Deletes the Graph that matches with the provided id

```rust
use gruphst::{ Graphs, Node, Graph };

let mut my_graph = Graphs::new("friends");
let alice = Node::new("Alice");
let bob = Node::new("Bob");
let alice_bob = Graph::new(&alice, "is friend of", &bob);
my_graph.add(&alice_bob);

let alice_fred =
    Graph::new(&alice, "is firend of", &Node::new("Fred"));
my_graph.add(&alice_fred);

assert_eq!(my_graph.len(), 2);

my_graph.delete_graph_by_id(alice_bob.id);
assert_eq!(my_graph.len(), 1);
```

Updates the Graphs with the provided one

```rust
use gruphst::{ Graphs, Node, Graph };

let mut my_graphs = Graphs::new("my-graphs");

let alice_node = Node::new("Alice");
let bob_node = Node::new("Bob");
let alice_bob_graph =
    Graph::new(&alice_node, "best friends", &bob_node);
my_graphs.add(&alice_bob_graph);

let fred_node = Node::new("Fred");
let mut alice_fred_graph =
    Graph::new(&alice_node, "super friends", &fred_node);
my_graphs.add(&alice_fred_graph);

assert_eq!(my_graphs.len(), 2);
assert_eq!(my_graphs.graphs[1].relation, "super friends");

alice_fred_graph.update_relation("besties");
my_graphs.update_graph(&alice_fred_graph);

assert_eq!(my_graphs.len(), 2);
let updated_graph = my_graphs.find_by_id(&alice_fred_graph.id);
assert_eq!(updated_graph.unwrap().relation, "besties");
```

### Graphs Stats

Returns stats from Graphs; size in bytes, amount of graph, name, total number of attributes and total amount of Nodes

```rust
use gruphst::{ Graphs, Node, Graph };

let mut my_graphs = Graphs::new("memories");
my_graphs.add(
    &Graph::new(
        &Node::new("Alice"),
        "recalls friendship with",
        &Node::new("Bob")
    )
);

let stats = my_graphs.stats().unwrap();
assert_eq!(stats.mem, 271);
assert_eq!(stats.len, 1);
assert_eq!(stats.name, "memories");
```

### Persistence

#### Save to file

Saves the current Graphs into a file with the Graphs's name

```rust
use gruphst::{ Graphs, Node, Graph };


let mut my_graph = Graphs::new("friends");
let alice = Node::new("Alice");
let bob = Node::new("Bob");
let alice_bob = Graph::new(&alice, "is friend of", &bob);
my_graph.add(&alice_bob);

my_graph.persists();
```

#### Load from file

Loads the persisted Graphs on a file

```rust
use gruphst::{ Graphs, Node, Graph };

let mut my_graph = Graphs::new("friends");
let alice = Node::new("Alice");
let bob = Node::new("Bob");
let alice_bob = Graph::new(&alice, "is friend of", &bob);
my_graph.add(&alice_bob);

let _ = my_graph.persists();

let name = my_graph.name;
let file_name = format!("{}.grphst", name);
let loaded_graphs = Graphs::load(&file_name);
match loaded_graphs {
    Ok(loaded_graphs) => {
        assert_eq!(loaded_graphs.name, name);
        assert_eq!(loaded_graphs.graphs[0].relation, alice_bob.relation);
    },
    Err(_) => panic!(),
}
```

## Loging

Enables logging providing a level

```rust
use gruphst::enable_logging;

enable_logging(log::Level::Info);
```
---

Feedback from usage and contributions are very welcome.
Also if you like it, please leave a :star: I would appreciate it ;)

