pub fn detect() -> Result<crate::Mode, crate::Error> {
    futures_lite::future::block_on(super::get_color_scheme())
}
