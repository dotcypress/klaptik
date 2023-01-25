use crate::*;

pub enum Glyphs {
    Single,
    Sequential(u8),
    Alphabet(&'static [Glyph]),
}

pub struct FlashSprite {
    id: SpriteId,
    glyphs: Glyphs,
    size: Size,
    bitmap: &'static [u8],
}

impl FlashSprite {
    pub const fn new(id: SpriteId, glyphs: Glyphs, size: Size, bitmap: &'static [u8]) -> Self {
        Self {
            id,
            glyphs,
            size,
            bitmap,
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

    pub fn bitmap(&self, glyph: Glyph) -> Option<&[u8]> {
        let glyph_idx = match self.glyphs {
            Glyphs::Single => 0,
            Glyphs::Sequential(len) if glyph < len => glyph,
            Glyphs::Alphabet(glyphs) => match glyphs.iter().position(|g| *g == glyph) {
                None => return None,
                Some(idx) => idx as u8,
            },
            _ => return None,
        };

        let size = (self.size.width as u32 * self.size.height as u32) >> 3;
        let size = size as usize;
        let offset = glyph_idx as usize * size;
        Some(&self.bitmap[offset..(offset + size)])
    }
}
