#!/bin/bash


rm -r target

CURR_DIR=$PWD
WASM_APPS=${PWD}/nmea_attack

cd ${WASM_APPS}

rustup target add wasm32-wasi

RUSTFLAGS="-C link-arg=--strip-all -C link-arg=-zstack-size=8192 -C link-arg=--export=__heap_base -C link-arg=--export=__data_end" cargo build --release --target wasm32-wasi

cd target/wasm32-wasi/release

xxd -i wasm_project.wasm > wasm_file.h

mv wasm_file.h ../../../../can_controllers/main/


