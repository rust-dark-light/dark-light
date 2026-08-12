#[cfg(any(feature = "tokio", feature = "async-io"))]
fn main() -> Result<(), dark_light::Error> {
    use futures_util::StreamExt;

    futures_executor::block_on(async {
        let mut stream = dark_light::stream()?;
        println!("Watching for theme changes, toggle your system theme to see updates...");
        while let Some(mode) = stream.next().await {
            match mode {
                dark_light::Mode::Dark => println!("Dark mode"),
                dark_light::Mode::Light => println!("Light mode"),
                dark_light::Mode::Unspecified => println!("Unspecified"),
            }
        }
        Ok(())
    })
}

#[cfg(not(any(feature = "tokio", feature = "async-io")))]
fn main() {
    eprintln!("This example requires the `tokio` or `async-io` feature.");
}
