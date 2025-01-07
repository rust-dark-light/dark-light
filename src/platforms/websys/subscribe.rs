pub async fn subscribe() -> Result<impl futures_lite::Stream<Item = crate::Mode>, crate::Error> {
    let stream = futures_lite::stream::empty();
    Ok(Box::pin(stream))
}
