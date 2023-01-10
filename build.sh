#!/bin/sh

wasm-pack build --debug --target web --out-dir $PWD/src/pyserver/static/pkg/
