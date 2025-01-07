build:
    echo "Building for Linux..."
    cargo build --target x86_64-unknown-linux-gnu
    echo "Building for macOS..."
    cargo build --target x86_64-apple-darwin
    echo "Building for Windows..."
    cargo build --target x86_64-pc-windows-gnu
    echo "Building for wasm..."
    cargo build --target wasm32-unknown-unknown

test:
    cargo test --doc

doc:
    cargo doc --no-deps --open

example-linux:
    echo "Building examples for Linux..."
    cargo build --example detect --target x86_64-unknown-linux-gnu
    cargo build --example subscribe --target x86_64-unknown-linux-gnu
example-macos:
    echo "Building examples for macOS..."
    cargo build --example detect --target x86_64-apple-darwin
    cargo build --example subscribe --target x86_64-apple-darwin
example-windows:
    echo "Building examples for Windows..."
    cargo build --example detect --target x86_64-pc-windows-gnu
    cargo build --example subscribe --target x86_64-pc-windows-gnu
example-wasm:
    echo "Building examples for wasm..."
    cargo build --example detect --target wasm32-unknown-unknown        
    cargo build --example subscribe --target wasm32-unknown-unknown
