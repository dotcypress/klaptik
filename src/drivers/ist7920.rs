use crate::Canvas;
use display_interface::WriteOnlyDataCommand;
use embedded_graphics_core::primitives::Rectangle;
use ist7920::*;

impl<DI, MODE> Canvas for Ist7920<DI, MODE>
where
    DI: WriteOnlyDataCommand,
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
