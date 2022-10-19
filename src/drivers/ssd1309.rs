use crate::*;
use display_interface::WriteOnlyDataCommand;
use ssd1309::properties::DisplayProperties;

impl<DI> Canvas for DisplayProperties<DI>
where
    DI: WriteOnlyDataCommand,
{
    fn draw(&mut self, bounds: Rectangle, buffer: &[u8]) {
        let origin = bounds.origin;
        let size = bounds.size;
        let start = (origin.x as u8, origin.y as u8);
        let end = (
            (origin.x as u32 + size.width as u32) as u8,
            (origin.y as u32 + size.height as u32) as u8,
        );
        self.set_draw_area(start, end).ok();
        self.draw(buffer).ok();
    }
}
