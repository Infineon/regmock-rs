use pac::SPI;
use test_pac as pac;

pub mod dut {
    use super::*;

    /// Write all `bytes` to the send buffer, do not check if there is enough space...
    pub fn spi_send(bytes: &[u8]) {
        for &b in bytes {
            unsafe {
                SPI.tx().init(|r| r.data().set(b));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::init_mock;
    use regmock_rs::{given, require_reg, require_seq, utils::access_gen::write_value};

    /// This example shows different ways of checking that a specific
    /// sequence of values awas written to a register (or that a specific sequence of
    /// accesses did happen)
    #[test]
    fn test_send_bytes() {
        init_mock(None);

        // call our DUT
        dut::spi_send(&[0x11, 0x22, 0x33]);

        // Verify the values written by using a matcher
        //
        // This does only verify writes to `tx` and ignores writes
        // to other registers, and all reads
        given!(
            full_log,
            require_reg!(SPI.tx(), values_written_are([0x11, 0x22, 0x33]))
        );

        // ... or use the sequence matcher
        //
        // This does check all the accesses that happened to all
        // registers.
        // Below the full log is used, but we could limit the accesses
        // checked by passing an iterator to `given!`, e.g.
        // `regmock_rs::log().iter().filter(|a| ...)`
        let w0 = write_value(SPI.tx().addr(), 0x11);
        let w1 = write_value(SPI.tx().addr(), 0x22);
        let w2 = write_value(SPI.tx().addr(), 0x33);
        given!(full_log, require_seq!(vec![&w0, &w1, &w2]));

        // ... or explicitly match
        //
        // The most "feature packed" version, but also the one that does
        // need the most groundwork.
        assert_eq!(
            regmock_rs::logs().iter().collect::<Vec<_>>(),
            vec![
                &write_value(SPI.tx().addr(), 0x11),
                &write_value(SPI.tx().addr(), 0x22),
                &write_value(SPI.tx().addr(), 0x33),
            ]
        );
    }
}
