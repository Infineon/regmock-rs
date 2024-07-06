use pac::SPI;
use test_pac as pac;

pub mod dut {
    use super::*;

    /// Waits until at least one byte is available in our SPI module and reads it
    pub fn read_byte() -> u8 {
        unsafe {
            // wait until byte is available
            while !SPI.status().read().rxe().get() {}
            // read a single byte from RX fifo
            SPI.rx().read().data().get()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::init_mock;
    use regmock_rs::{given, require_reg, silent, wait_until_polled};
    use std::thread;
    use test_pac::{spi, RegisterValue};

    /// Test that `read_byte` waits for a byte being available.
    /// In our imaginary SPI module that is signified by the status::rxe (RX empty)
    /// being false.
    /// Afterwards we expect that the DUT reads the available data byte
    /// from rx::data.
    #[test]
    fn run_dut_in_thread() {
        let regmock = init_mock(None);

        // run our DUT in a separate thread so that we can still do stuff
        // here and change the apparent content of registers
        //
        // Note: we need to initialize regmock in the new thread, otherwise
        // the thread will panic when the DUT is accessing the registers
        let dut_handle = thread::spawn(|| {
            init_mock(Some(regmock));
            dut::read_byte()
        });

        // use regmock helper to check if the thread is really polling the
        // status register.
        //
        // Note: the polling does not spam the access log with accesses
        // since consecutive read accesses to the same register will just
        // increment a counter.
        wait_until_polled(SPI.status().addr(), 5, None).expect("DUT did not poll");

        // now that we know that our DUT is polling the status register we can
        // mock that data has arrived
        //
        // we have to watch the order in which we prepare the register values
        // since we don't hold a lock inside the silent-closure.
        silent(|| unsafe {
            SPI.rx().init(|r| r.set_raw(0xaf));
            SPI.status()
                .modify(|r| r.set_raw(1 << spi::Status::default().rxe().offset()));
        });

        // since we have set everything up such that our function should
        // return now, wait until the thread exits and verify the return values
        let result = dut_handle.join().unwrap();
        assert_eq!(result, 0xaf);

        // lastly check that the `rx` register was not accessed before the
        // status register
        given!(skip_log, require_reg!(SPI.status(), read_last));
    }
}
