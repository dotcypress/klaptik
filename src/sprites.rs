use crate::*;

pub type Glyph = u8;

pub trait Sprite {
  fn size(&self) -> Size;
  fn glyph(&self, idx: usize) -> Glyph;
  fn render(&self, glyph: Glyph) -> Option<&[u8]>;
}

#[derive(Clone, Copy)]
pub struct RomSprite {
  bitmap: &'static [u8],
  glyphs: &'static str,
  glyph_size: Size,
}

impl RomSprite {
  pub const fn new(glyphs: &'static str, glyph_size: Size, bitmap: &'static [u8]) -> Self {
    Self {
      bitmap,
      glyphs,
      glyph_size,
    }
  }
}

impl Sprite for RomSprite {
  fn size(&self) -> Size {
    self.glyph_size
  }

  fn glyph(&self, idx: usize) -> Glyph {
    self.glyphs.as_bytes()[idx]
  }

  fn render(&self, glyph: Glyph) -> Option<&'static [u8]> {
    let glyph_bytes = self.glyph_size.width() * self.glyph_size.height() / 8;
    let glyph_index = self.glyphs.find(glyph as char)?;
    let offset = glyph_index * glyph_bytes as usize;
    Some(&self.bitmap[offset..(offset + glyph_bytes as usize)])
  }
}
