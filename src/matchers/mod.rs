//! Collection of matchers that can be run
//! against iterators that yield [`RegisterAccess`](crate::utils::RegisterAccess).

use std::usize;

use crate::utils::RegisterAccessType::*;
use crate::utils::*;
use itertools::Diff;
use itertools::Itertools;

mod macros;

/// Error produced by matchers.
pub struct MatchError {
    /// Name of the matcher
    pub name: &'static str,
    /// Reason why the matcher failed.
    pub reason: String,
}

impl MatchError {
    /// Construct new error result
    pub fn error(name: &'static str, reason: String) -> Result<(), MatchError> {
        Err(MatchError { name, reason })
    }
}

/// Match some matcher against a sequence of [`RegisterAccess`]'s.
///
/// The easies way to use the existing matchers is to use them with the
/// [`crate::given`] and `crate::require_*!` macros.
/// Alternatively matcher can be applied by calling [`match()`](#tymethod.match) with a list
/// of register accesses.
///
/// # Examples
/// ```rust,ignore
/// let logs = regmock_rs::get_logs().iter();
/// todo!("implement this example")
/// ```
pub trait LogMatcher<'log, T: IntoIterator<Item = &'log RegisterAccess>> {
    /// Consumes and matches `self` against some sequence of [`RegisterAccess`]'s.
    ///
    fn r#match(self, log: T) -> Result<(), MatchError>;
}

/// Match an *exact* sequence of values written to a specific register.
///
/// # Examples
///
/// Match using [`macro.given!`] and [`macro.require!`]
/// ```rust,ignore
/// use regmock_rs::{given, require};
/// use regmock_rs::utils::*;
/// let logs = regmock_rs::get_logs();
/// given!(
///     &logs.iter(),
///     require!(
///         pac::PERIPHERAL.register().addr(),
///         written_sequence(&vec![0xA, 0xB, 0xC])
///     )
/// );
/// ```
///
/// Apply matcher manually:
/// ```rust,ignore
/// let result = WrittenSequenceMatcher::new(peripheral_address, vec![0xA, 0xB, 0xC])
///     .r#match(crate::get_logs().iter());
/// ```
///
pub struct WrittenSequenceMatcher {
    /// Address of the target register.
    address: usize,
    /// Sequence of values written to the targeted register.
    write_sequence: Vec<u64>,
}

impl WrittenSequenceMatcher {
    const NAME: &'static str = "WrittenSequenceMatcher";
    /// Construct a new [`WrittenSequenceMatcher`] for a given address.
    pub fn new<T, I>(address: usize, write_sequence: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<u64> + Copy,
    {
        Self {
            address,
            write_sequence: write_sequence
                .into_iter()
                .map(move |val| Into::<u64>::into(val))
                .collect::<Vec<u64>>(),
        }
    }
}

impl<'log, T: IntoIterator<Item = &'log RegisterAccess>> LogMatcher<'log, T>
    for WrittenSequenceMatcher
{
    /// Verify that a given sequence of values was written to register.
    ///
    /// Will succeed if the values in [`self.write_sequence`] are written
    /// to [`self.address`] and **no** other writes happened.
    /// Reads are ignored. Writes to other registers are ignored.
    fn r#match(self, log: T) -> Result<(), MatchError> {
        let actual_write_sequence: Vec<u64> = log
            .into_iter()
            .filter(|r| {
                r.addr.as_ref().is_some_and(|addr| addr == &self.address)
                    && r.ty
                        .as_ref()
                        .is_some_and(|ty| ty == &RegisterAccessType::WRITE)
                    && r.after.as_ref().is_some()
            })
            .map(|m| m.after.unwrap())
            .collect();

        if let Some(diff) = itertools::diff_with(
            &actual_write_sequence,
            &self.write_sequence,
            |actual, expected| actual == expected,
        ) {
            MatchError::error(
                Self::NAME,
                match diff {
                    Diff::FirstMismatch(index, mut actual_rem, mut expected_rem) => {
                        format!(
                        "Actual writes to 0x{:08X} differ from expected writes at index:{index} with actual: 0x{:08X} and expected: 0x{:08X}",
                        self.address,
                        expected_rem.next().unwrap(),
                        actual_rem.next().unwrap()
                    )
                    }
                    Diff::Shorter(iter_count, actual_rem) => {
                        format!(
                        "Found more writes to 0x{:08X} than expected. Expected {iter_count} writes.\nValues of the surplus writes are:\n{}",
                        self.address,
                        actual_rem.map(|a| format!("0x{:08X}",a)).collect_vec().join("\n")
                        )
                    }
                    Diff::Longer(iter_count, expected_rem) => {
                        format!(
                        "Expected more writes to 0x{:08X}. Only {iter_count} elements were written.\nValues of the remaining expected writes are:\n{}",
                        self.address,
                        expected_rem.map(|e| format!("0X{:08X}",e)).collect_vec().join("\n")
                        )
                    }
                },
            )
        } else {
            Ok(())
        }
    }
}

