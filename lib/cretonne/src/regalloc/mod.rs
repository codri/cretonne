//! Register allocation.
//!
//! This module contains data structures and algorithms used for register allocation.

pub mod liverange;
pub mod liveness;
pub mod allocatable_set;
pub mod live_value_tracker;
pub mod coloring;

mod affinity;
mod context;
mod diversion;
mod pressure;
mod solver;

pub use self::allocatable_set::AllocatableSet;
pub use self::context::Context;
pub use self::diversion::RegDiversions;
