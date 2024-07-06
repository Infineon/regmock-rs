use pac::SPI;
use test_pac as pac;

pub mod dut {
    use super::*;

    pub fn spi_recv(buffer: &mut [u8]) {
        for b in buffer.iter_mut() {
            unsafe { *b = SPI.rx().read().data().get() }
        }
    }
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::common::init_mock;

    #[test]
    fn custom_read_function() {
        let regmock = init_mock(None);

        // prepare the values that we want to be "read" from the
        // register outside, so they will be moved into the closure
        // and we can slowly drain the vec
        //
        // Note: here we are fine with just a VecDeque which is moved
        // into the closure. If you need to access the variable here
        // later in the test then you'll have to wrap it in a
        // `Arc::new(Mutex::new(...))`
        let mut to_send = VecDeque::from(vec![0x11, 0x22, 0x33]);
        // register mock function in scope so that lock will be
        // released again - otherwise we'd dead-lock regmock
        {
            let mut regmock = regmock.lock().unwrap();

            regmock.read_fn.insert(
                SPI.rx().addr(),
                Box::new(move |_address, _mask| {
                    to_send.pop_front().expect("too many register reads") as u64
                }),
            );
        }

        // call the DUT
        let mut buffer = [0; 3];
        dut::spi_recv(&mut buffer);

        // assert that the correct values were put into
        // our receive buffer
        assert_eq!(buffer, [0x11, 0x22, 0x33]);
    }
}
