#!/bin/bash 

mv .env .env-tmp
cp .env-test .env
cargo test
mv .env-tmp .env
