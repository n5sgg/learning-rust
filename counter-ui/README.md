# Counter App (UI)

This app uses [Yew](https://yew.rs/) to create a frontend app, which under the hood compiles into WebAssembly (and Javascript for the parts WASM can't cover yet).

## Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- `rustup target add wasm32-unknown-unknown`
- `cargo install trunk` (this one takes a while, let's have a ☕️)

## Running
```bash
trunk serve
```
