use crate::*;
use core::marker::PhantomData;

pub struct Grid<S, L, const LEN: usize>
where
    S: Sprite,
    L: Layout,
{
    layout: PhantomData<L>,
    sprite: S,
    state: [Glyph; LEN],
    origins: [Point; LEN],
    render_req: [bool; LEN],
    cursor: usize,
}

impl<S, L, const LEN: usize> Grid<S, L, LEN>
where
    S: Sprite + Copy,
    L: Layout,
{
    pub fn new(sprite: S, val: &str, origin: Point) -> Self {
        let glyph = sprite.glyphs().next().unwrap();
        let mut state: [Glyph; LEN] = [glyph; LEN];
        let mut render_req: [bool; LEN] = [false; LEN];

        for (idx, sym) in val.bytes().enumerate() {
            state[idx] = sym;
            render_req[idx] = true;
        }

        let mut origins: [Point; LEN] = [Point::default(); LEN];
        let size = sprite.size();
        for (idx, pos) in origins.iter_mut().enumerate() {
            *pos = L::layout(idx, origin, size);
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

impl<S, L, const LEN: usize> Grid<S, L, LEN>
where
    S: Sprite,
    L: Layout,
{
    pub fn set_glyph(&mut self, idx: usize, glyph: Glyph) {
        if self.state[idx] != glyph {
            self.state[idx] = glyph;
            self.render_req[idx] = true;
        }
    }
}

impl<S, L, const LEN: usize> Widget<&[Glyph; LEN]> for Grid<S, L, LEN>
where
    S: Sprite,
    L: Layout,
{
    fn update(&mut self, state: &[Glyph; LEN]) {
        for (idx, glyph) in state.iter().enumerate() {
            self.set_glyph(idx, *glyph);
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

impl<S, L, const LEN: usize> core::fmt::Write for Grid<S, L, LEN>
where
    S: Sprite,
    L: Layout,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut cursor = self.cursor;
        for glyph in s.as_bytes() {
            self.set_glyph(cursor, *glyph);
            cursor += 1;
            if cursor >= LEN {
                cursor = 0;
            }
        }
        self.cursor = cursor;
        Ok(())
    }
}
