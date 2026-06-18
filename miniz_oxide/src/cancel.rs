//! Optional cooperative cancellation for long-running operations.
//!
//! [`CancelCheck`] is a minimal, dependency-free cancellation hook: anything
//! that can answer "should I stop now?" — an `Arc<AtomicBool>`, a channel, an
//! async cancellation token, a deadline — implements it, directly or via the
//! blanket `Fn() -> bool` impl. [`NeverCancel`] opts out at zero cost.
//!
//! Modelled on the `enough` crate's `Stop` trait
//! (<https://github.com/imazen/enough/blob/main/ZERO-DEP.md>), but deliberately
//! distinct in name and shape so a vendored copy never masquerades as that type.

/// A cheap "should I stop now?" probe, polled periodically by long-running
/// operations.
pub trait CancelCheck: Send + Sync {
    /// Returns `true` to stop as soon as possible. Polled often — keep it cheap.
    fn is_cancelled(&self) -> bool;

    /// Returns `false` if this can never fire, letting callers skip polling
    /// entirely. Defaults to `true`.
    fn may_cancel(&self) -> bool {
        true
    }
}

/// A [`CancelCheck`] that never cancels: a zero-cost opt-out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NeverCancel;

impl CancelCheck for NeverCancel {
    #[inline(always)]
    fn is_cancelled(&self) -> bool {
        false
    }
    #[inline(always)]
    fn may_cancel(&self) -> bool {
        false
    }
}

/// Any `Fn() -> bool` is a `CancelCheck`, so a bare closure works as one.
impl<F: Fn() -> bool + Send + Sync> CancelCheck for F {
    #[inline]
    fn is_cancelled(&self) -> bool {
        self()
    }
}
