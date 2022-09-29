use crate::*;
use ssd1306::prelude::*;

impl<DI, SIZE, MODE> Canvas for ssd1306::Ssd1306<DI, SIZE, MODE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
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
