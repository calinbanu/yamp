#!/bin/bash

cd ./parser
rm -rf ./app/parser-wasm

wasm-pack build --target web --out-dir ../app/parser-wasm --out-name parser-wasm .
