use crate::prelude::*;
use crate::Canvas;
use ssd1306::prelude::*;
use ssd1306::Ssd1306;

impl<DI, SIZE, MODE> Canvas for Ssd1306<DI, SIZE, MODE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    fn draw(&mut self, bounds: Rect, buffer: &[u8]) {
        let origin = bounds.origin();
        let size = bounds.size();
        let start = (origin.x() as u8, origin.y() as u8);
        let end = (
            (origin.x() + size.width()) as u8,
            (origin.y() + size.height()) as u8,
        );
        self.set_draw_area(start, end).expect("draw failed");
        self.draw(buffer).expect("draw failed");
    }
}
