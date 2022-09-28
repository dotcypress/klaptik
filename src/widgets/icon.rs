use crate::*;

pub type SpriteIcon = Icon<RomSprite>;

#[derive(Clone, Copy)]
pub struct Icon<S> {
    sprite: S,
    state: Glyph,
    origin: Point,
    render_req: bool,
}

impl<S: Sprite + Copy> Icon<S> {
    pub fn new<P: Into<Point>>(sprite: S, state: Glyph, origin: P) -> Self {
        Self {
            sprite,
            state,
            origin: origin.into(),
            render_req: true,
        }
    }
}

impl<S: Sprite> Widget<Glyph> for Icon<S> {
    fn invalidate(&mut self) {
        self.render_req = true;
    }

    fn update(&mut self, state: Glyph) {
        if self.state != state {
            self.state = state;
            self.render_req = true;
        }
    }

    fn render<C: Canvas>(&mut self, canvas: &mut C) {
        if self.render_req {
            self.sprite.render(self.state, self.origin, canvas);
            self.render_req = false;
        }
    }
}
