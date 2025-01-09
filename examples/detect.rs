fn main() -> Result<(), dark_light::Error> {
    match dark_light::detect()? {
        dark_light::Mode::Dark => println!("Dark mode"),
        dark_light::Mode::Light => println!("Light mode"),
        dark_light::Mode::Unspecified => println!("Unspecified"),
    }
    Ok(())
}
