/// Macro for matching [`LogMatcher`](crate::matchers::LogMatcher) against
/// iterators of [`RegisterAccess`](crate::utils::RegisterAccess) in a
/// "more prose like" manner.
///
/// # Panics
/// The macro panics, if applying the `LogMatcher` to the [`RegisterAccess`](crate::utils::RegisterAccess)
/// iterator fails.
///
/// # Parameters
///
/// - access list, pass one of:
///   - arbitrary iterater of register accesses, like `regmock_rs::log().iter()`, can be filtered etc.
///   - `full_log` to match against complete log with duplicate accesses unrolled
///   - `skip_log` to match against log with read accesses compressed (single entry for polling)
///
/// # Examples
///
/// Best used together with the `require_reg!` and `require_seq!` macros to construct the
/// [`LogMatcher`](crate::matchers::LogMatcher)s.
///
/// Check that register was written to only once.
/// ```rust,ignore
/// given!(
///     full_log,
///     require_reg!(
///         pac::PERIPHERAL.register().unwrap(),
///         written_once
///     )
/// );
/// ```
///
/// Check that sequence of values was written to register.
/// ```rust,ignore
/// let log_iter = regmock_rs::get_logs().iter();
/// given!(
///     log_iter,
///     require_reg!(
///         pac::PERIPHERAL.register(),
///         values_written_are(vec![0xAAAu32, 0xAABu32, 0xAACu32])
///     )
/// );
/// ```
#[macro_export]
macro_rules! given {
    (full_log, $matcher: expr) => {{
        use regmock_rs::matchers::*;
        let mut m = $matcher;
        match m.r#match(regmock_rs::logs().iter_full()) {
            Ok(_) => ..,
            Err(me) => {
                panic!("\nFailed to match {} because:\n'{}'", me.name, me.reason);
            }
        }
    }};
    (skip_log, $matcher: expr) => {{
        use regmock_rs::matchers::*;
        let mut m = $matcher;
        match m.r#match(regmock_rs::logs().iter()) {
            Ok(_) => ..,
            Err(me) => {
                panic!("\nFailed to match {} because:\n'{}'", me.name, me.reason);
            }
        }
    }};
    ($log: expr, $matcher: expr) => {{
        use regmock_rs::matchers::*;
        // let log = $log;
        let mut m = $matcher;
        match m.r#match($log) {
            Ok(_) => ..,
            Err(me) => {
                panic!("\nFailed to match {} because:\n'{}'", me.name, me.reason);
            }
        }
    }};
}

/// Macro for constructing register specific [`LogMatcher`](crate::matchers::LogMatcher)
/// structs in a "more prose like" manner.
///
/// All macro variants require a `$target:expr` that evaluates to a
/// register instance. `.addr()` will be called on it. The result of this
/// expression is used as the `target` register for the constructed
/// [`LogMatcher`](crate::matchers::LogMatcher).
///
/// Depending on the variants, the macro requires other paramters
/// to initialize specific [`LogMatcher`](crate::matchers::LogMatcher).
///
/// # Variants
///
/// ## `read_last`
/// Initializes a [`ReadLastMatcher`](crate::matchers::ReadLastMatcher)
/// with the value of `$target:expr` as target address.
///
/// ```rust,ignore
/// require_reg!(pac::PERIPHERAL.register(), read_last)
/// ```
///
/// ## `not_written`
/// Initializes a [`NotWrittenMatcher`](crate::matchers::NotWrittenMatcher)
/// with the value of `$target:expr` as target address.
///
/// ```rust,ignore
/// require_reg!(pac::PERIPHERAL.register(), not_written)
/// ```
///
/// ## `written_once`
/// Initializes a [`WrittenOnceMatcher`](crate::matchers::WrittenOnceMatcher)
/// with the value of `$target:expr` as target address.
///
/// ```rust,ignore
/// require_reg!(pac::PERIPHERAL.register(), written_once)
/// ```
///
/// ## `all_written_before`
/// Initializes a [`AllWritesBeforeWritesTo`](crate::matchers::AllWritesBeforeWritesTo)
/// with the value of `$target:expr` as the target register and
/// `$other_address:expr` as the other register.
///
/// ```rust,ignore
/// require_reg!(pac::PERIPHERAL.register(), all_written_before(pac::PERIPHERAL.other_register()))
/// ```
///
/// ## `written_before`
/// Initializes a [`WrittenToBeforeWriteTo`](crate::matchers::WrittenToBeforeWriteTo)
/// with the value of `$target:expr` as the target register and
/// `$other_address:expr` as the other register.
///
/// ```rust,ignore
/// require_reg!(pac::PERIPHERAL.register(), written_before(pac::PERIPHERAL.other_register()))
/// ```
///
/// ## `values_written_are`
/// Initializes a [`ValuesWrittenAre`](crate::matchers::ValuesWrittenAre)
/// with the value of `$target:expr` as the target register and
/// `$sequence:expr` as the sequence of written values.
/// The values passed will be converted to `u64`s and panic if not possible.
///
/// ```rust,ignore
/// require_reg!(pac::PERIPHERAL.register(), values_written_are([0x11, 0x22, 0x44]))
/// ```
#[macro_export]
macro_rules! require_reg {
    ($target:expr, read_last) => {
        regmock_rs::matchers::ReadLastMatcher::new($target.addr())
    };
    ($target:expr, not_written) => {
        regmock_rs::matchers::NotWrittenMatcher::new($target.addr())
    };
    ($target:expr, written_once) => {
        regmock_rs::matchers::WrittenOnceMatcher::new($target.addr())
    };
    ($target:expr, all_writes_before_writes_to($other_address:expr)) => {
        regmock_rs::matchers::AllWritesBeforeWritesTo::new($target.addr(), $other_address.addr())
    };
    ($target:expr, written_to_before_write_to($other_address:expr)) => {
        regmock_rs::matchers::WrittenToBeforeWriteTo::new($target.addr(), $other_address.addr())
    };
    ($target:expr, values_written_are($sequence:expr)) => {
        regmock_rs::matchers::ValuesWrittenAre::new(
            $target.addr(),
            $sequence.into_iter().map(|x| {
                u64::try_from(x)
                    .inspect_err(|x| {
                        panic!("Could not convert expected write value to u64 because: {x}")
                    })
                    .unwrap()
            }),
        )
    };
}

/// Macro for constructing a [`LogSequenceMatcher`](crate::matchers::LogSequenceMatcher)
/// struct in a "more prose like" manner.
/// Works together well with e.g. the [`crate::utils::access_gen::read_value`] and
/// [`crate::utils::access_gen::write_value`] helpers (and cousins).
///
/// # Example
///
/// ```rust,ignore
/// let r0 = read_value(pac::PERIPHERAL.register().addr(), 0);
/// let w0 = write_value(pac::PERIPHERAL.register().addr(), 0x1234);
/// given!(full_log, require_seq!(vec![&r0, &r1, &w0]));
/// ```
#[macro_export]
macro_rules! require_seq {
    ($seq:expr) => {
        regmock_rs::matchers::LogSequenceMatcher::new($seq)
    };
}
