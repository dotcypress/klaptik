use crate::*;
use embedded_hal::blocking::i2c;

pub enum FxCommand {
    ReadRegister = 0,
    DisplayOn = 1,
    BacklightLevel = 2,
}

pub struct FxDisplay<L, const ADDR: usize, const N: usize> {
    link: L,
    glyph_map: [(SpriteId, Glyphs); N],
}

impl<L, const ADDR: usize, const N: usize> FxDisplay<L, ADDR, N> {
    pub const fn new(link: L, glyph_map: [(SpriteId, Glyphs); N]) -> Self {
        Self { link, glyph_map }
    }

    pub fn link(&mut self) -> &mut L {
        &mut self.link
    }

    pub fn release(self) -> L {
        self.link
    }
}

impl<L: i2c::Write, const ADDR: usize, const N: usize> FxDisplay<L, ADDR, N> {
    pub fn switch_display(&mut self, on: bool) -> Result<(), <L as i2c::Write>::Error> {
        self.link
            .write(ADDR as _, &[FxCommand::DisplayOn as _, on as _])
    }

    pub fn set_backlight(&mut self, level: u8) -> Result<(), <L as i2c::Write>::Error> {
        self.link
            .write(ADDR as _, &[FxCommand::BacklightLevel as _, level])
    }
}

impl<L: i2c::WriteRead, const ADDR: usize, const N: usize> FxDisplay<L, ADDR, N> {
    pub fn read_register(&mut self, reg: u8) -> Result<[u8; 4], <L as i2c::WriteRead>::Error> {
        let mut scratch = [0; 4];
        self.link.write_read(
            ADDR as _,
            &[FxCommand::ReadRegister as _, reg],
            &mut scratch,
        )?;
        Ok(scratch)
    }
}

impl<L: i2c::Write, const ADDR: usize, const N: usize> Display for FxDisplay<L, ADDR, N> {
    fn render(&mut self, req: RenderRequest) {
        let req = self
            .glyph_map
            .iter()
            .find(|(sprite_id, _)| req.sprite_id == *sprite_id)
            .and_then(|(_, glyphs)| glyphs.index(req.glyph))
            .map(|idx| RenderRequest::new(req.origin, req.sprite_id, idx as u8))
            .unwrap_or(req);
        self.link.write(ADDR as u8 | 1, &req.as_bytes()).ok();
    }
}
