use crate::{Error, Mode};

pub fn detect() -> Result<Mode, Error> {
    futures_lite::future::block_on(super::get_color_scheme())
}
