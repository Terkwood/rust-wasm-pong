#!/bin/bash

cargo web build --target=wasm32-unknown-unknown --release &&
  cp target/wasm32-unknown-unknown/release/*.js static/. &&
  cp target/wasm32-unknown-unknown/release/*.wasm static/. &&
  wasm-opt -Oz -o static/rust-wasm-pong.wasm static/rust-wasm-pong.wasm
