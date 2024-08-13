#!/bin/bash 

TAG=$(cat Cargo.toml | awk '/version\s=\s"/ { print $3 }' | head -n 1 | sed 's/"//g')
git tag v${TAG}
git push origin v${TAG}

cargo publish --dry-run && cargo publish
