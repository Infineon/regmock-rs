mod common;
use common::init_mock;
use regmock_rs::{given, require_reg};
use test_pac::GPIO;

#[cfg(test)]
mod read_last {
    use super::*;

    #[test]
    pub fn simple() {
        init_mock(None);

        unsafe {
            let _ = GPIO.we().read();

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), read_last)
            );
        }
    }

    #[test]
    pub fn other_reads() {
        init_mock(None);

        unsafe {
            let _ = GPIO.we().read();
            let _ = GPIO.out().read();

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), read_last)
            );
        }
    }

    #[test]
    pub fn other_writes() {
        init_mock(None);

        unsafe {
            let _ = GPIO.we().read();
            GPIO.out().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), read_last)
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_write_after_read() {
        init_mock(None);

        unsafe {
            let _ = GPIO.we().read();
            GPIO.we().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), read_last)
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_read_modify_write() {
        init_mock(None);

        unsafe {
            GPIO.we().modify(|r| r);
        }

        given!(
            regmock_rs::logs().iter(),
            require_reg!(GPIO.we(), read_last)
        );
    }
}

#[cfg(test)]
mod not_written {
    use super::*;

    #[test]
    pub fn simple() {
        init_mock(None);

        given!(
            regmock_rs::logs().iter(),
            require_reg!(GPIO.we(), not_written)
        );
    }

    #[test]
    pub fn reads() {
        init_mock(None);

        unsafe {
            let _ = GPIO.we().read();

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), not_written)
            );
        }
    }

    #[test]
    pub fn other_writes() {
        init_mock(None);

        unsafe {
            GPIO.out().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), not_written)
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_write_to_register() {
        init_mock(None);

        unsafe {
            GPIO.we().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), not_written)
            );
        }
    }
}

#[cfg(test)]
mod written_once {
    use super::*;

    #[test]
    pub fn simple() {
        init_mock(None);

        unsafe {
            GPIO.we().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), written_once)
            );
        }
    }

    #[test]
    pub fn multiple_reads() {
        init_mock(None);

        unsafe {
            let _ = GPIO.we().read();
            GPIO.we().init(|r| r);
            let _ = GPIO.we().read();

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), written_once)
            );
        }
    }

    #[test]
    pub fn other_writes() {
        init_mock(None);

        unsafe {
            GPIO.out().init(|r| r);
            GPIO.we().init(|r| r);
            GPIO.out().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), written_once)
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_no_write() {
        init_mock(None);

        unsafe {
            GPIO.out().init(|r| r);
            GPIO.out().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), written_once)
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_written_multiple_times() {
        init_mock(None);

        unsafe {
            GPIO.we().init(|r| r);
            GPIO.out().init(|r| r);
            GPIO.we().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), written_once)
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_written_multiple_times_back_to_back() {
        // run this test separately because of compression of number of accesses
        init_mock(None);

        unsafe {
            GPIO.we().init(|r| r);
            GPIO.we().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), written_once)
            );
        }
    }
}

#[cfg(test)]
mod all_writes_before_writes_to {
    use test_pac::SPI;

    use super::*;

    #[test]
    pub fn simple() {
        init_mock(None);

        unsafe {
            GPIO.we().init(|r| r);
            GPIO.out().init(|r| r);
            given!(
                regmock_rs::logs().iter(),
                require_reg!(GPIO.we(), all_writes_before_writes_to(GPIO.out()))
            );
        }
    }

    #[test]
    pub fn other_writes() {
        init_mock(None);

        unsafe {
            SPI.ctrl().init(|r| r);
            SPI.tx().init(|r| r);
            SPI.ctrl().init(|r| r);
            SPI.status().init(|r| r);
            SPI.ctrl().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.tx(), all_writes_before_writes_to(SPI.status()))
            );
        }
    }

    #[test]
    pub fn reads_after() {
        init_mock(None);

        unsafe {
            SPI.ctrl().init(|r| r);
            SPI.tx().init(|r| r);
            let _ = SPI.ctrl().read();

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), all_writes_before_writes_to(SPI.tx()))
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn written_after() {
        init_mock(None);

        unsafe {
            SPI.ctrl().init(|r| r);
            SPI.tx().init(|r| r);
            SPI.ctrl().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), all_writes_before_writes_to(SPI.tx()))
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn read_modify_write_after() {
        init_mock(None);

        unsafe {
            SPI.ctrl().init(|r| r);
            SPI.tx().init(|r| r);
            SPI.ctrl().modify(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), all_writes_before_writes_to(SPI.tx()))
            );
        }
    }
}

