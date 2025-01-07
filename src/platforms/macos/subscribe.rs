use crate::{Error, Mode};

pub async fn subscribe() -> Result<impl futures_lite::Stream<Item = Mode>, Error> {
    let stream = futures_lite::stream::empty();
    Ok(Box::pin(stream))
}
