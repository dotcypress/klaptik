#![no_std]
#![deny(warnings)]

pub mod drivers;

mod sprites;
mod widgets;

pub use sprites::*;
pub use widgets::*;

pub trait Canvas {
    fn draw(&mut self, bounds: Rectangle, buf: &[u8]);

    fn clear(&mut self, bounds: Rectangle) {
        let pattern = [0; 32];
        let origin = bounds.origin;
        let size = bounds.size;
        for x in (0..size.width).step_by(8) {
            for y in (0..size.height).step_by(8) {
                let offset = Point::new(x + origin.x, y + origin.y);
                let tile = Size::new(u8::min(8, size.width - x), u8::min(8, size.height - y));
                let mut tile_len = (tile.width as u32 * tile.height as u32) >> 3;
                while tile_len > 0 {
                    let chunk_size = u32::min(32, tile_len);
                    tile_len -= chunk_size;
                    self.draw(
                        Rectangle::new(offset, tile),
                        &pattern[..chunk_size as usize],
                    )
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }

    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u8,
    pub height: u8,
}

impl Size {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub origin: Point,
    pub size: Size,
}

impl Rectangle {
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
}
