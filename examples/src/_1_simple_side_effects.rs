use pac::{spi, SPI};
use test_pac as pac;

// our example DUT that we want to "test"
pub fn spi_init(special_config: bool) {
    unsafe {
        let r = spi::Ctrl::default();
        SPI.ctrl().write(r);
        let r = r
            .cpha()
            .set((special_config as u8).into())
            .cpol()
            .set(special_config);
        SPI.ctrl().write(r);
        let r = r.en().set(true);
        SPI.ctrl().write(r);
    };
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use regmock_rs::silent;

    use super::*;
    use crate::common::init_mock;

    // simple tests that just verify the system "state" after calling the DUT
    //
    // we call `spi_init` and afterwards verify that the bits are set correctly
    //
    #[test]
    fn test_spi_init_special() {
        init_mock(None);

        spi_init(true);

        let ctrl = silent(|| unsafe { SPI.ctrl().read() });
        assert_eq!(ctrl.en().get(), true);
        assert_eq!(ctrl.cpha().get().0, spi::ctrl::Cpha::CPHA_FIRST_SHIFTS.0);
        assert_eq!(ctrl.cpol().get(), true);
    }

    #[test]
    fn test_spi_init_normal() {
        init_mock(None);

        spi_init(false);

        let ctrl = silent(|| unsafe { SPI.ctrl().read() });
        assert_eq!(ctrl.en().get(), true);
        assert_eq!(ctrl.cpha().get().0, spi::ctrl::Cpha::CPHA_FIRST_LATCHES.0);
        assert_eq!(ctrl.cpol().get(), false);
    }
}
