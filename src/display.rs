use crate::*;

pub struct SpriteDisplay<C: Canvas, const N: usize> {
    canvas: C,
    sprites: [FlashSprite; N],
}

impl<C: Canvas, const N: usize> SpriteDisplay<C, N> {
    pub const fn new(canvas: C, sprites: [FlashSprite; N]) -> Self {
        Self { canvas, sprites }
    }

    pub fn canvas(&mut self) -> &mut C {
        &mut self.canvas
    }
}

impl<C: Canvas, const N: usize> Display for SpriteDisplay<C, N> {
    fn render(&mut self, req: RenderRequest) {
        if let Some(sprite) = self
            .sprites
            .iter()
            .find(|sprite| sprite.id() == req.sprite_id)
        {
            if let Some(bitmap) = sprite
                .glyphs()
                .index(req.glyph)
                .and_then(|idx| sprite.bitmap(idx))
            {
                let bounds = Rectangle::new(req.origin, sprite.size());
                self.canvas.draw(bounds, bitmap);
            }
        }
    }
}

pub struct MirrorDisplay<A: Display, B: Display> {
    pub a: A,
    pub b: B,
}

impl<A: Display, B: Display> MirrorDisplay<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A: Display, B: Display> Display for MirrorDisplay<A, B> {
    fn render(&mut self, req: RenderRequest) {
        self.a.render(req);
        self.b.render(req);
    }
}
