use crate::*;

pub enum Glyphs {
    Single,
    Sequential(u8),
    Alphabet(&'static [Glyph]),
}

#[allow(clippy::len_without_is_empty)]
impl Glyphs {
    pub fn index(&self, glyph: Glyph) -> Option<usize> {
        match self {
            Glyphs::Single => Some(0),
            Glyphs::Sequential(len) if glyph < *len => Some(glyph as _),
            Glyphs::Alphabet(glyphs) => glyphs.iter().position(|g| *g == glyph),
            _ => None,
        }
    }

    pub const fn len(&self) -> usize {
        match self {
            Glyphs::Single => 1,
            Glyphs::Sequential(len) => *len as usize,
            Glyphs::Alphabet(glyphs) => glyphs.len(),
        }
    }
}

pub struct FlashSprite {
    id: SpriteId,
    glyphs: Glyphs,
    size: Size,
    glyph_len: usize,
    bitmap: &'static [u8],
}

impl FlashSprite {
    pub const fn new(id: SpriteId, glyphs: Glyphs, size: Size, bitmap: &'static [u8]) -> Self {
        let glyph_len = bitmap.len() / glyphs.len();
        Self {
            id,
            glyphs,
            size,
            bitmap,
            glyph_len,
        }
    }
}

impl FlashSprite {
    pub fn id(&self) -> SpriteId {
        self.id
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn glyphs(&self) -> &Glyphs {
        &self.glyphs
    }

    pub fn bitmap(&self) -> &[u8] {
        self.bitmap
    }

    pub fn glyph_bitmap(&self, glyph_index: usize) -> Option<&[u8]> {
        if glyph_index >= self.glyphs.len() {
            return None;
        }
        let offset = glyph_index * self.glyph_len;
        Some(&self.bitmap[offset..][..self.glyph_len])
    }
}
