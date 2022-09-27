use crate::*;
use core::marker::PhantomData;

pub trait Widget<S> {
    fn invalidate(&mut self);
    fn update(&mut self, state: S);
    fn render<C: Canvas>(&mut self, canvas: &mut C);
}

pub struct Background {
    bounds: Rect,
    render_req: bool,
}

impl Background {
    pub fn new<R: Into<Rect>>(bounds: R) -> Self {
        Self {
            bounds: bounds.into(),
            render_req: true,
        }
    }
}

impl Widget<()> for Background {
    fn update(&mut self, _: ()) {}

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
        for x in (0..size.width()).step_by(8) {
            for y in (0..size.height()).step_by(8) {
                let offset = Point(x + origin.x(), y + origin.y());
                let tile = Size(
                    u16::min(8, size.width() - x),
                    u16::min(8, size.height() - y),
                );

                let mut tile_len = tile.width() * tile.height() >> 3;
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
    pub fn new<P: Into<Point>>(sprite: S, state: Glyph, origin: P) -> Self {
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

    fn update(&mut self, state: Glyph) {
        if self.state != state {
            self.state = state;
            self.render_req = true;
        }
    }

    fn render<C: Canvas>(&mut self, canvas: &mut C) {
        if self.render_req {
            self.sprite.render(self.state, self.origin, canvas);
            self.render_req = false;
        }
    }
}

pub type SpriteIcon = Icon<RomSprite>;

pub trait Layout {
    fn layout(node_idx: usize, origin: Point, glyph_size: Size) -> Point;
}

pub struct Grid<S: Sprite, L: Layout, const SIZE: usize, const SW: usize, const SH: usize> {
    layout: PhantomData<L>,
    sprite: S,
    state: [Glyph; SIZE],
    origins: [Point; SIZE],
    render_req: [bool; SIZE],
    cursor: usize,
}

impl<S: Sprite + Copy, L: Layout, const SIZE: usize, const SW: usize, const SH: usize>
    Grid<S, L, SIZE, SW, SH>
{
    pub fn new<P: Into<Point>>(sprite: S, val: &str, origin: P) -> Self {
        let glyph = sprite.glyphs()[0];
        let mut state: [Glyph; SIZE] = [glyph; SIZE];
        let mut render_req: [bool; SIZE] = [false; SIZE];

        for (idx, sym) in val.bytes().enumerate() {
            state[idx] = sym;
            render_req[idx] = true;
        }

        let mut origins: [Point; SIZE] = [Point::default(); SIZE];
        let pos = origin.into();
        let size = Size(SW as _, SH as _);
        for (idx, origin) in origins.iter_mut().enumerate() {
            *origin = L::layout(idx, pos, size);
        }

        Self {
            origins,
            sprite,
            state,
            render_req,
            cursor: 0,
            layout: PhantomData {},
        }
    }
}

impl<S: Sprite, L: Layout, const SIZE: usize, const SW: usize, const SH: usize>
    Grid<S, L, SIZE, SW, SH>
{
    pub fn set_glyph(&mut self, idx: usize, glyph: Glyph) {
        if self.state[idx] != glyph {
            self.state[idx] = glyph;
            self.render_req[idx] = true;
        }
    }
}

impl<S: Sprite, L: Layout, const SIZE: usize, const SW: usize, const SH: usize>
    Widget<&[Glyph; SIZE]> for Grid<S, L, SIZE, SW, SH>
{
    fn update(&mut self, state: &[Glyph; SIZE]) {
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
                self.sprite.render(glyph, self.origins[idx], canvas);
                *render_req = false;
            }
        }
    }
}

impl<S: Sprite, L: Layout, const SIZE: usize, const SW: usize, const SH: usize> core::fmt::Write
    for Grid<S, L, SIZE, SW, SH>
{
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
    fn layout(node_idx: usize, origin: Point, size: Size) -> Point {
        let idx = node_idx as u16 % WRAP;
        let wraps = node_idx as u16 / WRAP;
        match DIR {
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
        }
    }
}

pub type Label<S, const SIZE: usize, const SW: usize, const SH: usize> =
    Grid<S, GridLayout<DIR_LTR, { u16::MAX }>, SIZE, SW, SH>;
pub type VerticalLabel<S, const SIZE: usize, const SW: usize, const SH: usize> =
    Grid<S, GridLayout<DIR_DOWN, { u16::MAX }>, SIZE, SW, SH>;
pub type TextBox<S, const SIZE: usize, const SW: usize, const SH: usize, const WRAP: u16> =
    Grid<S, GridLayout<DIR_LTR, WRAP>, SIZE, SW, SH>;