/// Verify that register was at least written to once, before other register was written to.
///
/// Will succeed if at least one write to [`target`](#structfield.target) happened before the first
/// write to [`other`](#structfield.other).
/// Any additional writes do not affect the result. Reads are ignored.
pub struct WrittenBeforeMatcher {
    /// Address of the target register.
    pub target: usize,
    /// Address of the other register.
    pub other: usize,
}

impl WrittenBeforeMatcher {
    const NAME: &'static str = "WrittenBeforeMatcher";

    pub fn new(target: usize, other: usize) -> Self {
        Self { target, other }
    }
}

impl<'log, T: IntoIterator<Item = &'log RegisterAccess>> LogMatcher<'log, T>
    for WrittenBeforeMatcher
{
    /// Match [`WrittenBeforeMatcher`] against log of [`RegisterAccess`]'s.
    fn r#match(self, log: T) -> Result<(), MatchError> {
        let mut filtered = log.into_iter().filter(|access| {
            access.ty.as_ref().is_some_and(|ty| *ty == WRITE)
                && access
                    .addr
                    .as_ref()
                    .is_some_and(|addr| addr == &self.target || addr == &self.other)
        });

        match filtered.next() {
            Some(access) => {
                if access.addr.unwrap() == self.other {
                    MatchError::error(Self::NAME, format!("Other register: 0x{:08X} was written to before target register: 0x{:08X}",self.other, self.target))
                } else if filtered
                    .any(|access| access.addr.as_ref().is_some_and(|addr| addr == &self.other))
                {
                    Ok(())
                } else {
                    MatchError::error(Self::NAME, format!("Other register: 0x{:08X} was not written to after write to target register: 0x{:08X}",self.other,self.target))
                }
            }
            None => MatchError::error(
                Self::NAME,
                format!(
                    "No writes to target register: 0x{:08X} or other register: 0x{:08X} recorded",
                    self.target, self.other,
                ),
            ),
        }
    }
}

/// Verify that all writes to register happened before any write to other register.
///
/// Will succeed if **all** (0..n) writes to [`target`](#structfield.target) happened before the
/// first write (0..1) to [`other`](#structfield.other).
/// Other registers and reads are ignored.
pub struct AllWrittenBeforeMatcher {
    /// Register whose writes must **all** happen before writing [`other`](#structfield.other)
    pub target: usize,
    /// Register written to **after** all writes to [`target`](#structfield.target)
    pub other: usize,
}

impl AllWrittenBeforeMatcher {
    const NAME: &'static str = "AllWrittenBeforeMatcher";
    /// Construct new [`AllWrittenBeforeMatcher`]
    pub fn new(target: usize, other: usize) -> Self {
        Self { target, other }
    }
}

impl<'log, T: IntoIterator<Item = &'log RegisterAccess>> LogMatcher<'log, T>
    for AllWrittenBeforeMatcher
{
    /// Match [`AllWrittenBeforeMatcher`] against log of [`RegisterAccess`]'s.
    fn r#match(self, log: T) -> Result<(), MatchError> {
        let filtered = log.into_iter().filter(|access| {
            access.ty.as_ref().is_some_and(|ty| *ty == WRITE)
                && access
                    .addr
                    .as_ref()
                    .is_some_and(|addr| addr == &self.target || addr == &self.other)
        });
        let all_written_before = filtered
            .skip_while(|access| {
                access
                    .addr
                    .as_ref()
                    .is_some_and(|addr| addr == &self.target)
            })
            .all(|access| access.addr.as_ref().is_some_and(|addr| addr == &self.other));

        if all_written_before {
            Ok(())
        } else {
            MatchError::error(Self::NAME,format!("Target register: 0x{:08X} was written to after write to other register: 0x{:08X}",self.target,self.other))
        }
    }
}

/// Verify that register was written **exactly** once.
///
/// Will succeed [`target`](#structfield.target) is written to exactly once.
/// Reads are ignored.
pub struct WrittenOnceMatcher {
    /// Register which must only be written once.
    pub target: usize,
}

impl WrittenOnceMatcher {
    const NAME: &'static str = "WrittenOnceMatcher";
    /// Construct new [`WrittenOnceMatcher`]
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl<'log, T: IntoIterator<Item = &'log RegisterAccess>> LogMatcher<'log, T>
    for WrittenOnceMatcher
{
    /// Match [`WrittenOnceMatcher`] against log of [`RegisterAccess`]'s.
    fn r#match(self, log: T) -> Result<(), MatchError> {
        match log
            .into_iter()
            .filter(|access| {
                access.ty.as_ref().is_some_and(|ty| *ty == WRITE)
                    && access
                        .addr
                        .as_ref()
                        .is_some_and(|addr| addr == &self.target)
            })
            .count()
        {
            1 => Ok(()),
            count => MatchError::error(
                Self::NAME,
                format!(
                    "Register: 0x{:08X} was written to {} times",
                    self.target, count
                ),
            ),
        }
    }
}

