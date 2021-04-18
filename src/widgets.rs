use crate::*;

pub trait Canvas {
  fn draw(&mut self, bounds: &Rect, buffer: &[u8]);
}

pub trait Widget<S> {
  fn set_state(&mut self, state: S);
  fn render<C: Canvas>(&mut self, canvas: &mut C);
  fn invalidate(&mut self);
}

pub struct ClearWidget {
  bpp: u16,
  bounds: Rect,
  render_req: bool,
}

impl ClearWidget {
  pub fn new<P: Into<Point>, S: Into<Size>>(origin: P, size: S, bpp: u16) -> Self {
    Self {
      bounds: Rect(origin.into(), size.into()),
      render_req: true,
      bpp,
    }
  }
}

impl Widget<()> for ClearWidget {
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
          canvas.draw(&Rect(origin, tile), &[0; 32][..chunk_size as usize])
        }
      }
    }
  }
}

#[derive(Clone, Copy)]
pub struct SpriteWidget<S> {
  sprite: S,
  state: Glyph,
  origin: Point,
  render_req: bool,
}

impl<S: Sprite + Copy> SpriteWidget<S> {
  pub fn new<P: Into<Point>>(origin: P, sprite: S, state: Glyph) -> Self {
    Self {
      sprite,
      state,
      origin: origin.into(),
      render_req: true,
    }
  }
}

impl<S: Sprite> Widget<Glyph> for SpriteWidget<S> {
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
        canvas.draw(&Rect(self.origin, self.sprite.size()), sprite);
      }
      self.render_req = false;
    }
  }
}

pub struct ChainWidget<S: Sprite, const SIZE: usize> {
  sprite: S,
  origin: Point,
  dir: Direction,
  state: [u8; SIZE],
  render_req: [bool; SIZE],
  cursor: u8,
}

impl<S: Sprite + Copy, const SIZE: usize> ChainWidget<S, SIZE> {
  pub fn new<P: Into<Point>>(origin: P, sprite: S, val: &str, dir: Direction) -> Self {
    let state = val.as_bytes();
    assert!(state.len() <= SIZE);
    let glyph = sprite.glyph(0);
    let mut state: [u8; SIZE] = [glyph; SIZE];
    let mut render_req: [bool; SIZE] = [false; SIZE];

    for (idx, sym) in val.bytes().enumerate() {
      state[idx] = sym;
      render_req[idx] = true;
    }

    Self {
      dir,
      sprite,
      state,
      render_req,
      cursor: 0,
      origin: origin.into(),
    }
  }
}

impl<S: Sprite, const SIZE: usize> Widget<&[u8; SIZE]> for ChainWidget<S, SIZE> {
  fn set_state(&mut self, state: &[u8; SIZE]) {
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
    let mut x = self.origin.x();
    let mut y = self.origin.y();
    for (idx, render_req) in self.render_req.iter_mut().enumerate() {
      if *render_req {
        let glyph = self.state[idx];
        if let Some(buf) = self.sprite.render(glyph) {
          canvas.draw(&Rect(Point(x, y), glyph_size), buf);
        }
        *render_req = false;
      }
      match self.dir {
        Direction::Left2Right => {
          x += glyph_size.width();
        }
        Direction::Right2Left => {
          x -= glyph_size.width();
        }
        Direction::Top2Bottom => {
          y += glyph_size.height();
        }
        Direction::Bottom2Top => {
          y -= glyph_size.height();
        }
      }
    }
  }
}

impl<S: Sprite, const SIZE: usize> core::fmt::Write for ChainWidget<S, SIZE> {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    let mut cursor = self.cursor as usize;
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
    self.cursor = cursor as u8;
    Ok(())
  }
}

pub struct LabelWidget<S: Sprite, const SIZE: usize>(ChainWidget<S, SIZE>);

impl<S: Sprite + Copy, const SIZE: usize> LabelWidget<S, SIZE> {
  pub fn new<P: Into<Point>>(origin: P, font: S, val: &str) -> Self {
    Self(ChainWidget::new(origin, font, val, Direction::Left2Right))
  }
}

impl<S: Sprite, const SIZE: usize> Widget<&[u8; SIZE]> for LabelWidget<S, SIZE> {
  fn set_state(&mut self, state: &[u8; SIZE]) {
    self.0.set_state(state);
  }

  fn invalidate(&mut self) {
    self.0.invalidate();
  }

  fn render<C: Canvas>(&mut self, canvas: &mut C) {
    self.0.render(canvas);
  }
}

impl<S: Sprite, const SIZE: usize> core::fmt::Write for LabelWidget<S, SIZE> {
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    self.0.write_str(s)
  }
}
