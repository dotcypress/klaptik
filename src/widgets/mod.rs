use crate::*;

mod drawing;
mod fill;
mod grid;
mod icon;
mod layout;

pub use drawing::*;
pub use fill::*;
pub use grid::*;
pub use icon::*;
pub use layout::*;

pub trait Widget<S> {
    fn invalidate(&mut self);
    fn update(&mut self, state: S);
    fn render<C: Canvas>(&mut self, canvas: &mut C);
}

pub type Label<S, const SIZE: usize, const W: usize, const H: usize> =
    Grid<S, GridLayout<DIR_LTR, { u32::MAX }>, SIZE, W, H>;
pub type VerticalLabel<S, const SIZE: usize, const W: usize, const H: usize> =
    Grid<S, GridLayout<DIR_DOWN, { u32::MAX }>, SIZE, W, H>;
pub type TextBox<S, const SIZE: usize, const W: usize, const H: usize, const WRAP: u32> =
    Grid<S, GridLayout<DIR_LTR, WRAP>, SIZE, W, H>;
