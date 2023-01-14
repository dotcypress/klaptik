use crate::*;
use display_interface::WriteOnlyDataCommand;
use ist7920::*;

impl<DI, MODE> Canvas for Ist7920<DI, MODE>
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
