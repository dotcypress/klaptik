use crate::*;
use core::marker::PhantomData;

pub trait Canvas {
  fn draw(&mut self, bounds: Rect, buffer: &[u8]);
}

pub trait Widget<S> {
  fn set_state(&mut self, state: S);
  fn render<C: Canvas>(&mut self, canvas: &mut C);
  fn invalidate(&mut self);
}

pub struct Background {
  bpp: u16,
  bounds: Rect,
  render_req: bool,
}

impl Background {
  pub fn new<P: Into<Point>, S: Into<Size>>(origin: P, size: S, bpp: u16) -> Self {
    Self {
      bounds: Rect(origin.into(), size.into()),
      render_req: true,
      bpp,
    }
  }
}

impl Widget<()> for Background {
  fn set_state(&mut self, _: ()) {}

  fn invalidate(&mut self) {
    self.render_req = true;
  }

  fn render<C: Canvas>(&mut self, canvas: &mut C) {
    if !self.render_req {
      return;
    }
    self.render_req = false;

    let bounds = self.bounds;
    for x in (bounds.origin().x()..bounds.size().width()).step_by(16) {
      for y in (bounds.origin().y()..bounds.size().height()).step_by(16) {
        let origin = Point(x, y);
        let tile = Size(
          u16::min(16, bounds.size().width() - x),
          u16::min(16, bounds.size().height() - y),
        );

        let mut tile_len = tile.width() * tile.height() * self.bpp / 8;
        while tile_len > 0 {
          let chunk_size = u16::min(32, tile_len);
          tile_len -= chunk_size;
          canvas.draw(Rect(origin, tile), &[0; 32][..chunk_size as usize])
        }
      }
    }
  }
}

#[derive(Clone, Copy)]
pub struct Icon<S> {
  sprite: S,
  state: Glyph,
  origin: Point,
  render_req: bool,
}

impl<S: Sprite + Copy> Icon<S> {
  pub fn new<P: Into<Point>>(origin: P, sprite: S, state: Glyph) -> Self {
    Self {
      sprite,
      state,
      origin: origin.into(),
      render_req: true,
    }
  }
}

impl<S: Sprite> Widget<Glyph> for Icon<S> {
  fn invalidate(&mut self) {
    self.render_req = true;
  }

  fn set_state(&mut self, state: Glyph) {
    if self.state != state {
      self.state = state;
      self.render_req = true;
    }
  }

  fn render<C: Canvas>(&mut self, canvas: &mut C) {
    if self.render_req {
      if let Some(sprite) = self.sprite.render(self.state) {
        canvas.draw(Rect(self.origin, self.sprite.size()), sprite);
      }
      self.render_req = false;
    }
  }
}

pub trait Layout {
  fn layout(node_idx: usize, origin: Point, glyph_size: Size) -> Rect;
}

pub struct Grid<S: Sprite, L: Layout, const SIZE: usize> {
  layout: PhantomData<L>,
  sprite: S,
  origin: Point,
  state: [Glyph; SIZE],
  render_req: [bool; SIZE],
  cursor: usize,
}

impl<S: Sprite + Copy, L: Layout, const SIZE: usize> Grid<S, L, SIZE> {
  pub fn new<P: Into<Point>>(origin: P, sprite: S, val: &str) -> Self {
    let state = val.as_bytes();
    assert!(state.len() <= SIZE);
    let glyph = sprite.glyph(0);
    let mut state: [Glyph; SIZE] = [glyph; SIZE];
    let mut render_req: [bool; SIZE] = [false; SIZE];

    for (idx, sym) in val.bytes().enumerate() {
      state[idx] = sym;
      render_req[idx] = true;
    }

    Self {
      sprite,
      state,
      render_req,
      cursor: 0,
      origin: origin.into(),
      layout: PhantomData {},
    }
  }
}

impl<S: Sprite, L: Layout, const SIZE: usize> Grid<S, L, SIZE> {
  pub fn set_glyph(&mut self, idx: usize, glyph: Glyph) {
    if self.state[idx] != glyph {
      self.state[idx] = glyph;
      self.render_req[idx] = true;
    }
  }
}

impl<S: Sprite, L: Layout, const SIZE: usize> Widget<&[Glyph; SIZE]> for Grid<S, L, SIZE> {
  fn set_state(&mut self, state: &[Glyph; SIZE]) {
    for (idx, sym) in state.iter().enumerate() {
      if self.state[idx] != *sym {
        self.state[idx] = *sym;
        self.render_req[idx] = true;
      }
    }
    self.cursor = 0;
  }

  fn invalidate(&mut self) {
    for render_req in self.render_req.iter_mut() {
      *render_req = true;
    }
  }

  fn render<C: Canvas>(&mut self, canvas: &mut C) {
    let glyph_size = self.sprite.size();
    for (idx, render_req) in self.render_req.iter_mut().enumerate() {
      if *render_req {
        let glyph = self.state[idx];
        if let Some(buf) = self.sprite.render(glyph) {
          let bounds = L::layout(idx, self.origin, glyph_size);
          canvas.draw(bounds, buf);
        }
        *render_req = false;
      }
    }
  }
}

impl<S: Sprite, L: Layout, const SIZE: usize> core::fmt::Write for Grid<S, L, SIZE> {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    let mut cursor = self.cursor;
    for byte in s.as_bytes() {
      if self.state[cursor] != *byte {
        self.state[cursor] = *byte;
        self.render_req[cursor] = true;
      }
      cursor += 1;
      if cursor >= SIZE {
        cursor = 0;
      }
    }
    self.cursor = cursor;
    Ok(())
  }
}

#[derive(PartialEq, Eq)]
pub enum LayoutDirection {
  Horizontal,
  Vertical,
}

pub const NO_WRAP: u16 = u16::MAX;
pub const DIR_LTR: usize = 0;
pub const DIR_RTL: usize = 1;
pub const DIR_DOWN: usize = 2;
pub const DIR_UP: usize = 3;

pub struct GridLayout<const DIR: usize, const WRAP: u16>(LayoutDirection);

impl<const DIR: usize, const WRAP: u16> Layout for GridLayout<DIR, WRAP> {
  fn layout(node_idx: usize, origin: Point, glyph_size: Size) -> Rect {
    let idx = node_idx as u16 % WRAP;
    let wraps = node_idx as u16 / WRAP;
    let new_origin = match DIR {
      DIR_UP => Point(
        origin.x() + glyph_size.width() * wraps,
        origin.y() - glyph_size.height() * (idx + 1),
      ),
      DIR_DOWN => Point(
        origin.x() + glyph_size.width() * wraps,
        origin.y() + glyph_size.height() * idx,
      ),
      DIR_RTL => Point(
        origin.x() - glyph_size.width() * (idx + 1),
        origin.y() + glyph_size.height() * wraps,
      ),
      _ => Point(
        origin.x() + glyph_size.width() * idx,
        origin.y() + glyph_size.height() * wraps,
      ),
    };
    Rect(new_origin, glyph_size)
  }
}

pub type Label<S, const SIZE: usize> = Grid<S, GridLayout<DIR_LTR, NO_WRAP>, SIZE>;
pub type VerticalLabel<S, const SIZE: usize> = Grid<S, GridLayout<DIR_DOWN, NO_WRAP>, SIZE>;
pub type TextBox<S, const SIZE: usize, const WRAP: u16> = Grid<S, GridLayout<DIR_LTR, WRAP>, SIZE>;
