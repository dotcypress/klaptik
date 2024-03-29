#![no_std]
#![deny(warnings)]

pub mod drivers;
pub mod layout;

mod display;
mod sprites;
mod widgets;

pub use display::*;
pub use sprites::*;
pub use widgets::*;

pub type Glyph = u8;
pub type SpriteId = u8;

pub trait Canvas {
    fn draw(&mut self, bounds: Rectangle, bitmap: &[u8]);
}

pub trait Display {
    fn render(&mut self, req: RenderRequest);
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderRequest {
    pub origin: Point,
    pub sprite_id: SpriteId,
    pub glyph: Glyph,
}

impl RenderRequest {
    pub fn new(origin: Point, sprite_id: SpriteId, glyph: Glyph) -> Self {
        Self {
            origin,
            sprite_id,
            glyph,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() == 4);
        Self::new(Point::new(bytes[0], bytes[1]), bytes[2], bytes[3])
    }

    pub fn as_bytes(&self) -> [u8; 4] {
        [self.origin.x, self.origin.y, self.sprite_id, self.glyph]
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<Point> for (u8, u8) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u8,
    pub height: u8,
}

impl Size {
    pub const fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub origin: Point,
    pub size: Size,
}

impl Rectangle {
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn start(&self) -> Point {
        self.origin
    }

    pub fn end(&self) -> Point {
        let origin = self.origin;
        let size = self.size;
        Point::new(origin.x + size.width, origin.y + size.height)
    }
}

pub enum Glyphs {
    Single,
    Sequential(u8),
    Alphabet(&'static [Glyph]),
}

#[allow(clippy::len_without_is_empty)]
impl Glyphs {
    pub fn index(&self, glyph: Glyph) -> Option<usize> {
        match self {
            Glyphs::Single => Some(0),
            Glyphs::Sequential(len) if glyph < *len => Some(glyph as _),
            Glyphs::Alphabet(glyphs) => glyphs.iter().position(|g| *g == glyph),
            _ => None,
        }
    }

    pub const fn len(&self) -> usize {
        match self {
            Glyphs::Single => 1,
            Glyphs::Sequential(len) => *len as usize,
            Glyphs::Alphabet(glyphs) => glyphs.len(),
        }
    }
}
