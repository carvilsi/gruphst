#!/bin/bash 

# requires tarpaulin
# install it with: $ cargo install cargo-tarpaulin

mv .env .env-tmp
cp .env-test .env
cargo +nightly tarpaulin --ignore-tests --verbose --all-features --workspace --timeout 120 --out xml
mv .env-tmp .env 
