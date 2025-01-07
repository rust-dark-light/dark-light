pub async fn detect() -> Result<crate::Mode, crate::Error> {
    if let Some(window) = web_sys::window() {
        let query_result = window.match_media("(prefers-color-scheme: dark)");
        if let Ok(Some(mql)) = query_result {
            return Ok(mql.matches().into());
        }
    }
    Err(crate::Error::DetectionFailed)
}
