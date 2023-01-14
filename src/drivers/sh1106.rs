use crate::*;
use sh1106::interface::DisplayInterface;
use sh1106::properties::DisplayProperties;

impl<DI> Canvas for DisplayProperties<DI>
where
    DI: DisplayInterface,
{
    fn draw(&mut self, bounds: Rectangle, bitmap: &[u8]) {
        let start = bounds.start().into();
        let end = bounds.end().into();
        self.set_draw_area(start, end).ok();
        self.draw(bitmap).ok();
    }
}
