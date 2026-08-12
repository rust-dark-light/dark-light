use std::sync::mpsc;

use crate::{Error, Mode, Watcher};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub fn detect() -> Result<Mode, Error> {
    let window = web_sys::window().ok_or(Error::WindowNotFound)?;
    let query_result = window
        .match_media("(prefers-color-scheme: dark)")
        .map_err(|_| Error::MediaQueryFailed)?;
    let mql = query_result.ok_or(Error::MediaQueryNotSupported)?;
    Ok((mql.matches()).into())
}

/// Background handle for a WASM theme watcher.
///
/// Holds the `MediaQueryList` and its `change` listener closure alive; both are removed
/// and dropped when the guard is dropped.
pub struct WatchGuard {
    mql: web_sys::MediaQueryList,
    closure: Closure<dyn FnMut(web_sys::Event)>,
}

impl Drop for WatchGuard {
    fn drop(&mut self) {
        let _ = self
            .mql
            .remove_event_listener_with_callback("change", self.closure.as_ref().unchecked_ref());
    }
}

pub fn subscribe() -> Result<Watcher, Error> {
    let window = web_sys::window().ok_or(Error::WindowNotFound)?;
    let query_result = window
        .match_media("(prefers-color-scheme: dark)")
        .map_err(|_| Error::MediaQueryFailed)?;
    let mql = query_result.ok_or(Error::MediaQueryNotSupported)?;

    let (sender, receiver) = mpsc::channel();
    let watched = mql.clone();
    let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_event: web_sys::Event| {
        let mode: Mode = watched.matches().into();
        let _ = sender.send(mode);
    });

    mql.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
        .map_err(|_| Error::MediaQueryFailed)?;

    Ok(Watcher {
        receiver,
        guard: WatchGuard { mql, closure },
    })
}
