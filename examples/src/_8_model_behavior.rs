use pac::SPI;
use test_pac as pac;

pub mod dut {
    use super::*;

    /// A bog standard SPI receive function, waiting until
    /// all data is available
    pub fn spi_recv(buffer: &mut [u8]) {
        for b in buffer {
            unsafe {
                // wait until a byte is available
                while SPI.status().read().rxe().get() {}

                *b = SPI.rx().read().data().get();
            }
        }
    }
}

/// A module for our our modelling needs
#[cfg(test)]
pub mod model {
    use super::*;
    use crate::common::init_mock;
    use regmock_rs::utils::Regmock;
    use std::{
        cell::RefCell,
        sync::{Arc, Mutex, OnceLock},
        thread::LocalKey,
    };

    /// A very simple model of an SPI module
    ///
    /// The idea here is to force the DUT to poll correctly
    /// by making data available every few times the `status`
    /// register is read.
    ///
    /// What the model implements and how complete/complex it is can be
    /// decided depending on the circumstance. A wide range of possibilities
    /// are there:
    ///
    /// - starting from a simple implementation like this one
    /// - a more complex model that e.g. models interaction between multiple registers
    /// - opening a socket and exposing a virtual SPI interface
    /// - to running hardware-in-the loop tests be forwarding reads/writes to the real HW
    #[derive(Debug)]
    pub struct SpiModel {
        // how often the `status` register needs to be read before data becomes available
        pub default_delay: u32,
        // a counter just as protection against deadlocks - not necessary, but nice
        pub poll_cntr: u32,

        // a vector of the bytes that are being received
        pub to_receive: Vec<u8>,
        // how often the DUT needs to read the status register until
        // the next byte becomes available
        pub cycles_until_rx_ne: u32,
    }

    // Creation of an simple, empty state
    impl Default for SpiModel {
        fn default() -> Self {
            Self {
                default_delay: 3,
                poll_cntr: 0,
                to_receive: vec![],
                cycles_until_rx_ne: 0,
            }
        }
    }

    // Since rust testcases run multi-threaded, our model must be a thread-local
    //
    // Note: if we start a thread in our testcase, we only need to initialize
    // the regmock thread-local on that thread, not the model. This is the
    // case because we never directly access the model thread-local, only through
    // boxed callback functions that we register
    thread_local! {
        #[allow(clippy::missing_const_for_thread_local)]
        pub static SPI_MODEL: OnceLock<RefCell<SpiModel>> = const { OnceLock::new() };
    }

    impl SpiModel {
        /// Register all the read callback functions with our regmock instance
        pub fn register(
            regmock: Arc<Mutex<Regmock>>,
        ) -> &'static LocalKey<OnceLock<RefCell<SpiModel>>> {
            let mut regmock = regmock.lock().unwrap();
            SPI_MODEL.with(|sm| sm.set(RefCell::new(SpiModel::default())).unwrap());

            regmock.read_fn.insert(
                SPI.status().addr(),
                Box::new(|_, _| {
                    SPI_MODEL.with(|sm| sm.get().unwrap().borrow_mut().read_status()) as u64
                }),
            );
            regmock.read_fn.insert(
                SPI.rx().addr(),
                Box::new(|_, _| {
                    SPI_MODEL.with(|sm| sm.get().unwrap().borrow_mut().read_rx()) as u64
                }),
            );
            &SPI_MODEL
        }

        // the mock function for the `status` register
        //
        // Note: we have a lock on regmock when executing this function, so we don't
        // watch timing, etc.
        pub fn read_status(&mut self) -> u32 {
            // do a quick check for a stuck DUT so that we don't have
            // to wait so long if we made a mistake
            if self.poll_cntr > 1000 {
                panic!("polling in an endless loop")
            }
            self.poll_cntr += 1;

            // decrement our delay counter
            self.cycles_until_rx_ne = self.cycles_until_rx_ne.saturating_sub(1);

            // if the counter is zero, indicate that we have data available
            let (rxe, rx_fill) = if self.cycles_until_rx_ne == 0 && !self.to_receive.is_empty() {
                (0, 1 << 16)
            } else {
                (1 << 2, 0)
            };

            // log the read and return
            let status = (rxe | rx_fill) as u32;
            log::debug!("R status {:08x}", status);
            status
        }

        pub fn read_rx(&mut self) -> u32 {
            // reset the polling counter since we do something else then polling
            self.poll_cntr = 0;

            // if we are indicating that data is available, provide the next word
            if self.cycles_until_rx_ne == 0 && !self.to_receive.is_empty() {
                let byte = *self.to_receive.first().unwrap();
                self.to_receive.remove(0);

                // reset the delay counter
                self.cycles_until_rx_ne = self.default_delay;

                // log the read
                log::debug!("R rx {:02x}", byte);
                byte as u32
            } else {
                panic!("RX register read, even though no data is ready")
            }
        }
    }

    // This wrapper is just a simple thing to make accessing the model
    // and setting data, etc. much simpler
    pub struct ModelWrapper {
        model: &'static LocalKey<OnceLock<RefCell<SpiModel>>>,
    }

    impl ModelWrapper {
        // helper to get the thread-local model
        pub fn with<F>(&self, f: F)
        where
            F: FnOnce(&mut SpiModel),
        {
            self.model.with(|m| f(&mut m.get().unwrap().borrow_mut()));
        }

        // prepare data to be "received"
        pub fn set_receive(&self, b: Vec<u8>) {
            self.with(|sm| sm.to_receive = b);
        }
    }

    // the init function that we'll call in case we want to use the model
    // in a test - also does the regmock init if we want it to for convenience
    pub fn init_radio_model(
        regmock: Option<Arc<Mutex<Regmock>>>,
    ) -> (Arc<Mutex<Regmock>>, ModelWrapper) {
        let regmock = regmock.unwrap_or(init_mock(None));
        let spi_model = SpiModel::register(regmock.clone());

        (regmock, ModelWrapper { model: spi_model })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::init_radio_model;

    #[test]
    fn read_data() {
        // initialize and prepare data
        let (_regmock, model) = init_radio_model(None);
        model.set_receive(vec![0x11, 0x22, 0x33]);

        // call the DUT
        let mut buffer = [0; 3];
        dut::spi_recv(&mut buffer);

        // check the DUT received correctly
        assert_eq!(buffer, [0x11, 0x22, 0x33]);
    }
}
