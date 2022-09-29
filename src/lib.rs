#![no_std]
// #![deny(warnings)]

pub mod drivers;

mod macros;
mod sprites;
mod widgets;

pub use embedded_graphics_core::geometry::*;
pub use embedded_graphics_core::primitives::Rectangle;
pub use macros::*;
pub use sprites::*;
pub use widgets::*;

pub trait Canvas {
    fn draw(&mut self, bounds: Rectangle, buf: &[u8]);
}
