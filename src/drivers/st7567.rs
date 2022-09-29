use crate::drivers::spi::SpiLink;
use crate::Canvas;
use crate::*;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::spi;
use embedded_hal::digital::v2::*;

pub enum Command {
    AllPixelsOn = 0xa5,
    Bias1_7 = 0xa3,
    Bias1_9 = 0xa2,
    Contrast = 0x81,
    DisplayInverse = 0xa7,
    DisplayNormal = 0xa6,
    DisplayOn = 0xaf,
    DisplayOff = 0xae,
    DisplayRAM = 0xa4,
    EnterRWRMode = 0xe0,
    ExitRWRMode = 0xee,
    PowerOff = 0x28,
    PowerOn = 0x2f,
    RegulatorRatio = 0x20,
    Reset = 0xe2,
    SegmentDirectionNormal = 0xa0,
    SegmentDirectionRev = 0xa1,
    SetColumnHigh = 0x10,
    SetColumnLow = 0x00,
    SetCOMNormal = 0xc0,
    SetCOMReverse = 0xc8,
    SetPage = 0xb0,
    SetStartLine = 0x40,
}

pub struct ST7567<SPI, RST, CS, DC>
where
    SPI: spi::Write<u8>,
    RST: OutputPin,
    CS: OutputPin,
    DC: OutputPin,
{
    link: SpiLink<SPI, CS, DC>,
    rst: RST,
}

impl<SPI, RST, CS, DC> ST7567<SPI, RST, CS, DC>
where
    SPI: spi::Write<u8>,
    RST: OutputPin,
    CS: OutputPin,
    DC: OutputPin,
{
    pub fn new(spi: SPI, cs: CS, dc: DC, rst: RST) -> Self {
        Self {
            rst,
            link: SpiLink::new(spi, cs, dc),
        }
    }

    pub fn release(self) -> (SPI, CS, DC, RST) {
        let (spi, cs, dc) = self.link.release();
        (spi, cs, dc, self.rst)
    }

    pub fn reset<D: DelayMs<u32>>(&mut self, delay: &mut D) {
        self.rst.set_low().ok();
        delay.delay_ms(16_u32);
        self.rst.set_high().ok();
        delay.delay_ms(64_u32);

        self.link
            .command(|tx| {
                tx.write(&[
                    Command::SegmentDirectionNormal as _,
                    Command::SetCOMNormal as _,
                    Command::DisplayNormal as _,
                    Command::SetStartLine as _,
                    Command::PowerOn as _,
                ])
            })
            .ok();
    }

    pub fn on(&mut self) {
        self.link
            .command(|tx| tx.write(&[Command::DisplayOn as _]))
            .ok();
    }

    pub fn off(&mut self) {
        self.link
            .command(|tx| tx.write(&[Command::DisplayOff as _]))
            .ok();
    }
}

impl<SPI, RST, CS, DC> Canvas for ST7567<SPI, RST, CS, DC>
where
    SPI: spi::Write<u8>,
    RST: OutputPin,
    CS: OutputPin,
    DC: OutputPin,
{
    fn draw(&mut self, bounds: Rectangle, buf: &[u8]) {
        let col = bounds.top_left.x as u8;
        let page = bounds.top_left.y as u32 >> 3;
        let chunks = bounds.size.height >> 3;
        let width = bounds.size.width as usize;

        self.link
            .command(|tx| tx.write(&[Command::EnterRWRMode as _]))
            .ok();
        for chunk in 0..chunks {
            self.link
                .command(|tx| {
                    tx.write(&[
                        Command::SetPage as u8 | (page + chunk) as u8,
                        Command::SetColumnLow as u8 | (col & 0x0f),
                        Command::SetColumnHigh as u8 | (col >> 4),
                    ])
                })
                .ok();
            let offset = width * chunk as usize;
            self.link
                .data(|tx| tx.write(&buf[offset..(offset + width)]))
                .ok();
        }

        self.link
            .command(|tx| tx.write(&[Command::ExitRWRMode as _]))
            .ok();
    }
}
