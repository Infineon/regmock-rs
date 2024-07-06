use pac::{spi, SPI};
use test_pac as pac;

pub mod dut {
    use super::*;

    /// Initialize the SPI module
    ///
    /// Normally we set up for mode 0, if `special_config` is set
    /// the module is set up for mode 3
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
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::init_mock;
    use regmock_rs::silent;

    // simple test that just verifies the system "state" after calling the DUT
    //
    // we call `spi_init` and afterwards verify that the bits are set correctly
    #[test]
    fn test_spi_init_special() {
        // we need to initalize so that the PAC know which callback functions to call
        init_mock(None);

        // call the DUT
        dut::spi_init(true);

        // 'read' the fake register content and verify the setup
        let ctrl = silent(|| unsafe { SPI.ctrl().read() });
        assert_eq!(ctrl.en().get(), true);
        assert_eq!(ctrl.cpha().get().0, spi::ctrl::Cpha::CPHA_FIRST_SHIFTS.0);
        assert_eq!(ctrl.cpol().get(), true);
    }

    // simple test that just verifies the system "state" after calling the DUT
    //
    // we call `spi_init` and afterwards verify that the bits are set correctly
    #[test]
    fn test_spi_init_normal() {
        init_mock(None);

        dut::spi_init(false);

        let ctrl = silent(|| unsafe { SPI.ctrl().read() });
        assert_eq!(ctrl.en().get(), true);
        assert_eq!(ctrl.cpha().get().0, spi::ctrl::Cpha::CPHA_FIRST_LATCHES.0);
        assert_eq!(ctrl.cpol().get(), false);
    }
}
