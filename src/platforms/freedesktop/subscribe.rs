use std::error::Error;

use ashpd::desktop::settings::Settings;
use futures_lite::{Stream, StreamExt};

use crate::Mode;

pub async fn subscribe() -> impl Stream<Item = Mode> + Send {
    match color_scheme_stream().await {
        Ok(stream) => stream.boxed(),
        Err(err) => {
            log::error!("Failed to subscribe to color scheme changes: {}", err);
            futures_lite::stream::empty().boxed()
        }
    }
}

pub async fn color_scheme_stream() -> Result<impl Stream<Item = Mode> + Send, Box<dyn Error>> {
    let color_scheme_stream = Settings::new()
        .await?
        .receive_color_scheme_changed()
        .await?
        .map(Mode::from);
    Ok(Box::pin(color_scheme_stream))
}
