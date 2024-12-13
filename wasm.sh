echo "ビルドを開始"
cargo build --release --target wasm32-unknown-unknown

echo "Wasmに変換"
wasm-bindgen --target web --out-dir ./examples target/wasm32-unknown-unknown/release/bevy-2dshooting-game.wasm
