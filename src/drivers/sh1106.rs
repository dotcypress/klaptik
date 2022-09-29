use crate::*;
use sh1106::interface::DisplayInterface;
use sh1106::properties::DisplayProperties;

impl<DI> Canvas for DisplayProperties<DI>
where
    DI: DisplayInterface,
{
    fn draw(&mut self, bounds: Rectangle, buffer: &[u8]) {
        let origin = bounds.top_left;
        let size = bounds.size;
        let start = (origin.x as u8, origin.y as u8);
        let end = (
            (origin.x as u32 + size.width) as u8,
            (origin.y as u32 + size.height) as u8,
        );
        self.set_draw_area(start, end).ok();
        self.draw(buffer).ok();
    }
}
