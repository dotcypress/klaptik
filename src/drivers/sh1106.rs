use crate::prelude::*;
use crate::Canvas;
use sh1106::properties::DisplayProperties;
use sh1106::interface::DisplayInterface;

impl<DI> Canvas for DisplayProperties<DI>
where
    DI: DisplayInterface,
{
    fn draw(&mut self, bounds: Rect, buffer: &[u8]) {
        let origin = bounds.origin();
        let size = bounds.size();
        let start = (origin.x() as u8, origin.y() as u8);
        let end = (
            (origin.x() + size.width()) as u8,
            (origin.y() + size.height()) as u8,
        );
        self.set_draw_area(start, end).ok();
        self.draw(buffer).ok();
    }
}
