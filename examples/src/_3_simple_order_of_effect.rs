use pac::SPI;
use test_pac::{self as pac, RegisterValue};

pub mod dut {
    use super::*;

    /// Initialize the SPI by resetting all sticky status bits
    /// and enabling it afterwards
    pub fn spi_init() {
        unsafe {
            // reset all sticky status flags
            SPI.status().init(|r| r.set_raw(0));
            // enable SPI
            SPI.ctrl().modify(|r| r.en().set(true));
        };
    }
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::init_mock;
    use regmock_rs::{given, require_reg, silent};

    /// Verify that the DUT leaves the system in the correct state.
    /// In contrast to the first example, also verify that the enable
    /// bit was written after the status bits were cleared, as required.
    #[test]
    fn test_spi_init_special() {
        init_mock(None);

        // 'set' all the bits in the status register, otherwise we'd not
        // test anything since registers default to 0#
        //
        // since we intend to check the access log at the end of the test
        // it's important that the testbench preparation happens inside
        // `silent` so that those 'writes' are not recorded in the log
        silent(|| unsafe { SPI.status().init(|r| r.set_raw(0xffffffffu32)) });

        // call our DUT
        dut::spi_init();

        // check that the registers were written to the correct values
        //
        // Note: depending on the test it may be necessary to 'set' the
        // register values to a different value
        assert_eq!(silent(|| unsafe { SPI.status().read() }).get_raw(), 0);
        assert_eq!(silent(|| unsafe { SPI.ctrl().read() }).en().get(), true);
        // use the log to ensure that the enable was written after the status
        // was cleared.
        given!(
            regmock_rs::logs().iter(),
            require_reg!(SPI.status(), all_writes_before_writes_to(SPI.ctrl()))
        );
    }
}
