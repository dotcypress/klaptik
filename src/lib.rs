#![no_std]
// #![deny(warnings)]

pub mod drivers;

mod macros;
mod prelude;
mod sprites;
mod widgets;

pub use macros::*;
pub use prelude::*;
pub use sprites::*;
pub use widgets::*;

pub trait Canvas {
    fn draw(&mut self, bounds: Rect, buf: &[u8]);
}
