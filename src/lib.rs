#![no_std]
#![deny(warnings)]

mod macros;
mod prelude;
mod sprites;
mod widgets;
mod spi;

pub use macros::*;
pub use prelude::*;
pub use sprites::*;
pub use widgets::*;
pub use spi::*;

pub trait Canvas {
    fn draw(&mut self, bounds: Rect, buf: &[u8]);
}
