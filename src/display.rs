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
            if let Some(bitmap) = sprite.bitmap(req.glyph) {
                let bounds = Rectangle::new(req.origin, sprite.size());
                self.canvas.draw(bounds, bitmap);
            }
        }
    }
}
