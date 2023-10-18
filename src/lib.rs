//! An attribute to create an atomic wrapper around a C-style enum.
//!
//! # Example
//!
//! ```rust
//! # use atomic_enum::atomic_enum;
//! # use std::sync::atomic::Ordering;
//! #[atomic_enum]
//! #[derive(Clone, Copy, Debug, PartialEq)]
//! enum CatState {
//!     Dead = 0,
//!     BothDeadAndAlive,
//!     Alive,
//! }
//!
//! let state = AtomicCatState::new(CatState::Dead);
//! state.store(CatState::Alive, Ordering::Relaxed);
//! assert_eq!(state.load(Ordering::Relaxed), CatState::Alive);
//! ```

#![no_std]

pub use portable_atomic;
pub use portable_atomic_enum_macros::atomic_enum;
