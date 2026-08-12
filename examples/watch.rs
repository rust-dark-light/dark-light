fn main() -> Result<(), dark_light::Error> {
    let watcher = dark_light::subscribe()?;
    println!("Watching for theme changes, toggle your system theme to see updates...");
    for mode in watcher.iter() {
        match mode {
            dark_light::Mode::Dark => println!("Dark mode"),
            dark_light::Mode::Light => println!("Light mode"),
            dark_light::Mode::Unspecified => println!("Unspecified"),
        }
    }
    Ok(())
}
