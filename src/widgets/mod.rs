use crate::*;

mod drawing;
mod fill;
mod grid;
mod icon;
mod layout;
mod macros;

pub use drawing::*;
pub use fill::*;
pub use grid::*;
pub use icon::*;
pub use layout::*;
pub use macros::*;

pub trait Widget<S> {
    fn invalidate(&mut self);
    fn update(&mut self, state: S);
    fn render<C: Canvas>(&mut self, canvas: &mut C);
}

pub type Label<S, const LEN: usize> = Grid<S, GridLayout<DIR_LTR, { usize::MAX }>, LEN>;
pub type VerticalLabel<S, const LEN: usize> = Grid<S, GridLayout<DIR_DOWN, { usize::MAX }>, LEN>;
pub type TextBox<S, const LEN: usize, const WRAP: usize> = Grid<S, GridLayout<DIR_LTR, WRAP>, LEN>;
