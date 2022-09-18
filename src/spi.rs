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

pub struct SpiDisplay<SPI, CE, DC>
where
    SPI: spi::Write<u8>,
    CE: OutputPin,
    DC: OutputPin,
{
    spi: SPI,
    ce: CE,
    dc: DC,
}

impl<SPI, CE, DC> SpiDisplay<SPI, CE, DC>
where
    SPI: spi::Write<u8>,
    CE: OutputPin,
    DC: OutputPin,
{
    pub fn new(spi: SPI, ce: CE, dc: DC) -> Self {
        Self { spi, ce, dc }
    }

    pub fn release(self) -> (SPI, CE, DC) {
        (self.spi, self.ce, self.dc)
    }

    pub fn tx<RES, TX: FnOnce(&mut SPI) -> Result<RES, <SPI as spi::Write<u8>>::Error>>(
        &mut self,
        cmd: bool,
        tx: TX,
    ) -> Result<RES, Error<SPI>> {
        self.ce.set_low().map_err(|_| Error::PinError)?;
        if cmd {
            self.dc.set_high().map_err(|_| Error::PinError)?;
        } else {
            self.dc.set_low().map_err(|_| Error::PinError)?;
        }
        let res = tx(&mut self.spi).map_err(Error::WriteError);
        self.ce.set_high().map_err(|_| Error::PinError).and(res)
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
