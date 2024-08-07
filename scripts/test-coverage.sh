#! /bin/bash 

# requires tarpaulin
# install it with: $ cargo install cargo-tarpaulin

cargo tarpaulin --ignore-tests --out Html
