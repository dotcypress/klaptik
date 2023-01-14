use crate::*;

#[derive(Clone, Copy)]
pub struct Icon<S> {
    state: S,
    sprite_id: SpriteId,
    origin: Point,
    invalidate: bool,
}

impl<S> Icon<S>
where
    S: Copy + PartialEq + Into<Glyph>,
{
    pub fn new<SI: Into<SpriteId>>(sprite_id: SI, state: S, origin: Point) -> Self {
        Self {
            origin,
            state,
            sprite_id: sprite_id.into(),
            invalidate: true,
        }
    }
}

impl<S> Widget<S> for Icon<S>
where
    S: Copy + PartialEq + Into<Glyph>,
{
    fn invalidate(&mut self) {
        self.invalidate = true;
    }

    fn update(&mut self, state: S) {
        if self.state != state {
            self.state = state;
            self.invalidate = true;
        }
    }

    fn render<D: Display>(&mut self, display: &mut D) {
        if self.invalidate {
            display.render(RenderRequest::new(
                self.origin,
                self.sprite_id,
                self.state.into(),
            ));
            self.invalidate = false;
        }
    }
}
