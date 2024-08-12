#! /bin/bash 

mv .env .env-tmp
cp .env-test .env

cargo bench

mv .env-tmp .env 
