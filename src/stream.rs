use futures_core::Stream;

use crate::{Error, Mode};

/// Subscribes to theme changes as an async [`Stream`].
///
/// This is an adapter over [`crate::subscribe`]. On most platforms it drives the
/// underlying blocking [`crate::Watcher`] on a background thread and forwards each
/// [`Mode`] into the returned stream. On `wasm32` (which has no threads here), the
/// browser's `MediaQueryList` `change` event is forwarded directly instead.
///
/// # Example
///
/// ``` no_run
/// use dark_light::{ Error, Mode };
/// use futures_util::StreamExt;
///
/// fn main() -> Result<(), Error> {
///     futures_executor::block_on(async {
///         let mut stream = dark_light::stream()?;
///         while let Some(mode) = stream.next().await {
///             match mode {
///                 Mode::Dark => {},
///                 Mode::Light => {},
///                 Mode::Unspecified => {},
///             }
///         }
///         Ok(())
///     })
/// }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn stream() -> Result<impl Stream<Item = Mode>, Error> {
    let watcher = crate::subscribe()?;
    let (tx, rx) = futures_channel::mpsc::unbounded();
    std::thread::spawn(move || {
        for mode in watcher.iter() {
            if tx.unbounded_send(mode).is_err() {
                break;
            }
        }
    });
    Ok(rx)
}

#[cfg(target_arch = "wasm32")]
pub fn stream() -> Result<impl Stream<Item = Mode>, Error> {
    crate::platforms::websys::stream()
}
