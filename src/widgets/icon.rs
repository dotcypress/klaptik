use crate::*;

pub type RomIcon<G> = Icon<RomSprite, G>;

#[derive(Clone, Copy)]
pub struct Icon<S, G> {
    sprite: S,
    state: G,
    origin: Point,
    render_req: bool,
}

impl<S: Sprite + Copy, G: Copy + PartialEq + Into<Glyph>> Icon<S, G> {
    pub fn new(sprite: S, state: G, origin: Point) -> Self {
        Self {
            sprite,
            origin,
            state,
            render_req: true,
        }
    }
}

impl<S: Sprite + Copy, G: Copy + PartialEq + Into<Glyph>> Widget<G> for Icon<S, G> {
    fn invalidate(&mut self) {
        self.render_req = true;
    }

    fn update(&mut self, state: G) {
        if self.state != state {
            self.state = state;
            self.render_req = true;
        }
    }

    fn render<C: Canvas>(&mut self, canvas: &mut C) {
        if self.render_req {
            self.sprite.render(self.state.into(), self.origin, canvas);
            self.render_req = false;
        }
    }
}
