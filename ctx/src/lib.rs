//! ctx --++-- Rust Runtime Contexts

mod err;

pub use err::{Error, Result};
pub use init_tokio;
pub use macros::{main, test};
pub use rayon;
pub use tokio;

use quickcheck::{Arbitrary, Gen};
use std::fmt::{self, Debug};
#[cfg(test)]
mod tests;

#[derive(Copy, Clone)]
pub struct CtxInit {
  // Prevent code outside of this crate from constructing.
  _private: (),
}

/// # Safety
///
/// This function should not be called before the horsemen are ready.
pub const unsafe fn assume_init() -> CtxInit {
  CtxInit { _private: () }
}

impl Debug for CtxInit {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("CtxInit")
  }
}

impl Arbitrary for CtxInit {
  /// # Safety
  ///
  /// This function should not be called before the horsemen are ready.
  fn arbitrary(_: &mut Gen) -> Self {
    unsafe { r#impl::perform_init() }
  }
}

// Not public API. These are used by the attribute macros.
// The non ctx_build version is not performing any Ctx
// initializations.
#[doc(hidden)]
pub mod r#impl {
  pub use super::assume_init as perform_init;
  use crate::CtxInit;

  /// # Safety
  ///
  /// This function should not be called before the horsemen are ready.
  pub const unsafe fn perform_init_with_disable_signals(_: u64) -> CtxInit {
    perform_init()
  }

  pub struct DestroyGuard;

  #[allow(clippy::new_without_default)]
  impl DestroyGuard {
    pub fn new() -> Self {
      DestroyGuard
    }
  }
}
