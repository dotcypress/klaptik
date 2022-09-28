use crate::*;

pub struct Background {
    bounds: Rect,
    render_req: bool,
}

impl Background {
    pub fn new<R: Into<Rect>>(bounds: R) -> Self {
        Self {
            bounds: bounds.into(),
            render_req: true,
        }
    }
}

impl Widget<()> for Background {
    fn update(&mut self, _: ()) {}

    fn invalidate(&mut self) {
        self.render_req = true;
    }

    fn render<C: Canvas>(&mut self, canvas: &mut C) {
        if !self.render_req {
            return;
        }
        self.render_req = false;

        let origin = self.bounds.origin();
        let size = self.bounds.size();
        for x in (0..size.width()).step_by(8) {
            for y in (0..size.height()).step_by(8) {
                let offset = Point(x + origin.x(), y + origin.y());
                let tile = Size(
                    u16::min(8, size.width() - x),
                    u16::min(8, size.height() - y),
                );

                let mut tile_len = tile.width() * tile.height() >> 3;
                while tile_len > 0 {
                    let chunk_size = u16::min(32, tile_len);
                    tile_len -= chunk_size;
                    canvas.draw(Rect(offset, tile), &[0; 32][..chunk_size as usize])
                }
            }
        }
    }
}
