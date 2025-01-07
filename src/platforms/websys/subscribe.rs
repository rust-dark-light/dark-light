pub async fn subscribe() -> Result<impl futures_lite::Stream<Item = crate::Mode>, crate::Error> {
    let stream = futures_lite::stream::unfold(crate::detect().await?, |last_mode| async move {
        loop {
            match crate::detect().await {
                Ok(current_mode) => {
                    if current_mode != last_mode {
                        return Some((current_mode, current_mode));
                    }
                }
                Err(err) => {
                    log::error!("{}", err);
                }
            }
        }
    });
    Ok(stream)
}
