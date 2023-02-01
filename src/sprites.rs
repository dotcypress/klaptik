use crate::*;

pub struct FlashSprite {
    id: SpriteId,
    size: Size,
    glyphs: usize,
    glyph_len: usize,
    bitmap: &'static [u8],
}

impl FlashSprite {
    pub const fn new(id: SpriteId, glyphs: usize, size: Size, bitmap: &'static [u8]) -> Self {
        let glyph_len = bitmap.len() / glyphs;
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

    pub fn glyphs(&self) -> usize {
        self.glyphs
    }

    pub fn raw(&self) -> &[u8] {
        self.bitmap
    }

    pub fn glyph_bitmap(&self, glyph_index: usize) -> Option<&[u8]> {
        if glyph_index >= self.glyphs {
            return None;
        }
        let offset = glyph_index * self.glyph_len;
        Some(&self.bitmap[offset..][..self.glyph_len])
    }
}
