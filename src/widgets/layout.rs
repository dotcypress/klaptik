use crate::*;

pub const DIR_LTR: u32 = 0;
pub const DIR_RTL: u32 = 1;
pub const DIR_DOWN: u32 = 2;
pub const DIR_UP: u32 = 3;

pub trait Layout {
    fn layout(node_idx: usize, origin: Point, glyph_size: Size) -> Point;
}

pub struct GridLayout<const DIR: u32, const WRAP: u32>;

impl<const DIR: u32, const WRAP: u32> Layout for GridLayout<DIR, WRAP> {
    fn layout(node_idx: usize, origin: Point, size: Size) -> Point {
        let idx = (node_idx as u32 % WRAP) as i32;
        let wraps = (node_idx as u32 / WRAP) as i32;
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
