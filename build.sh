#!/bin/bash

cargo web build --target=wasm32-unknown-unknown --release &&
  cp target/wasm32-unknown-unknown/release/*.js static/. &&
  cp target/wasm32-unknown-unknown/release/*.wasm static/. &&
  wasm-opt -Oz -o static/rust-wasm-pong.wasm static/rust-wasm-pong.wasm &&
  gzip --force -9 static/rust-wasm-pong.wasm &&
  mv static/rust-wasm-pong.wasm.gz static/rust-wasm-pong.wasm
