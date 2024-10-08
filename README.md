<div class="text" align="center">
    <img src="https://img.shields.io/github/actions/workflow/status/carvilsi/gruphst/test.yml?logo=github&label=tests" alt="test">
    <img src="https://img.shields.io/crates/v/gruphst.svg" alt="crates">
    <img src="https://img.shields.io/docsrs/gruphst/latest" alt="docs">
    <img src="https://codecov.io/github/carvilsi/gruphst/graph/badge.svg?token=W1XVSQB3H0" alt="cov"/> 
    <img src="https://img.shields.io/badge/License-MIT-purple.svg" alt="cov"/> 
    <p></p>
    <p>GruPHst</p>
    <p>An in-memory graph database</p>
</div> 

---

# GruPHst

Possible to persists on file (just because is something that we always expect from an in-memory databases).

Early state of development with lot of TODOs, just doing nerdy things with Graph Databases while trying to learn some Rust.

[Documentation](https://docs.rs/gruphst/latest/gruphst/)

[Code Coverage](https://app.codecov.io/github/carvilsi/gruphst)

---

1. [Basic Usage](#basic-usage)
2. [Install](#install)
3. [Tests & Coverage & Benchmarking](#tests-coverage-benchmarking)
4. [Configuration](#configuration)
    1. [Configurable variables](#configurable-variables)
    2. [Maximum memory usage](#maximum-memory-usage)
    3. [Level for logging](#level-for-logging) 
    4. [Character delimiter for CSV file](#character-delimiter-for-csv-file)
5. [Save & Load](#save-load)
6. [Export & Import](#export-import)
    1. [CSV](#csv)
        1. [File Format](#file-format)
        2. [Export & Import Usage](#export-import-usage)
7. [Cryptography](#cryptography)
    1. [Argon2 Hashes](#argon2-hashes)
8. [Examples](#examples)

---

## Basic Usage<a name="basic-usage">

```rust
use gruphst::{edge::Edge, graphs::Graphs, vertex::Vertex};
use std::error::Error;

// The idea it's to create some graph related with 
// the Middle-Earth, relating some characters and
// places

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new vertex
    let frodo = Vertex::new("Frodo");
       
    // Let's create another vertex
    let mut gandalf = Vertex::new("Gandalf");
     
    // A vertex can have attributes
    gandalf.set_attr("known as", "The Gray");
    gandalf.set_attr("years old", 24000);

    // Now lets make a relation between these two friends
    // by creating an Edge
    let mut edge = Edge::create(&gandalf, "friend of", &frodo);

    // An Edge can have attributes
    edge.set_attr("duration in years", 42);

    // Now we need something to hold, and store the created Edge
    // and the new ones that we'll create later.
    // Lets init a Graphs, we could do this step at the begining 
    // of the main function.
    let mut graphs = Graphs::init("middle-earth");
    
    // Now we add the edge or relation between Gandalf and Frodo
    graphs.add_edge(&edge, None);

    // We can add another relation or Edge to the graphs
    // for these two friends, e.g.
    graphs.add_edge(&Edge::create(&frodo, "has best friend", &gandalf), None);

    // Lets create more vertices for places and characters and edges
    // for the relation between them
    let mut sam = Vertex::new("Samwise");
    sam.set_attr("surname", "Gamgee");
    graphs.add_edge(
        &Edge::create(
            &sam,
            "has best friend",
            &frodo),
        None);

    let mut vertex = Vertex::new("The Shire");

    // Vertices and Edges has a uuid generated on creation
    let id_vertex_the_shire = vertex.get_id();

    graphs.add_edge(&Edge::create(&frodo, "lives at", &vertex), None); 

    vertex = Vertex::new("Isengard");
    vertex.set_attr("type", "tower");

    graphs.add_edge(&Edge::create(&Vertex::new("Saruman"), "lives at", &vertex), None); 

    // we can use the id or the label to retrieve a Vertex that we have on Graph
    let the_shire = graphs.find_vertex_by_id(id_vertex_the_shire.as_str(), None)?;

    graphs.add_edge(&Edge::create(&sam, "lives at", &the_shire), None); 

    // Now we can do things like get stats of the Graphs
    let stats = graphs.get_stats();

    // and print it
    println!("{:#?}", stats);
    // GraphsStats {
    //    mem: 1578,
    //    total_edges: 6,
    //    total_graphs: 1,
    //    total_attr: 8,
    //    total_vertices: 12,
    //    uniq_rel: 3,
    //    max_mem: 104857600,
    // }
    
    // or get some value from stats
    // like the amount of vertices
    assert_eq!(stats.get_total_vertices(), 12);

    // We can print the current Graphs object
    println!("{:#?}", graphs);

    // We can retrieve the uniq relations from the graph
    let unique_relations_vertices = graphs.uniq_relations();
    assert_eq!(unique_relations_vertices, vec!["friend of", "has best friend", "lives at"]);

    // Also possible to retrieve the vertices that has a certain
    // relation in
    let vertices_with_relation_in = graphs.find_vertices_with_relation_in("lives at", None)?; 
    assert_eq!(vertices_with_relation_in[0].get_label(), "The Shire");
    assert_eq!(vertices_with_relation_in[1].get_label(), "Isengard");

    // Or get the edge that has a vertex with an attribute equals to
    let found = graphs.find_edges_with_vertex_attr_str_equals_to("years old", 24000, None)?;
    assert_eq!(found[0].get_from_vertex().get_label(), "Gandalf");

    // Since we have a humble middle-earth network
    // we can persists it for another day
    // a file called "middle-earth.grphst" will be created, 
    // later we can load it with:
    // let loaded_graphs = Graphs::load("middle-earth.grphst")?;
    graphs.save(None)?;

    Ok(())
}
```

## Install<a name="install">

Run the following Cargo command in your project directory:

`$ cargo add gruphst`

Or add the following line to your Cargo.toml:

`gruphst = "0.15.0"`

## Tests & Coverage & Benchmarking<a name="tests-coverage-benchmarking">

**To run tests locally**
This will show output, if a test name is provided as argument will run this tests 

`$ ./scripts/local-test.sh`

If nodemon is installed, you can use the tests in watch mode:

`$ ./scripts/dev-watch.sh`

**Coverage**

`$ ./scripts/test-coverage.sh`

It will generate a report called *tarpauling-report.html*

**Benchmarking**

`$ ./scripts/benchmarking.sh`

Right now only covers *add_edge* method.

## Configuration<a name="configuration">

GruPHst uses [dotenv](https://docs.rs/dotenv/latest/dotenv/index.html) to deal with configurations.
You can place a *.env* file in order to handle your configuration values or you can use *environment variables* instead to run your binary. The *environmental variables* will override the configuration from *.env* file.

*e.g. override log level in your binary:*

`$ GRUPHST_LOG_LEVEL=trace cargo run`

This is the currnet *.env* file:

```toml
# limit for memory usage in MB
GRUPHST_MAX_MEM_USAGE=100 

# log level, case insensitive, possible values:
# trace
# debug
# info
# warn 
# warning
# err
# error
GRUPHST_LOG_LEVEL=info

# delimiter character for CSV import-export 
GRUPHST_CSV_DELIMITER=;
```

### Configurable variables<a name="configurable-variables">

#### Maximum memory usage<a name="maximum-memory-usage">

Configures the maximum memory in **MB** that GruPHst will use. In case that this limit will reach, before **panic** will persists the current status.

`GRUPHST_MAX_MEM_USAGE=100`

#### Level for logging<a name="level-for-logging">

Sets the level for logging in case insensitive, the possible values are:

- trace
- debug
- info
- warn 
- warning
- err
- error

`GRUPHST_LOG_LEVEL=info`

In order to use it on your binary:

```rust
// import from config and logger level
use gruphst::config::get_log_level;
use gruphst::logger::enable_logging;

// get the configured log level; on .env file or environmental
let log_level = get_log_level();

// enable logging
enable_logging(log_level);
```

#### Character delimiter for CSV file<a name="character-delimiter-for-csv-file">

Configures the character used to import and export for CSV format.

`GRUPHST_CSV_DELIMITER=;`

## Save & Load<a name="save-load">

You can persists the data on a file in GruPHst format.
And later load the saved data.

```rust
use gruphst::graphs::Graphs;
use gruphst::edge::Edge;
use gruphst::vertex::Vertex;

let mut graphs = Graphs::init("to_export");
let foo = Vertex::new("foo");
let bar = Vertex::new("bar");
graphs.add_edge(&Edge::create(&foo, "is related to", &bar), None);

// persists the graphs data on file, 
// with "./to_export.grphst"
graphs.save(Some("./"));

// load the saved data
let saved_graphs = Graphs::load("./to_export.grphst").unwrap();
```

## Export & Import<a name="export-import">

### CSV<a name="csv">

The **delimiter** could be configured with **GRUPHST_CSV_DELIMITER** variable, via *.env* file or with *environmental var* usage. The default character is '**;**'.

#### File Format<a name="file-format">

**Headers:**

```csv
graphs_vault;from_label;from_attributes;relation;to_label;to_attributes
```

**Row example:**
```csv
shire-friendships;gandalf;known as: Gandalf the Gray | name: Gandalf;friend of;frodo;name: Frodo Bolson
```

**Note:**
The different attributes are separated by '|' character and key followed by ':' and vaule.

#### Export & Import Usage<a name="export-import-usage">

```rust
use gruphst::graphs::Graphs;
use gruphst::edge::Edge;
use gruphst::vertex::Vertex;
use gruphst::exporter_importer::csv::*;

let mut graphs = Graphs::init("to_export");
let foo = Vertex::new("foo");
let bar = Vertex::new("bar");
graphs.add_edge(&Edge::create(&foo, "is related to", &bar), None);

// export graphs to CSV file
export_to_csv_gruphst_format(&graphs, Some("./"), Some("export_csv_filename")).unwrap();

// import graphs from CSV file
let graphs: Graphs = import_from_csv_gruphst_format("./export_csv_filename.csv").unwrap();
```

## Cryptography<a name="cryptography">

### Argon2 Hashes<a name="argon2-hashes">

You can use [Argon2](https://docs.rs/argon2/latest/argon2/) to store passwords or whatever sensible data you are dealing with, and verify it.

```rust
use gruphst::vertex::Vertex;

// create a vertex
let mut vertex = Vertex::new("Brian");
// set an Argon2 hash
vertex.set_hash("password", "53cr37");

// Check if the provided value is valid
assert!(vertex.is_hash_valid("password", "53cr37").unwrap());
assert!(!vertex.is_hash_valid("password", "f00b4r").unwrap());
```

## Examples<a name="examples">

Check the [Rock Paper Scissors Spock Lizard](https://github.com/carvilsi/gruphst/tree/main/examples/rock-paper-scissors-lizard-spock) example.

Check the [Middle-Earth](https://github.com/carvilsi/gruphst/tree/main/examples/middle-earth) example.

Also worth to check the [tests](https://github.com/carvilsi/gruphst/tree/main/tests) folder.

---

Thanks [@ChrisMcMStone](https://github.com/ChrisMcMStone) for all the help and memory tips ;-)

Feedback from usage and contributions are very welcome.
Also if you like it, please leave a :star: I would appreciate it ;)

