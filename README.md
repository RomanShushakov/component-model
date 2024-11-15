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

- `cargo component new subtractor --lib && cd subtractor`
- `cargo component build --release`
- `wasm-tools component wit target/wasm32-wasip1/release/subtractor.wasm`

- `cd ..`

- `mkdir multiplier`
- `python3 -m venv venv`
- `source venv/bin/activate`
- `pip install componentize-py`
- `mkdir wasm`
- `componentize-py --wit-path wit/multiplier.wit --world multiplier componentize app -o wasm/multiplier.wasm`

- `cd ..`

- `mkdir divider`
- `python3 -m venv venv`
- `source venv/bin/activate`
- `pip install componentize-py`
- `mkdir wasm`
- `componentize-py --wit-path wit/divider.wit --world divider componentize app -o wasm/divider.wasm`

- `cd ..`

- `npm install`
- `mkdir wasm`
- `npx jco componentize divider-jco.js --wit wit/divider-jco.wit --world-name divider-jco --out wasm/divider-jco.wasm --disable all`

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
(cd subtractor && cargo component build --release)
(cd multiplier && python3 -m venv venv && source venv/bin/activate && pip install -r requirements.txt && componentize-py --wit-path wit/multiplier.wit --world multiplier componentize app -o wasm/multiplier.wasm)
(cd divider && python3 -m venv venv && source venv/bin/activate && pip install -r requirements.txt && componentize-py --wit-path wit/divider.wit --world divider componentize app -o wasm/divider.wasm)
(cd divider-jco && npm install && npx jco componentize \
    divider-jco.js \
    --wit wit/divider-jco.wit \
    --world-name divider-jco \
    --out wasm/divider-jco.wasm \
    --disable all)
(cd command && cargo component build --release)
wac plug calculator/target/wasm32-wasip1/release/calculator.wasm --plug adder/target/wasm32-wasip1/release/adder.wasm --plug subtractor/target/wasm32-wasip1/release/subtractor.wasm --plug multiplier/wasm/multiplier.wasm --plug divider/wasm/divider.wasm --plug divider-jco/wasm/divider-jco.wasm -o app/composed.wasm
wac plug command/target/wasm32-wasip1/release/command.wasm --plug app/composed.wasm -o app/app.wasm`

- `wasmtime run app/app.wasm 1 2.5 add`
- `wasmtime run app/app.wasm 1 2.5 subtract`
- `wasmtime run app/app.wasm 50 2.5 multiply`
- `wasmtime run app/app.wasm 50 2.5 divide`
- `wasmtime run app/app.wasm 50 0.0 divide`
- `wasmtime run app/app.wasm 50 2.5 divide-jco`
- `wasmtime run app/app.wasm 50 0.0 divide-jco`
