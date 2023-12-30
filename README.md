# Compose a Wasm DAG

A simple 3 component composition which shares a common `count` state.

## Compose

```bash
cargo component build --workspace
wasm-tools compose --config config.yml --output aggregate.wasm target/wasm32-wasi/debug/aggregate.wasm
```
