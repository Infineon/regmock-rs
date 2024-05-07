/// Macro for matching [`LogMatcher`](crate::matchers::LogMatcher) against
/// iterators of [`RegisterAccess`](crate::utils::RegisterAccess) in a
/// "more prose like" manner.
///
/// # Panics
/// The macro panics, if applying the `LogMatcher` to the [`RegisterAccess`](crate::utils::RegisterAccess)
/// iterator fails.
///
/// # Examples
///
/// Best used together with `require_*!` macros to construct the
/// [`LogMatcher`](crate::matchers::LogMatcher)s.
///
/// Check that register was written to only once.
/// ```rust,ignore
/// let log_iter = regmock_rs::get_logs().iter();
/// given!(
///     log_iter,
///     require_reg!(
///         pac::PERIPHERAL.register().addr(),
///         written_once
///     )
/// );
/// ```
///
/// Check that sequnece of values was written to register.
/// ```rust,ignore
/// let log_iter = regmock_rs::get_logs().iter();
/// given!(
///     log_iter,
///     require_reg!(
///         pac::PERIPHERAL.register().addr(),
///         written_sequence(vec![0xAAAu32, 0xAABu32, 0xAACu32])
///     )
/// );
/// ```

#[macro_export]
macro_rules! given {
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
/// All macro variants require a `$target:expr` that evaluates to an
/// address of a device register in for of a `u64`. The result of this
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
/// ## `not_written`
/// Initializes a [`NotWrittenMatcher`](crate::matchers::NotWrittenMatcher)
/// with the value of `$target:expr` as target address.
///
/// ## `written_once`
/// Initializes a [`WrittenOnceMatcher`](crate::matchers::WrittenOnceMatcher)
/// with the value of `$target:expr` as target address.
///
/// ## `all_written_before`
/// Initializes a [`AllWrittenBeforeMatcher`](crate::matchers::AllWrittenBeforeMatcher)
/// with the value of `$target:expr` as target address and the value of
/// `$other_address:expr` as the other address.
///
/// ## `written_before`
/// Initializes a [`WrittenBeforeMatcher`](crate::matchers::WrittenBeforeMatcher)
/// with the value of `$target:expr` as target address and the value of
/// `$other_address:expr` as the other address.
///
/// ## `written_sequence`
/// Initializes a [`WrittenSequenceMatcher`](crate::matchers::WrittenSequenceMatcher)
/// with the value of `$target:expr` as target address and the value of
/// `$sequence:expr` as the sequence of written values.
#[macro_export]
macro_rules! require_reg {
    ($target:expr, read_last) => {
        regmock_rs::matchers::ReadLastMatcher::new($target)
    };
    ($target:expr, not_written) => {
        regmock_rs::matchers::NotWrittenMatcher::new($target)
    };
    ($target:expr, written_once) => {
        regmock_rs::matchers::WrittenOnceMatcher::new($target)
    };
    ($target:expr, all_written_before($other_address:expr)) => {
        regmock_rs::matchers::AllWrittenBeforeMatcher::new($target, $other_address)
    };
    ($target:expr, written_before($other_address:expr)) => {
        regmock_rs::matchers::WrittenBeforeMatcher::new($target, $other_address)
    };
    ($target:expr, written_sequence($sequence:expr)) => {
        regmock_rs::matchers::WrittenSequenceMatcher::new($target, $sequence)
    };
}

/// Macro for constructing a [`LogSequenceMatcher`](crate::matchers::LogSequenceMatcher)
/// struct in a "more prose like" manner.
///
/// # Variants and Examples
/// Construct [`LogSequenceMatcher`](crate::matchers::LogSequenceMatcher).
#[macro_export]
macro_rules! require_seq {
    ($seq:expr) => {
        regmock_rs::matchers::LogSequenceMatcher::new($seq)
    };
}
