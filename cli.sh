#!/bin/sh
echo 'cli.sh start ...'

(cd target/debug && \
./gen -m /Volumes/code/github/rust-web-server-cli/cli/mustache.config.toml -d)

echo 'cli.sh end ...'