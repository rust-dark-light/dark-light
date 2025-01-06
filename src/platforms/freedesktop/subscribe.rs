use futures_lite::{Stream, StreamExt};

pub async fn subscribe() -> impl Stream<Item = crate::Mode> + Send {
    match super::color_scheme_stream().await {
        Ok(stream) => stream.boxed(),
        Err(err) => {
            log::error!("Failed to subscribe to color scheme changes: {}", err);
            futures_lite::stream::empty().boxed()
        }
    }
}
