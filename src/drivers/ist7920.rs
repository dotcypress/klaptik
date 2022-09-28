use crate::prelude::*;
use crate::Canvas;
use display_interface::WriteOnlyDataCommand;
use ist7920::*;

impl<DI, MODE> Canvas for Ist7920<DI, MODE>
where
    DI: WriteOnlyDataCommand,
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
