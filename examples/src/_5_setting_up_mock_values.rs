use pac::GPIO;
use test_pac as pac;

pub mod dut {
    use super::*;

    pub fn is_output_high() -> bool {
        unsafe { GPIO.out().read().gpio0().get() }
    }

    pub fn is_input_high() -> bool {
        unsafe { GPIO.r#in().read().gpio0().get() }
    }
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use regmock_rs::silent;
    use test_pac::{gpio, RegisterValue};

    use super::*;
    use crate::common::init_mock;

    #[test]
    fn single_value() {
        init_mock(None);

        // prepare the register with a known value that will be read by the FUT
        //
        // when setting up values we recommend to do this using the `silent` helper
        // as this will prevent the access to be recorded in the log
        // (in this specific test it wouldn't make a difference though)
        silent(|| unsafe { GPIO.out().init(|r| r.set_raw(0x11)) });

        // testing the DUT
        assert_eq!(dut::is_output_high(), true);
    }

    #[allow(unused_imports)] // incorrectly flags `insanely_unsafe` import below
    #[test]
    fn single_value_in_ro_register() {
        // we import the impls from the `insanely_unsafe` crate - this
        // will allow us to "write" to a read-only bitfield, effectively
        // allowing us to prepare a value for a test-case
        use pac::tracing::insanely_unsafe::*;

        init_mock(None);

        // since `in` is a read-only register, the normal approach of using `init` or `modify`
        // would not work as those functions would not be available
        // `write_read_only` is the workaround for this
        silent(|| unsafe { GPIO.r#in().write_read_only(gpio::In::new(0x11u32)) });

        // testing the DUT
        assert_eq!(dut::is_input_high(), true);
    }
}
