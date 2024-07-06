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
    use regmock_rs::{
        silent,
        utils::{access_gen::write_value, RegisterAccessType},
    };

    use super::*;
    use crate::common::init_mock;

    // simple tests that just verify the system "state" after calling the DUT
    //
    // we call `spi_init` and afterwards verify that the bits are set correctly
    //
    #[test]
    fn test_spi_init_special() {
        init_mock(None);

        // call our DUT
        spi_init(true);

        let ctrl = silent(|| unsafe { SPI.ctrl().read() });
        // check that the config is set correctly
        assert_eq!(ctrl.en().get(), true);
        assert_eq!(ctrl.cpha().get().0, spi::ctrl::Cpha::CPHA_FIRST_SHIFTS.0);
        assert_eq!(ctrl.cpol().get(), true);

        // verify that first write disabled the SPI
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
        assert!(writes
            .iter()
            .rev()
            .skip(1) // remove the last write which enables the SPI
            .all(|a| a.ty.as_ref().unwrap() == &RegisterAccessType::WRITE
                && (a.addr.unwrap() != SPI.ctrl().addr()
                    || a.after.unwrap() as u32 & spi::Ctrl::default().en().mask() == 0)));
    }
}
