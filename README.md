Simple vm for executing wasm based on https://github.com/dusk-network/rusk-vm

Test-run:
```sh
rustup target add wasm32-unknown-unknown

cargo build -p compile --release --target wasm32-unknown-unknown
cargo run -p run --release 
```
