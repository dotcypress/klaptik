use crate::*;

pub type Glyph = u8;

pub trait Sprite {
    fn glyphs(&self) -> &[Glyph];
    fn render<C: Canvas>(&self, glyph: Glyph, origin: Point, canvas: &mut C);
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
    fn glyphs(&self) -> &[Glyph] {
        self.glyphs.as_bytes()
    }

    fn render<C: Canvas>(&self, glyph: Glyph, origin: Point, canvas: &mut C) {
        let glyph_bytes = self.glyph_size.width() * self.glyph_size.height() >> 3;
        if let Some(glyph_index) = self.glyphs.find(glyph as char) {
            let offset = glyph_index * glyph_bytes as usize;
            canvas.draw(
                Rect(origin, self.glyph_size),
                &self.bitmap[offset..(offset + glyph_bytes as usize)],
            );
        }
    }
}
