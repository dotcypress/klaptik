use crate::*;

pub type Background = Tile<0>;

pub struct Tile<const P: u8> {
    bounds: Rectangle,
    render_req: bool,
}

impl<const P: u8> Tile<P> {
    pub fn new(origin: Point, size: Size) -> Self {
        Self {
            bounds: Rectangle::new(origin, size),
            render_req: true,
        }
    }
}

impl<const P: u8> Widget<()> for Tile<P> {
    fn update(&mut self, _: ()) {}

    fn invalidate(&mut self) {
        self.render_req = true;
    }

    fn render<C: Canvas>(&mut self, canvas: &mut C) {
        if !self.render_req {
            return;
        }
        self.render_req = false;

        let origin = self.bounds.top_left;
        let size = self.bounds.size;
        for x in (0..size.width).step_by(8) {
            for y in (0..size.height).step_by(8) {
                let offset = Point::new(x as i32 + origin.x, y as i32 + origin.y);
                let tile = Size::new(u32::min(8, size.width - x), u32::min(8, size.height - y));

                let mut tile_len = tile.width * tile.height >> 3;
                while tile_len > 0 {
                    let chunk_size = u32::min(32, tile_len);
                    tile_len -= chunk_size;
                    canvas.draw(
                        Rectangle::new(offset, tile),
                        &[P; 32][..chunk_size as usize],
                    )
                }
            }
        }
    }
}
