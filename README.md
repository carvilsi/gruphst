<div class="text" align="center">
    <img src="https://img.shields.io/github/actions/workflow/status/carvilsi/gruphst/test.yml?logo=github&label=tests" alt="test">
    <img src="https://img.shields.io/crates/v/gruphst.svg" alt="crates">
    <img src="https://codecov.io/github/carvilsi/gruphst/graph/badge.svg?token=W1XVSQB3H0"/> 
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
[Code Coverage](https://app.codecov.io/github/carvilsi/gruphst)

## Basic Usage

```rust
use gruphst::vertex::Vertex;


```

## Install

Run the following Cargo command in your project directory:

`$ cargo add gruphst`

Or add the following line to your Cargo.toml:

`gruphst = "0.11.2"`

## Tests & Coverage & Benchmarking

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

### Examples

Check the [Rock Paper Scissors Spock Lizard](https://github.com/carvilsi/gruphst/tree/main/examples/rock-paper-scissors-lizard-spock) example.

---

Thanks [@ChrisMcMStone](https://github.com/ChrisMcMStone) for all the help and memory tips ;-)

Feedback from usage and contributions are very welcome.
Also if you like it, please leave a :star: I would appreciate it ;)

