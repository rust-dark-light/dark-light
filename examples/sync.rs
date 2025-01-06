fn main() -> Result<(), Box<dyn std::error::Error>> {
    detect();
    Ok(())
}

fn detect() {
    println!("Current mode: {:?}", dark_light::sync::detect());
}
