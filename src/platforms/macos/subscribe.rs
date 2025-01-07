use crate::Mode;

pub async fn subscribe() -> impl futures_lite::Stream<Item = Mode> {
    Box::pin(futures_lite::stream::unfold(
        crate::detect(),
        |last_mode| async move {
            loop {
                let current_mode = crate::detect();

                if current_mode != last_mode {
                    return Some((current_mode, current_mode));
                }
            }
        },
    ))
}
