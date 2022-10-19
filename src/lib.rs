#![no_std]
#![deny(warnings)]

pub mod drivers;

mod sprites;
mod widgets;

pub use sprites::*;
pub use widgets::*;

pub trait Canvas {
    fn draw(&mut self, bounds: Rectangle, buf: &[u8]);
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
