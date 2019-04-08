#!/usr/bin/env bash
<<<<<<< HEAD
=======
python setup_markdown.py
#cargo build --target wasm32-unknown-unknown
#wasm-bindgen target/wasm32-unknown-unknown/debug/seed_homepage.wasm --no-modules --out-dir ./pkg --out-name package

>>>>>>> 120884dccd13e179489134a53d4d28393bd19370
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/seed_homepage.wasm --no-modules --out-dir ./pkg --out-name package