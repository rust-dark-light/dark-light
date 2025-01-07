<div align="center">
    <img src="resources/icon/icon.svg" width="200"/>
    <h1>dark-light</h1>
    <p>Rust crate to detect the system theme mode</p>
</div>

Supports macOS, Windows, Linux, BSDs, and WebAssembly. 

On Linux the XDG Desktop Portal D-Bus API is checked for the `color-scheme` preference, which works in Flatpak sandboxes without needing filesystem access.

[API Documentation](https://docs.rs/dark-light/)

## Usage

### Detect current theme mode
You can detect the current mode by using the `detect` function. This function returns a `Mode` value.
```rust
fn main() -> Result<(), dark_light::Error> {
    match dark_light::detect()? {
        dark_light::Mode::Dark => println!("Dark mode"),
        dark_light::Mode::Light => println!("Light mode"),
        dark_light::Mode::Unspecified => println!("Unspecified"),
    }
    Ok(())
}
```

### Subscribe to system theme changes
You can subscribe to system theme changes by using the `subscribe` function. This function returns a stream of `Mode` values. The stream will emit a new value whenever the system theme changes.

> [!WARNING]
> The `subscribe` function is not yet supported on macOS, Windows, and WebAssembly.
> Using it will result in an empty stream.

```rust
use futures_lite::StreamExt;

#[tokio::main]
async fn main() -> Result<(), dark_light::Error> {
    let mut stream = dark_light::subscribe().await?;
    while let Some(mode) = stream.next().await {
        println!("System mode changed: {:?}", mode);
    }
    Ok(())
}
```

## License

Licensed under either of the following licenses:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)