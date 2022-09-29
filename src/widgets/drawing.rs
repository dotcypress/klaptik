use crate::*;
use core::convert::Infallible;
use embedded_graphics_core::pixelcolor::BinaryColor;
use embedded_graphics_core::prelude::*;

pub trait DrawingState
where
    Self: PartialEq + Sized,
{
    fn draw<const N: usize>(&self, widget: &mut Drawing<Self, N>);
}

pub struct Drawing<S: DrawingState + PartialEq + Sized, const N: usize> {
    pub framebuffer: [u8; N],
    state: S,
    bounds: Rectangle,
    render_req: bool,
}

impl<S: DrawingState + PartialEq + Sized, const N: usize> Drawing<S, N> {
    pub fn new<V: Into<S>>(state: V, origin: Point, size: Size) -> Self {
        Self {
            state: state.into(),
            render_req: true,
            framebuffer: [0; N],
            bounds: Rectangle::new(origin, size),
        }
    }
}

impl<S: DrawingState + PartialEq + Sized, const N: usize> Widget<S> for Drawing<S, N> {
    fn update(&mut self, state: S) {
        if self.state == state {
            return;
        }

        state.draw(self);
        self.state = state;
        self.render_req = true;
    }

    fn invalidate(&mut self) {
        self.render_req = true;
    }

    fn render<C: Canvas>(&mut self, canvas: &mut C) {
        if self.render_req {
            self.render_req = false;
            canvas.draw(self.bounds, &self.framebuffer);
        }
    }
}

impl<S: DrawingState + PartialEq + Sized, const N: usize> OriginDimensions for Drawing<S, N> {
    fn size(&self) -> Size {
        self.bounds.size
    }
}

impl<S: DrawingState + PartialEq + Sized, const N: usize> DrawTarget for Drawing<S, N> {
    type Color = BinaryColor;
    type Error = Infallible;

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let val = if color.is_on() { 0xff } else { 0x00 };
        self.framebuffer = [val; N];

        Ok(())
    }

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = Rectangle::new(Point::zero(), self.bounds.size);
        for Pixel(coord, color) in pixels.into_iter().filter(|Pixel(p, _)| bb.contains(*p)) {
            let width = bb.size.width as usize;
            let x = coord.x as usize;
            let y = coord.y as usize;
            let mask = 1 << (y % 8);
            let bit_idx = (x + width * (y >> 3)) << 3;
            let idx = bit_idx >> 3;
            if color.is_on() {
                self.framebuffer[idx] |= mask;
            } else {
                self.framebuffer[idx] &= !mask;
            }
        }

        Ok(())
    }
}