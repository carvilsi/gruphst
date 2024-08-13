#!/bin/bash 

mv .env .env-tmp
cp .env-test .env
cargo test $1 -- --show-output
mv .env-tmp .env
