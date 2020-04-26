//! Add window support using [`winit`] crate
//!
//! [`winit`]: ../winit/index.html

#![warn(clippy::all, missing_docs, unused_qualifications)]
#![deny(
    clippy::correctness,
    missing_copy_implementations,
    missing_debug_implementations
)]

mod bundle;
mod resources;
mod systems;

pub use crate::bundle::WindowBundle;
pub use crate::resources::WindowSize;
pub use crate::systems::*;
pub use winit as wm;

use game_engine_core::{self as core, ecs, event_channel};
