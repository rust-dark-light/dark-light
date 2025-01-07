pub async fn detect() -> Result<crate::Mode, crate::Error> {
    let window = web_sys::window().ok_or(crate::Error::WindowNotFound)?;
    let query_result = window
        .match_media("(prefers-color-scheme: dark)")
        .map_err(|_| crate::Error::MediaQueryFailed)?;
    let mql = query_result.ok_or(crate::Error::MediaQueryNotSupported)?;
    Ok((mql.matches()).into())
}
