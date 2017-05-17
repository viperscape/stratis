#!/bin/sh

## this will start the pan app service and watch for cargo builds
## if you have Windows, download git tools which includes gitbash, and can run shell scripts

cd ./pan/
cargo build
cd ..

./pan/target/debug/pan.exe -rwd
