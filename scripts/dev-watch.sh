#! /bin/bash 

edgemon -w ./.. -e rs -x 'cargo test -- --show-output' $1
