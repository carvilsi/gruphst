#!/bin/bash

CHNGLG=CHANGELOG.md
NAME=$(cat Cargo.toml | awk '/name\s=/ { print $3 }' | head -n 1 | sed 's/"//g')
VERSION=$(cat Cargo.toml | awk '/version\s=\s"/ { print $3 }' | head -n 1 | sed 's/"//g')
today=$(date +%Y-%m-%d)
line="# [v${VERSION}](https:\/\/github.com\/carvilsi\/${NAME}\/releases\/tag\/v${VERSION}) (${today})"
last_release_date=$(head -n 3 CHANGELOG.md | tail -1 | awk '{ print $3 }' | sed 's/(\|)//g')
commit_message=$(git log --after=${last_release_date} --format='- %s' | grep -v 'Merge' |  sed '{:q;N;s/\n//g;t q}')

sed -i '2s/^/\nnewchangelogentry\n/' $CHNGLG
sed -i "s/newchangelogentry/${line}\n\n${commit_message}/g" $CHNGLG

