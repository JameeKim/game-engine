//! System types

use crate::ecs::schedule::{Runnable, Schedulable};
use crate::ecs::world::World;
use crate::systems::seal::Seal;
use crate::systems::SystemType;

/// System type of systems that can be run in parallel
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Parallel;

impl Seal for Parallel {}

impl SystemType for Parallel {
    type System = Box<dyn Schedulable>;
}

/// System type of systems that should be run on the main thread
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ThreadLocal;

impl Seal for ThreadLocal {}

impl SystemType for ThreadLocal {
    type System = Box<dyn Runnable>;
}

/// System type of systems that should be run on the main thread, represented in `FnMut` form
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ThreadLocalFn;

impl Seal for ThreadLocalFn {}

impl SystemType for ThreadLocalFn {
    type System = Box<dyn FnMut(&mut World)>;
}
