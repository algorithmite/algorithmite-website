#!/bin/bash

# compile and pack client wasm
cd client

# replace --dev with --release for wasm minification
wasm-pack build --target web --dev
rollup -c
uglifyjs -c -- ./pkg/bundle.js > ./pkg/bundle.min.js

sass ./src/index.scss ./pkg/bundle.css
uglifycss ./pkg/bundle.css > ./pkg/bundle.min.css

cd ..

# compile and run server
cd server

cargo run

cd ..