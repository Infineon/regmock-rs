use pac::SPI;
use test_pac::{self as pac, RegisterValue};

// our example DUT that we want to "test"
pub fn spi_init() {
    unsafe {
        // reset all sticky status flags
        SPI.status().init(|r| r.set_raw(0));
        // enable SPI
        SPI.ctrl().modify(|r| r.en().set(true));
    };
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use regmock_rs::{given, require_reg, silent};

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
        spi_init();

        assert_eq!(silent(|| unsafe { SPI.status().read() }).get_raw(), 0);
        assert_eq!(silent(|| unsafe { SPI.ctrl().read() }).en().get(), true);
        given!(
            regmock_rs::logs().iter(),
            require_reg!(SPI.status(), all_writes_before_writes_to(SPI.ctrl()))
        );
    }
}
