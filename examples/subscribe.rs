use futures_lite::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = dark_light::subscribe().await;
    while let Some(mode) = stream.next().await {
        println!("System mode changed: {:?}", mode);
    }
}
