use crate::*;

pub struct Background {
    bounds: Rectangle,
    render_req: bool,
}

impl Background {
    pub const fn new(origin: Point, size: Size) -> Self {
        Self {
            bounds: Rectangle::new(origin, size),
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
        canvas.clear(self.bounds);
    }
}
