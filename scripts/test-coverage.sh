#!/bin/bash 

# requires tarpaulin
# install it with: $ cargo install cargo-tarpaulin

mv .env .env-tmp
cp .env-test .env

cargo tarpaulin --ignore-tests --out Html

mv .env-tmp .env 
