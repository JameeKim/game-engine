//! Transform and hierarchy management

#![warn(clippy::all, missing_docs, unused_qualifications)]
#![deny(
    clippy::correctness,
    missing_copy_implementations,
    missing_debug_implementations
)]

use game_engine_core as core;
use game_engine_core::{ecs, math};

pub mod components;
pub mod systems;
pub mod utils;
