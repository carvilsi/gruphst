#!/bin/bash 

nodemon -w ./.. -e rs -x 'mv .env .env-tmp; cp .env-test .env; cargo test -- --show-output; mv .env-tmp .env' 
