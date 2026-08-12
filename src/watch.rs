use std::sync::mpsc;

use crate::Mode;

/// A handle to a background theme-change watcher.
///
/// Dropping the `Watcher` stops the underlying platform watcher and joins its
/// background thread (where applicable).
///
/// Only mode transitions are emitted: if the OS reports the same mode twice
/// in a row, no duplicate message is sent.
pub struct Watcher {
    pub(crate) receiver: mpsc::Receiver<Mode>,
    #[allow(dead_code)]
    pub(crate) guard: crate::platforms::platform::WatchGuard,
}

impl Watcher {
    /// Blocks until the next theme change is received.
    pub fn recv(&self) -> Result<Mode, mpsc::RecvError> {
        self.receiver.recv()
    }

    /// Returns the next theme change if one is already available, without blocking.
    pub fn try_recv(&self) -> Result<Mode, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    /// Returns a blocking iterator over theme changes.
    pub fn iter(&self) -> mpsc::Iter<'_, Mode> {
        self.receiver.iter()
    }

    /// Returns a non-blocking iterator that yields only already-available theme changes.
    pub fn try_iter(&self) -> mpsc::TryIter<'_, Mode> {
        self.receiver.try_iter()
    }
}
