pub fn detect() -> crate::Mode {
    futures_lite::future::block_on(super::get_color_scheme())
}
