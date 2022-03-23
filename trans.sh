#!/bin/sh
echo 'trans.sh start ...'

(cd target/debug && \
./trans \
-s /Volumes/code/github/rust-web-server-example \
-d /Volumes/code/temp/rust-web-server-mustache)

echo 'trans.sh end ...'