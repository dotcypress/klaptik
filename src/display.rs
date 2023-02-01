use crate::*;

pub struct SpriteDisplay<C, const N: usize> {
    canvas: C,
    sprite_map: [(FlashSprite, Glyphs); N],
}

impl<C, const N: usize> SpriteDisplay<C, N> {
    pub const fn new(canvas: C, sprite_map: [(FlashSprite, Glyphs); N]) -> Self {
        Self { canvas, sprite_map }
    }

    pub fn canvas(&mut self) -> &mut C {
        &mut self.canvas
    }
}

impl<C: Canvas, const N: usize> Display for SpriteDisplay<C, N> {
    fn render(&mut self, req: RenderRequest) {
        if let Some((sprite, glyphs)) = self
            .sprite_map
            .iter()
            .find(|(sprite, _)| sprite.id() == req.sprite_id)
        {
            if let Some(bitmap) = glyphs
                .index(req.glyph)
                .and_then(|idx| sprite.glyph_bitmap(idx))
            {
                let bounds = Rectangle::new(req.origin, sprite.size());
                self.canvas.draw(bounds, bitmap);
            }
        }
    }
}

impl<A: Display, B: Display> Display for (A, B) {
    fn render(&mut self, req: RenderRequest) {
        self.0.render(req);
        self.1.render(req);
    }
}

impl<A: Display, B: Display, C: Display> Display for (A, B, C) {
    fn render(&mut self, req: RenderRequest) {
        self.0.render(req);
        self.1.render(req);
        self.2.render(req);
    }
}
