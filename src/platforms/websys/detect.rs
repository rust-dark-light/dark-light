use crate::{Error, Mode};

pub fn detect() -> Result<Mode, Error> {
    let window = web_sys::window().ok_or(Error::WindowNotFound)?;
    let query_result = window
        .match_media("(prefers-color-scheme: dark)")
        .map_err(|_| Error::MediaQueryFailed)?;
    let mql = query_result.ok_or(Error::MediaQueryNotSupported)?;
    Ok((mql.matches()).into())
}
