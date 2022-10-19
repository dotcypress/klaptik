use crate::*;

#[derive(Clone, Copy)]
pub struct Icon<S> {
    state: S,
    sprite: Sprite,
    origin: Point,
    render_req: bool,
}

impl<S> Icon<S>
where
    S: Copy + PartialEq + Into<Glyph>,
{
    pub fn new(sprite: Sprite, state: S, origin: Point) -> Self {
        Self {
            sprite,
            origin,
            state,
            render_req: true,
        }
    }
}

impl<S> Widget<S> for Icon<S>
where
    S: Copy + PartialEq + Into<Glyph>,
{
    fn invalidate(&mut self) {
        self.render_req = true;
    }

    fn update(&mut self, state: S) {
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
