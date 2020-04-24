//! Core functionality to be used in the engine

#![warn(clippy::all, missing_docs, unused_qualifications)]
#![deny(
    clippy::correctness,
    missing_copy_implementations,
    missing_debug_implementations
)]

mod builder_ext;
pub mod frame_rate;
mod query_all;
pub mod schedule_wrapper;
pub mod sized_queue;
pub mod time;

pub use self::builder_ext::BuilderExt;
pub use self::query_all::All;
pub use legion as ecs;
pub use nalgebra as math;
