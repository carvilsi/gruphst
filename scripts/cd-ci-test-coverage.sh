#!/bin/bash 

# requires tarpaulin
# install it with: $ cargo install cargo-tarpaulin

mv .env-test .env
cargo tarpaulin --ignore-tests --timeout 120 --out xml
