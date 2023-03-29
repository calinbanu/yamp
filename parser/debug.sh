#!/bin/bash

WORKSPACE=/workspaces/yamp/parser
NAME=vsc-yamp-dev
BIN=target/debug/deps/parser-fbd61cae5f05f28e

# docker exec -ti -w $WORKSPACE $NAME cargo clean && cargo test --lib --no-run -j 1
docker exec -ti -e LC_ALL=en_US.UTF-8 -e PYTHONIOENCODING=UTF-8 -w $WORKSPACE $NAME rust-gdb -x .gdbcfg --args $BIN --test-threads 1