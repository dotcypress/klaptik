use crate::*;
use core::marker::PhantomData;

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
        let size = Size::new(SW as _, SH as _);
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
