use crate::*;

pub type Glyph = u8;

pub struct GlyphIterator {
    glyphs: Glyphs,
    cursor: u8,
}

impl Iterator for GlyphIterator {
    type Item = Glyph;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.len() as u8 {
            return None;
        }

        let glyph = match self.glyphs {
            Glyphs::Sequential(_) => self.cursor,
            Glyphs::Alphabet(glyphs) => glyphs[self.cursor as usize],
        };
        self.cursor += 1;

        Some(glyph)
    }
}

impl ExactSizeIterator for GlyphIterator {
    fn len(&self) -> usize {
        match self.glyphs {
            Glyphs::Sequential(len) => len as usize,
            Glyphs::Alphabet(glyphs) => glyphs.len(),
        }
    }
}

pub trait Sprite {
    fn glyphs(&self) -> GlyphIterator;
    fn sprite_size(&self) -> Size;
    fn render<C: Canvas>(&self, glyph: Glyph, origin: Point, canvas: &mut C);
}

#[derive(Clone, Copy)]
pub enum Glyphs {
    Alphabet(&'static [Glyph]),
    Sequential(u8),
}

#[derive(Clone, Copy)]
pub struct RomSprite {
    glyphs: Glyphs,
    sprite_size: Size,
    bitmap: &'static [u8],
}

impl RomSprite {
    pub const fn new(glyphs: Glyphs, sprite_size: Size, bitmap: &'static [u8]) -> Self {
        Self {
            bitmap,
            glyphs,
            sprite_size,
        }
    }
}

impl Sprite for RomSprite {
    fn sprite_size(&self) -> Size {
        self.sprite_size
    }

    fn glyphs(&self) -> GlyphIterator {
        GlyphIterator {
            glyphs: self.glyphs,
            cursor: 0,
        }
    }

    fn render<C: Canvas>(&self, glyph: Glyph, origin: Point, canvas: &mut C) {
        let glyph_index = match self.glyphs {
            Glyphs::Sequential(len) if glyph < len => glyph as usize,
            Glyphs::Alphabet(glyphs) => match glyphs.iter().position(|g| *g == glyph) {
                None => return,
                Some(idx) => idx,
            },
            _ => return,
        };

        let size = self.sprite_size.width * self.sprite_size.height >> 3;
        let size = size as usize;
        let offset = glyph_index * size;
        canvas.draw(
            Rectangle::new(origin, self.sprite_size),
            &self.bitmap[offset..(offset + size)],
        );
    }
}