#[cfg(test)]
mod written_before {
    use test_pac::SPI;

    use super::*;

    #[test]
    pub fn simple() {
        init_mock(None);

        unsafe {
            SPI.ctrl().init(|r| r);
            SPI.tx().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), written_to_before_write_to(SPI.tx()))
            );
        }
    }

    #[test]
    pub fn writes_and_reads_before_and_after() {
        init_mock(None);

        unsafe {
            let _ = SPI.ctrl().read();
            SPI.ctrl().init(|r| r);
            SPI.tx().init(|r| r);
            let _ = SPI.ctrl().read();
            SPI.ctrl().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), written_to_before_write_to(SPI.tx()))
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_only_write_after() {
        init_mock(None);

        unsafe {
            SPI.tx().init(|r| r);
            SPI.ctrl().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), written_to_before_write_to(SPI.tx()))
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_no_write() {
        init_mock(None);

        unsafe {
            SPI.tx().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), written_to_before_write_to(SPI.tx()))
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_only_read_before() {
        init_mock(None);

        unsafe {
            let _ = SPI.ctrl().read();
            SPI.tx().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), written_to_before_write_to(SPI.tx()))
            );
        }
    }
}

#[cfg(test)]
mod values_written_to {
    use test_pac::{spi, SPI};

    use super::*;

    #[test]
    pub fn simple() {
        init_mock(None);

        unsafe {
            SPI.ctrl()
                .init(|r| r.cpha().set(spi::ctrl::Cpha::CPHA_FIRST_LATCHES));
            SPI.ctrl().init(|r| r.cpol().set(true));

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), values_written_are([0x0u8, 0x4u8]))
            );
        }
    }

    #[test]
    pub fn other_writes() {
        init_mock(None);

        unsafe {
            SPI.status().init(|r| r);
            SPI.ctrl()
                .init(|r| r.cpha().set(spi::ctrl::Cpha::CPHA_FIRST_LATCHES));
            SPI.tx().init(|r| r);
            SPI.ctrl().init(|r| r.cpol().set(true));
            SPI.rx().init(|r| r);

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), values_written_are([0x0u8, 0x4u8]))
            );
        }
    }

    #[test]
    pub fn other_reads() {
        init_mock(None);

        unsafe {
            let _ = SPI.ctrl().read();
            SPI.ctrl()
                .init(|r| r.cpha().set(spi::ctrl::Cpha::CPHA_FIRST_LATCHES));
            let _ = SPI.rx().read();
            SPI.ctrl().init(|r| r.cpol().set(true));
            let _ = SPI.ctrl().read();

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), values_written_are([0x0u8, 0x4u8]))
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_wrong_value() {
        init_mock(None);

        unsafe {
            SPI.ctrl()
                .init(|r| r.cpha().set(spi::ctrl::Cpha::CPHA_FIRST_SHIFTS));
            SPI.ctrl().init(|r| r.cpol().set(true));

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), values_written_are([0x0u8, 0x4u8]))
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_write_missing() {
        init_mock(None);

        unsafe {
            SPI.ctrl()
                .init(|r| r.cpha().set(spi::ctrl::Cpha::CPHA_FIRST_SHIFTS));

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), values_written_are([0x0u8, 0x4u8]))
            );
        }
    }

    #[test]
    #[should_panic]
    pub fn fail_too_many_writes() {
        init_mock(None);

        unsafe {
            SPI.ctrl()
                .init(|r| r.cpha().set(spi::ctrl::Cpha::CPHA_FIRST_SHIFTS));
            SPI.ctrl().init(|r| r.cpol().set(true));
            SPI.ctrl().init(|r| r.cpol().set(true));

            given!(
                regmock_rs::logs().iter(),
                require_reg!(SPI.ctrl(), values_written_are([0x0u8, 0x4u8]))
            );
        }
    }
}