/// Verify that register was **not**.
///
/// Will succeed [`self.target`](#structfield.target) is never written to.
/// Reads are ignored.
pub struct NotWrittenMatcher {
    /// Register which must not be written to.
    pub target: usize,
}

impl NotWrittenMatcher {
    const NAME: &'static str = "NotWrittenMatcher";
    /// Construct new [`NotWrittenMatcher`]
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl<'log, T: IntoIterator<Item = &'log RegisterAccess>> LogMatcher<'log, T> for NotWrittenMatcher {
    /// Match [`NotWrittenMatcher`] against log of [`RegisterAccess`]'s.
    fn r#match(self, log: T) -> Result<(), MatchError> {
        match log
            .into_iter()
            .filter(|access| {
                access.ty.as_ref().is_some_and(|ty| *ty == WRITE)
                    && access
                        .addr
                        .as_ref()
                        .is_some_and(|addr| addr == &self.target)
            })
            .count()
        {
            0 => Ok(()),
            count => MatchError::error(
                Self::NAME,
                format!(
                    "Register: 0x{:08X} was written to {} times",
                    self.target, count
                ),
            ),
        }
    }
}

/// Verify that last access to a register was a read.
///
/// Will succeed [`target`](#structfield.target) was read, and no access happened afterwards.
/// Other registers are ignored.
pub struct ReadLastMatcher {
    /// Register which must not be written to.
    pub target: usize,
}

impl ReadLastMatcher {
    const NAME: &'static str = "ReadLastMatcher";
    /// Construct new [`ReadLastMatcher`]
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl<'log, T: IntoIterator<Item = &'log RegisterAccess>> LogMatcher<'log, T> for ReadLastMatcher {
    /// Match [`ReadLastMatcher`] against log of [`RegisterAccess`]'s.
    fn r#match(self, log: T) -> Result<(), MatchError> {
        match log
            .into_iter()
            .filter(|access| {
                access
                    .addr
                    .as_ref()
                    .is_some_and(|addr| addr == &self.target)
            })
            .last()
        {
            Some(access) => {
                if access.ty.as_ref().is_some_and(|ty| *ty == READ) {
                    Ok(())
                } else {
                    MatchError::error(
                        Self::NAME,
                        format!(
                            "Last access to register: 0x{:08X} was: {:?}",
                            self.target, access.ty
                        ),
                    )
                }
            }
            None => MatchError::error(
                Self::NAME,
                format!("Register: 0x{:08X} was not accessed.", self.target),
            ),
        }
    }
}

/// Verify a sequence of [`RegisterAccess`](crate::utils::RegisterAccess)'s
/// happened.
///
/// Will succeed if [`seq`](#structfield.seq) yields equal [`RegisterAccess`](crate::utils::RegisterAccess)'s
/// as the provided log iterator. Fails is the iterators are not pairwise equal.
pub struct LogSequenceMatcher<'seq, SEQ>
where
    SEQ: IntoIterator<Item = &'seq RegisterAccess>,
{
    pub seq: SEQ,
}

impl<'seq, SEQ> LogSequenceMatcher<'seq, SEQ>
where
    SEQ: IntoIterator<Item = &'seq RegisterAccess>,
{
    const NAME: &'static str = "LogSequenceMatcher";
    /// Construct new [`LogSequenceMatcher`]
    pub fn new(seq: SEQ) -> Self {
        Self { seq }
    }
}

impl<'seq, 'log, SEQ, T> LogMatcher<'log, T> for LogSequenceMatcher<'seq, SEQ>
where
    SEQ: IntoIterator<Item = &'seq RegisterAccess>,
    T: IntoIterator<Item = &'log RegisterAccess>,
{
    /// Match [`LogSequenceMatcher`] against log of [`RegisterAccess`]'s.
    fn r#match(self, log: T) -> Result<(), MatchError> {
        if let Some(diff) =
            itertools::diff_with(log, self.seq, |actual, expected| expected.eq(actual))
        {
            MatchError::error(
                Self::NAME,
                match diff {
                    Diff::FirstMismatch(index, mut actual_rem, mut expected_rem) => {
                        format!(
                            "Actual register accesses differ from expected accesses at index:{index} with\nexpected: {:?}\nactual:   {:?}",
                            expected_rem.next().unwrap(),
                            actual_rem.next().unwrap()
                        )
                    }
                    Diff::Shorter(iter_count, actual_rem) => {
                        format!(
                        "Found more accesses than expected. Expected {iter_count} writes.\nValues of the surplus accesses are:\n{}",
                        actual_rem.map(|a| format!("{:?}",a)).collect_vec().join("\n")
                        )
                    }
                    Diff::Longer(iter_count, expected_rem) => {
                        format!(
                        "Expected more accesse. Only {iter_count} accesses were recorded.\nValues of the remaining expected accesses are:\n{}",
                        expected_rem.map(|e| format!("{:?}",e)).collect_vec().join("\n")
                        )
                    }
                },
            )
        } else {
            Ok(())
        }
    }
}
