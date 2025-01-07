use futures_lite::StreamExt;

#[tokio::main]
async fn main() -> Result<(), dark_light::Error> {
    let mut stream = dark_light::subscribe().await?;
    while let Some(mode) = stream.next().await {
        println!("System mode changed: {:?}", mode);
    }
    Ok(())
}
