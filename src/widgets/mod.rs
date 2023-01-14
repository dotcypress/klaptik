use crate::*;

mod grid;
mod icon;
mod macros;
mod tile;

pub use grid::*;
pub use icon::*;
pub use layout::*;
pub use macros::*;
pub use tile::*;

pub trait Widget<S> {
    fn invalidate(&mut self);
    fn update(&mut self, state: S);
    fn render<D: Display>(&mut self, render: &mut D);
}

pub type GlyphIcon = Icon<Glyph>;
pub type Label<const L: usize> = Grid<WrapLayout<DirRight, { usize::MAX }>, L>;
pub type VerticalLabel<const L: usize> = Grid<WrapLayout<DirDown, { usize::MAX }>, L>;
pub type WrapPanel<const L: usize, const W: usize> = Grid<WrapLayout<DirRight, W>, L>;
