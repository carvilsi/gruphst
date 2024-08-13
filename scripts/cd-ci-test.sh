#!/bin/bash 

mv .env .env-tmp
cp .env-test .env
cargo test --verbose
mv .env-tmp .env
