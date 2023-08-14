#!/bin/bash


rm -r target

CURR_DIR=$PWD
WASM_APPS=${PWD}/nmea_attack

cd ${WASM_APPS}

rustup target add wasm32-wasi

# Build wasm app
RUSTFLAGS="-C link-arg=--strip-all -C link-arg=-zstack-size=8192 -C link-arg=--export=__heap_base -C link-arg=--export=__data_end" cargo build --release --target wasm32-wasi

cd target/wasm32-wasi/release

# Convert wasm file to bytecode
xxd -i nmea_attack.wasm > nmea_attack.h

# Generate an aot file with wamrc
#wamrc --target=riscv32 -o nmea_attack.aot nmea_attack.wasm

# Convert aot file to byte code
#xxd -i nmea_attack.aot > nmea_attack_aot.h

# Move the files to main component directory
mv nmea_attack.h ../../../../can_controllers/main/

#mv nmea_attack_aot.h ../../../../can_controllers/main/


