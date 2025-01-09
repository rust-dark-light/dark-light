build:
    echo "Building for Linux..."
    cargo build --target x86_64-unknown-linux-gnu
    echo "Building for macOS..."
    cargo build --target aarch64-apple-darwin
    echo "Building for Windows..."
    cargo build --target x86_64-pc-windows-gnu
    echo "Building for wasm..."
    cargo build --target wasm32-unknown-unknown

test:
    cargo test --doc

doc-linux:
    cargo doc --no-deps --open --target x86_64-unknown-linux-gnu

doc-macos:
    cargo doc --no-deps --open --target aarch64-apple-darwin

doc-windows:
    cargo doc --no-deps --open --target x86_64-pc-windows-gnu

doc-wasm:
    cargo doc --no-deps --open --target wasm32-unknown-unknown

example-linux:
    echo "Building examples for Linux..."
    cargo build --example detect --target x86_64-unknown-linux-gnu
example-macos:
    echo "Building examples for macOS..."
    cargo build --example detect --target aarch64-apple-darwin
example-windows:
    echo "Building examples for Windows..."
    cargo build --example detect --target x86_64-pc-windows-gnu
example-wasm:
    echo "Building examples for wasm..."
    cargo build --example detect --target wasm32-unknown-unknown
