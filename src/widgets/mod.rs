use crate::*;

mod background;
mod grid;
mod icon;
mod layout;

pub use background::*;
pub use grid::*;
pub use icon::*;
pub use layout::*;

pub trait Widget<S> {
    fn invalidate(&mut self);
    fn update(&mut self, state: S);
    fn render<C: Canvas>(&mut self, canvas: &mut C);
}

pub type Label<S, const SIZE: usize, const SW: usize, const SH: usize> =
    Grid<S, GridLayout<DIR_LTR, { u16::MAX }>, SIZE, SW, SH>;
pub type VerticalLabel<S, const SIZE: usize, const SW: usize, const SH: usize> =
    Grid<S, GridLayout<DIR_DOWN, { u16::MAX }>, SIZE, SW, SH>;
pub type TextBox<S, const SIZE: usize, const SW: usize, const SH: usize, const WRAP: u16> =
    Grid<S, GridLayout<DIR_LTR, WRAP>, SIZE, SW, SH>;
