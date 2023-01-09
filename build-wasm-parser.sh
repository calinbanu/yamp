#!/bin/bash

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

cd $SCRIPTPATH/parser
rm -rf ./app/parser-wasm

wasm-pack build --target bundler --out-dir ../app/parser-wasm --out-name parser-wasm .
