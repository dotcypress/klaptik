use crate::*;

pub const DIR_LTR: usize = 0;
pub const DIR_RTL: usize = 1;
pub const DIR_DOWN: usize = 2;
pub const DIR_UP: usize = 3;

pub trait Layout {
    fn layout(node_idx: usize, origin: Point, glyph_size: Size) -> Point;
}

pub struct GridLayout<const DIR: usize, const WRAP: usize>;

impl<const DIR: usize, const WRAP: usize> Layout for GridLayout<DIR, WRAP> {
    fn layout(node_idx: usize, origin: Point, size: Size) -> Point {
        let idx = (node_idx % WRAP) as u8;
        let wraps = (node_idx / WRAP) as u8;
        let width = size.width;
        let height = size.height;
        match DIR {
            DIR_UP => Point::new(origin.x + width * wraps, origin.y - height * (idx + 1)),
            DIR_DOWN => Point::new(origin.x + width * wraps, origin.y + height * idx),
            DIR_RTL => Point::new(origin.x - width * (idx + 1), origin.y + height * wraps),
            _ => Point::new(origin.x + width * idx, origin.y + height * wraps),
        }
    }
}
