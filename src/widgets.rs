use crate::*;
use core::marker::PhantomData;

pub trait Canvas {
    fn draw(&mut self, bounds: Rect, buf: &[u8]);
}

pub trait Widget<S> {
    fn set_state(&mut self, state: S);
    fn render<C: Canvas>(&mut self, canvas: &mut C);
    fn invalidate(&mut self);
}

pub struct Background {
    bpp: u16,
    bounds: Rect,
    render_req: bool,
}

impl Background {
    pub fn new<P: Into<Point>, S: Into<Size>>(origin: P, size: S, bpp: u16) -> Self {
        Self {
            bounds: Rect(origin.into(), size.into()),
            render_req: true,
            bpp,
        }
    }
}

impl Widget<()> for Background {
    fn set_state(&mut self, _: ()) {}

    fn invalidate(&mut self) {
        self.render_req = true;
    }

    fn render<C: Canvas>(&mut self, canvas: &mut C) {
        if !self.render_req {
            return;
        }
        self.render_req = false;

        let origin = self.bounds.origin();
        let size = self.bounds.size();
        for x in (0..size.width()).step_by(16) {
            for y in (0..size.height()).step_by(16) {
                let offset = Point(x + origin.x(), y + origin.y());
                let tile = Size(
                    u16::min(16, size.width() - x),
                    u16::min(16, size.height() - y),
                );

                let mut tile_len = tile.width() * tile.height() * self.bpp / 8;
                while tile_len > 0 {
                    let chunk_size = u16::min(32, tile_len);
                    tile_len -= chunk_size;
                    canvas.draw(Rect(offset, tile), &[0; 32][..chunk_size as usize])
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Icon<S> {
    sprite: S,
    state: Glyph,
    origin: Point,
    render_req: bool,
}

impl<S: Sprite + Copy> Icon<S> {
    pub fn new<P: Into<Point>>(origin: P, sprite: S, state: Glyph) -> Self {
        Self {
            sprite,
            state,
            origin: origin.into(),
            render_req: true,
        }
    }
}

impl<S: Sprite> Widget<Glyph> for Icon<S> {
    fn invalidate(&mut self) {
        self.render_req = true;
    }

    fn set_state(&mut self, state: Glyph) {
        if self.state != state {
            self.state = state;
            self.render_req = true;
        }
    }

    fn render<C: Canvas>(&mut self, canvas: &mut C) {
        if self.render_req {
            if let Some(sprite) = self.sprite.render(self.state) {
                canvas.draw(Rect(self.origin, self.sprite.glyph_size()), sprite);
            }
            self.render_req = false;
        }
    }
}

pub trait Layout {
    fn layout(node_idx: usize, origin: Point, glyph_size: Size) -> Rect;
}

pub struct Grid<S: Sprite, L: Layout, const SIZE: usize> {
    layout: PhantomData<L>,
    sprite: S,
    state: [Glyph; SIZE],
    bounds: [Rect; SIZE],
    render_req: [bool; SIZE],
    cursor: usize,
}

impl<S: Sprite + Copy, L: Layout, const SIZE: usize> Grid<S, L, SIZE> {
    pub fn new<P: Into<Point>>(origin: P, sprite: S, val: &str) -> Self {
        let state = val.as_bytes();
        assert!(state.len() <= SIZE);
        let glyph = sprite.glyphs()[0];
        let mut state: [Glyph; SIZE] = [glyph; SIZE];
        let mut render_req: [bool; SIZE] = [false; SIZE];

        for (idx, sym) in val.bytes().enumerate() {
            state[idx] = sym;
            render_req[idx] = true;
        }

        let mut bounds: [Rect; SIZE] = [Rect::default(); SIZE];
        let origin = origin.into();
        let glyph_size = sprite.glyph_size();
        for (idx, rect) in bounds.iter_mut().enumerate() {
            *rect = L::layout(idx, origin, glyph_size);
        }

        Self {
            bounds,
            sprite,
            state,
            render_req,
            cursor: 0,
            layout: PhantomData {},
        }
    }
}

impl<S: Sprite, L: Layout, const SIZE: usize> Grid<S, L, SIZE> {
    pub fn set_glyph(&mut self, idx: usize, glyph: Glyph) {
        if self.state[idx] != glyph {
            self.state[idx] = glyph;
            self.render_req[idx] = true;
        }
    }
}

impl<S: Sprite, L: Layout, const SIZE: usize> Widget<&[Glyph; SIZE]> for Grid<S, L, SIZE> {
    fn set_state(&mut self, state: &[Glyph; SIZE]) {
        for (idx, glyph) in state.iter().enumerate() {
            if self.state[idx] != *glyph {
                self.state[idx] = *glyph;
                self.render_req[idx] = true;
            }
        }
        self.cursor = 0;
    }

    fn invalidate(&mut self) {
        for render_req in self.render_req.iter_mut() {
            *render_req = true;
        }
    }

    fn render<C: Canvas>(&mut self, canvas: &mut C) {
        for (idx, render_req) in self.render_req.iter_mut().enumerate() {
            if *render_req {
                let glyph = self.state[idx];
                if let Some(buf) = self.sprite.render(glyph) {
                    canvas.draw(self.bounds[idx], buf);
                }
                *render_req = false;
            }
        }
    }
}

impl<S: Sprite, L: Layout, const SIZE: usize> core::fmt::Write for Grid<S, L, SIZE> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut cursor = self.cursor;
        for glyph in s.as_bytes() {
            if self.state[cursor] != *glyph {
                self.state[cursor] = *glyph;
                self.render_req[cursor] = true;
            }
            cursor += 1;
            if cursor >= SIZE {
                cursor = 0;
            }
        }
        self.cursor = cursor;
        Ok(())
    }
}

pub struct GridLayout<const DIR: usize, const WRAP: u16>;

const DIR_LTR: usize = 0;
const DIR_RTL: usize = 1;
const DIR_DOWN: usize = 2;
const DIR_UP: usize = 3;

impl<const DIR: usize, const WRAP: u16> Layout for GridLayout<DIR, WRAP> {
    fn layout(node_idx: usize, origin: Point, size: Size) -> Rect {
        let idx = node_idx as u16 % WRAP;
        let wraps = node_idx as u16 / WRAP;
        let new_origin = match DIR {
            DIR_UP => Point(
                origin.x() + size.width() * wraps,
                origin.y() - size.height() * (idx + 1),
            ),
            DIR_DOWN => Point(
                origin.x() + size.width() * wraps,
                origin.y() + size.height() * idx,
            ),
            DIR_RTL => Point(
                origin.x() - size.width() * (idx + 1),
                origin.y() + size.height() * wraps,
            ),
            _ => Point(
                origin.x() + size.width() * idx,
                origin.y() + size.height() * wraps,
            ),
        };
        Rect(new_origin, size)
    }
}

pub type Label<S, const SIZE: usize> = Grid<S, GridLayout<DIR_LTR, { u16::MAX }>, SIZE>;
pub type VerticalLabel<S, const SIZE: usize> = Grid<S, GridLayout<DIR_DOWN, { u16::MAX }>, SIZE>;
pub type TextBox<S, const SIZE: usize, const WRAP: u16> = Grid<S, GridLayout<DIR_LTR, WRAP>, SIZE>;
