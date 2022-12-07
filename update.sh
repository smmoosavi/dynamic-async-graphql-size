#!/usr/bin/env bash

cargo build --release
stat -c "%s %n" target/release/dynamic-graphql target/release/static-graphql > size

cat size

echo "static graphql"
target/release/static-graphql > static-graphql/schema.graphql

echo "dynamic graphql"
target/release/dynamic-graphql > dynamic-graphql/schema.graphql

diff static-graphql/schema.graphql dynamic-graphql/schema.graphql