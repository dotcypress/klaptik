use crate::*;

pub const DIR_LTR: usize = 0;
pub const DIR_RTL: usize = 1;
pub const DIR_DOWN: usize = 2;
pub const DIR_UP: usize = 3;

pub trait Layout {
    fn layout(node_idx: usize, origin: Point, glyph_size: Size) -> Point;
}

pub struct GridLayout<const DIR: usize, const WRAP: u16>;

impl<const DIR: usize, const WRAP: u16> Layout for GridLayout<DIR, WRAP> {
    fn layout(node_idx: usize, origin: Point, size: Size) -> Point {
        let idx = node_idx as u16 % WRAP;
        let wraps = node_idx as u16 / WRAP;
        match DIR {
            DIR_UP => Point(
                origin.x() + size.width() * wraps,
                origin.y() - size.height() * (idx + 1),
            ),
            DIR_DOWN => Point(
                origin.x() + size.width() * wraps,
                origin.y() + size.height() * idx,
            ),
            DIR_RTL => Point(
                origin.x() - size.width() * (idx + 1),
                origin.y() + size.height() * wraps,
            ),
            _ => Point(
                origin.x() + size.width() * idx,
                origin.y() + size.height() * wraps,
            ),
        }
    }
}
