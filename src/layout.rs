use crate::*;
use core::marker::PhantomData;

pub trait Layout {
    fn layout(node_idx: usize, origin: Point, sprite_size: Size) -> Point;
}

pub struct WrapLayout<DIR, const WRAP: usize> {
    _dir: PhantomData<DIR>,
}

pub struct DirLeft;
pub struct DirRight;
pub struct DirDown;
pub struct DirUp;

struct WrapInfo {
    idx: u8,
    wraps: u8,
    width: u8,
    height: u8,
}

impl<DIR, const WRAP: usize> WrapLayout<DIR, WRAP> {
    fn wrap(node_idx: usize, sprite_size: Size) -> WrapInfo {
        let idx = (node_idx % WRAP) as u8;
        let wraps = (node_idx / WRAP) as u8;
        let width = sprite_size.width;
        let height = sprite_size.height;
        WrapInfo {
            idx,
            wraps,
            width,
            height,
        }
    }
}

impl<const WRAP: usize> Layout for WrapLayout<DirLeft, WRAP> {
    fn layout(node_idx: usize, origin: Point, sprite_size: Size) -> Point {
        let WrapInfo {
            idx,
            wraps,
            width,
            height,
        } = Self::wrap(node_idx, sprite_size);
        Point::new(origin.x - width * (idx + 1), origin.y + height * wraps)
    }
}

impl<const WRAP: usize> Layout for WrapLayout<DirRight, WRAP> {
    fn layout(node_idx: usize, origin: Point, sprite_size: Size) -> Point {
        let WrapInfo {
            idx,
            wraps,
            width,
            height,
        } = Self::wrap(node_idx, sprite_size);
        Point::new(origin.x + width * idx, origin.y + height * wraps)
    }
}

impl<const WRAP: usize> Layout for WrapLayout<DirUp, WRAP> {
    fn layout(node_idx: usize, origin: Point, sprite_size: Size) -> Point {
        let WrapInfo {
            idx,
            wraps,
            width,
            height,
        } = Self::wrap(node_idx, sprite_size);
        Point::new(origin.x + width * wraps, origin.y - height * (idx + 1))
    }
}

impl<const WRAP: usize> Layout for WrapLayout<DirDown, WRAP> {
    fn layout(node_idx: usize, origin: Point, sprite_size: Size) -> Point {
        let WrapInfo {
            idx,
            wraps,
            width,
            height,
        } = Self::wrap(node_idx, sprite_size);
        Point::new(origin.x + width * wraps, origin.y + height * idx)
    }
}
