# MacOS
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

mkdir -p "out/mac/SlitheringOnes.app/Contents/MacOS"

lipo "target/x86_64-apple-darwin/release/SpookyGame" "target/aarch64-apple-darwin/release/SpookyGame" -create -output "out/mac/SlitheringOnes.app/Contents/MacOS/SlitheringOnes"

mkdir -p "out/dmg"
pushd "out/dmg"
create-dmg --volname "Slitering Ones" --window-size 800 400 --hide-extension "SlitheringOnes.app" --app-drop-link 600 200 "slitheringones_release_mac.dmg" "out/mac/"
popd 

# Windows
cargo build --release --target x86_64-pc-windows-gnu

# Web
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/web/ --target web ./target/wasm32-unknown-unknown/release/SpookyGame.wasm
cp index.html ./out/web/