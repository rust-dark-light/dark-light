use dark_light::Mode;

fn main() {
    match dark_light::detect() {
        Mode::Dark => println!("Dark mode"),
        Mode::Light => println!("Light mode"),
        Mode::Default => println!("Unspecified"),
    }
}
