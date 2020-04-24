//! The main application for the engine

#![warn(clippy::all, missing_docs, unused_qualifications)]
#![deny(
    clippy::correctness,
    missing_copy_implementations,
    missing_debug_implementations
)]

use game_engine_core as core;
#[allow(unused_imports)]
use game_engine_core::{ecs, math};

mod app;
mod log;

pub use self::app::{Application, ApplicationBuilder};
pub use self::log::init_logger;
