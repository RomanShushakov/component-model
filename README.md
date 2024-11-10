## Component Model

This is a simple example of webassembly component model.

### Prerequisites

- `cargo install cargo-component`
- `cargo install wasm-tools`
- `cargo install wac-cli`
- `cargo install wasmtime-cli`

### Commands

- `cargo component new adder --lib && cd adder`
- `cargo component build --release`
- `wasm-tools component wit target/wasm32-wasip1/release/adder.wasm`

- `cd ..`

- `cargo component new calculator --lib && cd calculator`
- `cargo component build --release`
- `wasm-tools component wit target/wasm32-wasip1/release/calculator.wasm`

- `cd ..`


- `cargo component new command && cd command`
- `cargo component build --release`
- `wasm-tools component wit target/wasm32-wasip1/release/command.wasm`

- `cd ..`

- `mkdir app`

- `(cd calculator && cargo component build --release)
(cd adder && cargo component build --release)
(cd command && cargo component build --release)
wac plug calculator/target/wasm32-wasip1/release/calculator.wasm --plug adder/target/wasm32-wasip1/release/adder.wasm -o app/composed.wasm
wac plug command/target/wasm32-wasip1/release/command.wasm --plug app/composed.wasm -o app/app.wasm`

`wasmtime run app/app.wasm 1 2.5 add`