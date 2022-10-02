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
        let idx = (node_idx % WRAP) as i32;
        let wraps = (node_idx / WRAP) as i32;
        let width = size.width as i32;
        let height = size.height as i32;
        let offset = match DIR {
            DIR_UP => Point::new(width * wraps, -height * (idx + 1)),
            DIR_DOWN => Point::new(width * wraps, height * idx),
            DIR_RTL => Point::new(-width * (idx + 1), height * wraps),
            _ => Point::new(width * idx, height * wraps),
        };
        origin + offset
    }
}
