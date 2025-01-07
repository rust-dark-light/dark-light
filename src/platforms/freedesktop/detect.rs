pub async fn detect() -> Result<crate::Mode, crate::Error> {
    super::get_color_scheme().await
}
