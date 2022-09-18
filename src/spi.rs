use core::convert::Infallible;

use embedded_hal::blocking::spi;
use embedded_hal::digital::v2::*;

pub struct NoCS;

impl OutputPin for NoCS {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct NoDC;

impl OutputPin for NoDC {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub enum Error<SPI: spi::Write<u8>> {
    PinError,
    WriteError(<SPI as spi::Write<u8>>::Error),
}

impl<SPI: spi::Write<u8>> core::fmt::Debug for Error<SPI> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PinError => write!(f, "GPIO Error"),
            Self::WriteError(_) => write!(f, "SPI Write Error"),
        }
    }
}

pub struct SpiDisplay<SPI, CS, DC>
where
    SPI: spi::Write<u8>,
    CS: OutputPin,
    DC: OutputPin,
{
    spi: SPI,
    cs: CS,
    dc: DC,
}

impl<SPI, CS, DC> SpiDisplay<SPI, CS, DC>
where
    SPI: spi::Write<u8>,
    CS: OutputPin,
    DC: OutputPin,
{
    pub fn new(spi: SPI, cs: CS, dc: DC) -> Self {
        Self { spi, cs, dc }
    }

    pub fn release(self) -> (SPI, CS, DC) {
        (self.spi, self.cs, self.dc)
    }

    pub fn tx<RES, TX: FnOnce(&mut SPI) -> Result<RES, <SPI as spi::Write<u8>>::Error>>(
        &mut self,
        cmd: bool,
        tx: TX,
    ) -> Result<RES, Error<SPI>> {
        self.cs.set_low().map_err(|_| Error::PinError)?;
        if cmd {
            self.dc.set_high().map_err(|_| Error::PinError)?;
        } else {
            self.dc.set_low().map_err(|_| Error::PinError)?;
        }
        let res = tx(&mut self.spi).map_err(Error::WriteError);
        self.cs.set_high().map_err(|_| Error::PinError).and(res)
    }

    pub fn command<RES, TX: FnOnce(&mut SPI) -> Result<RES, <SPI as spi::Write<u8>>::Error>>(
        &mut self,
        tx: TX,
    ) -> Result<RES, Error<SPI>> {
        self.tx(true, tx)
    }

    pub fn data<RES, TX: FnOnce(&mut SPI) -> Result<RES, <SPI as spi::Write<u8>>::Error>>(
        &mut self,
        tx: TX,
    ) -> Result<RES, Error<SPI>> {
        self.tx(false, tx)
    }
}
