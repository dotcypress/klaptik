use crate::*;
use display_interface::WriteOnlyDataCommand;
use ssd1309::properties::DisplayProperties;

impl<DI> Canvas for DisplayProperties<DI>
where
    DI: WriteOnlyDataCommand,
{
    fn draw(&mut self, bounds: Rectangle, bitmap: &[u8]) {
        let start = bounds.start().into();
        let end = bounds.end().into();
        self.set_draw_area(start, end).ok();
        self.draw(bitmap).ok();
    }
}
