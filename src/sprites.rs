use crate::*;

pub type Glyph = u8;

pub struct GlyphIterator {
    glyphs: Glyphs,
    cursor: u8,
}

impl GlyphIterator {
    pub fn new(glyphs: Glyphs) -> Self {
        Self { glyphs, cursor: 0 }
    }
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

#[derive(Clone, Copy)]
pub enum Glyphs {
    Alphabet(&'static [Glyph]),
    Sequential(u8),
}

#[derive(Clone, Copy)]
pub struct Sprite {
    glyphs: Glyphs,
    size: Size,
    bitmap: &'static [u8],
}

impl Sprite {
    pub const fn new(glyphs: Glyphs, size: Size, bitmap: &'static [u8]) -> Self {
        Self {
            bitmap,
            glyphs,
            size,
        }
    }
}

impl Sprite {
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn glyphs(&self) -> GlyphIterator {
        GlyphIterator::new(self.glyphs)
    }

    pub fn render<C: Canvas>(&self, glyph: Glyph, origin: Point, canvas: &mut C) {
        let glyph_index = match self.glyphs {
            Glyphs::Sequential(len) if glyph < len => glyph as usize,
            Glyphs::Alphabet(glyphs) => match glyphs.iter().position(|g| *g == glyph) {
                None => return,
                Some(idx) => idx,
            },
            _ => return,
        };

        let size = (self.size.width as u32 * self.size.height as u32) >> 3;
        let size = size as usize;
        let offset = glyph_index * size;
        canvas.draw(
            Rectangle::new(origin, self.size),
            &self.bitmap[offset..(offset + size)],
        );
    }
}
