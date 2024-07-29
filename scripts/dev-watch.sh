#! /bin/bash 

nodemon -w ./.. -e rs -x 'cargo test -- --show-output' $1
