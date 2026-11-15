cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/caevern_portal_module.wasm ../CaevernClient/assets/modules/
cd ../CaevernClient
cargo run - --server=localhost:5000
