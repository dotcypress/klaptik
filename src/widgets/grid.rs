use crate::layout::*;
use crate::*;
use core::marker::PhantomData;

pub struct Grid<L: Layout, const LEN: usize> {
    layout: PhantomData<L>,
    sprite_id: SpriteId,
    state: [Glyph; LEN],
    origins: [Point; LEN],
    invalidate: [bool; LEN],
    cursor: usize,
}

impl<L: Layout, const LEN: usize> Grid<L, LEN>
where
    L: Layout,
{
    pub fn new<SI: Into<SpriteId>>(
        sprite_id: SI,
        val: &str,
        origin: Point,
        sprite_size: Size,
    ) -> Self {
        let mut state: [Glyph; LEN] = [0; LEN];
        let mut invalidate: [bool; LEN] = [false; LEN];

        for (idx, sym) in val.bytes().enumerate() {
            state[idx] = sym;
            invalidate[idx] = true;
        }

        let mut origins: [Point; LEN] = [Point::zero(); LEN];
        for (idx, pos) in origins.iter_mut().enumerate() {
            *pos = L::layout(idx, origin, sprite_size);
        }

        Self {
            origins,
            state,
            invalidate,
            cursor: 0,
            sprite_id: sprite_id.into(),
            layout: PhantomData {},
        }
    }
}

impl<L: Layout, const LEN: usize> Grid<L, LEN> {
    pub fn set_glyph(&mut self, idx: usize, glyph: Glyph) {
        if self.state[idx] != glyph {
            self.state[idx] = glyph;
            self.invalidate[idx] = true;
        }
    }
}

impl<L: Layout, const LEN: usize> Widget<&[Glyph; LEN]> for Grid<L, LEN> {
    fn update(&mut self, state: &[Glyph; LEN]) {
        for (idx, glyph) in state.iter().enumerate() {
            self.set_glyph(idx, *glyph);
        }
        self.cursor = 0;
    }

    fn invalidate(&mut self) {
        for invalidate in self.invalidate.iter_mut() {
            *invalidate = true;
        }
    }

    fn render<D: Display>(&mut self, display: &mut D) {
        for (idx, invalidate) in self.invalidate.iter_mut().enumerate() {
            if *invalidate {
                let glyph = self.state[idx];
                display.render(RenderRequest::new(self.origins[idx], self.sprite_id, glyph));
                *invalidate = false;
            }
        }
    }
}

impl<L: Layout, const LEN: usize> core::fmt::Write for Grid<L, LEN> {
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
