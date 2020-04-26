//! Core functionality to be used in the engine

#![warn(clippy::all, missing_docs, unused_qualifications)]
#![deny(
    clippy::correctness,
    missing_copy_implementations,
    missing_debug_implementations
)]

pub mod frame_rate;
mod query_all;
pub mod sized_queue;
pub mod systems;
pub mod time;
pub mod tuple_list;

pub use self::query_all::All;
pub use legion as ecs;
pub use nalgebra as math;
pub use shrev as event_channel;

use game_engine_derive as derive;
