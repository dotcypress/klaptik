use crate::*;

#[derive(Clone, Copy)]
pub struct Tile<S> {
    state: S,
    sprite_id: SpriteId,
    origin: Point,
    columns: usize,
    rows: usize,
    sprite_size: Size,
    invalidate: bool,
}

impl<S> Tile<S>
where
    S: Copy + PartialEq + Into<Glyph>,
{
    pub fn new<SI: Into<SpriteId>>(
        sprite_id: SI,
        state: S,
        origin: Point,
        sprite_size: Size,
        columns: usize,
        rows: usize,
    ) -> Self {
        Self {
            origin,
            state,
            columns,
            rows,
            sprite_size,
            sprite_id: sprite_id.into(),
            invalidate: true,
        }
    }
}

impl<S> Widget<S> for Tile<S>
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
            let glyph = self.state.into();
            for x in 0..self.columns {
                for y in 0..self.rows {
                    let origin = Point::new(
                        self.origin.x + self.sprite_size.width * x as u8,
                        self.origin.y + self.sprite_size.height * y as u8,
                    );
                    display.render(RenderRequest::new(origin, self.sprite_id, glyph));
                }
            }
            self.invalidate = false;
        }
    }
}
