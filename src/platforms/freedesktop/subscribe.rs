use ashpd::desktop::settings::Settings;
use futures_lite::{Stream, StreamExt};

pub async fn subscribe() -> Result<impl Stream<Item = crate::Mode>, crate::Error> {
    let stream = Settings::new()
        .await?
        .receive_color_scheme_changed()
        .await?
        .map(crate::Mode::from);
    Ok(stream)
}
