use pac::SPI;
use test_pac as pac;

pub fn spi_send(bytes: &[u8]) {
    for &b in bytes {
        unsafe {
            SPI.tx().init(|r| r.data().set(b));
        }
    }
}

#[cfg(test)]
mod tests {
    use regmock_rs::utils::access_gen::write_value;

    use super::*;
    use crate::common::init_mock;

    #[test]
    fn test_send_bytes() {
        init_mock(None);

        // call our DUT
        spi_send(&[0x11, 0x22, 0x33]);

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
