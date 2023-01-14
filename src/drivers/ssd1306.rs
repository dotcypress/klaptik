use crate::*;
use ssd1306::prelude::*;

impl<DI, SIZE, MODE> Canvas for ssd1306::Ssd1306<DI, SIZE, MODE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    fn draw(&mut self, bounds: Rectangle, bitmap: &[u8]) {
        let start = bounds.start().into();
        let end = bounds.end().into();
        self.set_draw_area(start, end).ok();
        self.draw(bitmap).ok();
    }
}
