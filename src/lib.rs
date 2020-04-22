//! A hobby game engine

#![warn(clippy::all, missing_docs, unused_qualifications)]
#![deny(
    clippy::correctness,
    missing_copy_implementations,
    missing_debug_implementations
)]

pub use game_engine_core as core;
pub use game_engine_core::{ecs, math};
pub use game_engine_transform as transform;
