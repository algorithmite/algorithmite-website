#!/bin/bash

# compile and pack client wasm
cd client

# replace --dev with --release for wasm minification
cargo fmt
wasm-pack build --target web --dev
rollup -c
uglifyjs -c -- ./pkg/bundle.js > ./static/bundle.min.js

sass ./src/index.scss ./pkg/bundle.css
uglifycss ./pkg/bundle.css > ./static/bundle.min.css

cd ..

# compile and run server
cd server

cargo fmt
cargo run

cd ..