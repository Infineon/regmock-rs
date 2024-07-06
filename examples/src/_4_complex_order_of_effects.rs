use pac::{spi, SPI};
use test_pac as pac;

pub mod dut {
    use super::*;

    /// Initialize the SPI module
    ///
    /// Normally we set up for mode 0, if `special_config` is set
    /// the module is set up for mode 3.
    ///
    /// Disable the module before changing it's config, enable it
    /// at the end.
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
    use regmock_rs::{
        silent,
        utils::{access_gen::write_value, RegisterAccessType},
    };
    use test_pac::RegisterValue;

    /// Verify the system state as in previous examples, but also show
    /// different ways how to ensure that the module was disabled in the
    /// beginning, and enabled in the end.
    #[test]
    fn test_spi_init_special() {
        init_mock(None);

        // call our DUT
        dut::spi_init(true);

        // Verify the state after setup
        let ctrl = silent(|| unsafe { SPI.ctrl().read() });
        // check that the config is set correctly
        assert_eq!(ctrl.en().get(), true);
        assert_eq!(ctrl.cpha().get().0, spi::ctrl::Cpha::CPHA_FIRST_SHIFTS.0);
        assert_eq!(ctrl.cpol().get(), true);

        // Verify that first write disabled the SPI
        let logs = regmock_rs::logs();
        let writes = logs
            .iter()
            .filter(|a| a.ty == Some(RegisterAccessType::WRITE))
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(writes[0].addr.unwrap(), SPI.ctrl().addr());
        assert_eq!(
            writes[0].after.unwrap() as u32 & spi::Ctrl::default().en().mask(),
            0
        );

        // or, similar (compares whole written values, not only EN bit)

        assert_eq!(writes[0], write_value(SPI.ctrl().addr(), 0));

        // check that we did not enable the SPI before being ready, by checking there was no write
        // with the enable bit set in all but the last write access
        assert!(writes
            .iter()
            .rev() // reverse since there is no "skip last"
            .skip(1) // remove the last write which enables the SPI
            .all(|a| !(a.ty.as_ref().unwrap() == &RegisterAccessType::WRITE
                && a.addr.unwrap() == SPI.ctrl().addr()
                && spi::Ctrl::new(a.after.unwrap() as u32).en().get())));
    }
}
