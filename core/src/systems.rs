//! Utility types for building systems and schedule

pub mod entry;
mod order;
pub mod types;

pub use self::order::{SystemOrder, SystemPriority};
pub use crate::derive::SystemDesc;

use self::entry::*;
use self::types::{Parallel, ThreadLocal, ThreadLocalFn};
use crate::ecs::schedule::{Runnable, Schedulable, Schedule};
use crate::ecs::world::World;
use std::collections::BTreeMap;
use std::fmt;

mod seal {
    /// Trait for sealing the `SystemType` trait
    pub trait Seal {}
}

/// Trait for types that represent types of systems: either thread-local or non-thread-local
pub trait SystemType: self::seal::Seal {
    /// The real type of the built system: `Box<dyn Runnable>`, `Box<dyn Schedulable>`, or
    /// `Box<dyn FnMut(&mut World)>`
    ///
    /// `Box<dyn Runnable>` and `Box<dyn FnMut(&mut World)>` are thread-local systems, while
    /// `Box<dyn Schedulable`> is a system that can run on other threads.
    type System: Into<ScheduleEntry> + Sized;
}

/// A descriptor for building a system
///
/// This trait is meant to be implemented by structs that wrap the system building process to add
/// any needed resources to the world for the system it is building.
pub trait SystemDesc {
    /// The type of the system: whether the system is thread-local or not
    type SystemType: SystemType;

    /// Build the system, adding any needed resources if any
    fn build(self, world: &mut World) -> <Self::SystemType as SystemType>::System;
}

/// A bundle of systems that add multiple related systems
pub trait SystemBundle {
    /// Build the systems
    ///
    /// This method should insert any needed resources.
    fn build_systems(self, world: &mut World, builder: &mut ScheduleBuilder<'_>);
}

/// Builder for [`Schedule`]
///
/// # Lifetime Parameters
///
/// - `a`: The lifetime of the closures, system descriptors, and system bundles passed into this
///    builder
///
/// [`Schedule`]: ../../legion/schedule/struct.Schedule.html
#[derive(Default)]
pub struct ScheduleBuilder<'a> {
    desc: Vec<Box<dyn ScheduleEntryDesc + 'a>>,
}

impl<'a> ScheduleBuilder<'a> {
    /// Create a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a system that is created from the given function and will be added to the given order
    pub fn add_system_create_fn<F>(&mut self, order: SystemOrder, f: F)
    where
        F: FnOnce(&mut World) -> Box<dyn Schedulable> + 'a,
    {
        self.add_general_system_create_fn::<Parallel, F>(order, f);
    }

    /// Add a system that is created from the given function and will be added to the given order
    pub fn with_system_create_fn<F>(mut self, order: SystemOrder, f: F) -> Self
    where
        F: FnOnce(&mut World) -> Box<dyn Schedulable> + 'a,
    {
        self.add_system_create_fn(order, f);
        self
    }

    /// Add a thread-local system that is created from the given function and will be added to the
    /// given order
    pub fn add_thread_local_system_create_fn<F>(&mut self, order: SystemOrder, f: F)
    where
        F: FnOnce(&mut World) -> Box<dyn Runnable> + 'a,
    {
        self.add_general_system_create_fn::<ThreadLocal, F>(order, f);
    }

    /// Add a thread-local system that is created from the given function and will be added to the
    /// given order
    pub fn with_thread_local_system_create_fn<F>(mut self, order: SystemOrder, f: F) -> Self
    where
        F: FnOnce(&mut World) -> Box<dyn Runnable> + 'a,
    {
        self.add_thread_local_system_create_fn(order, f);
        self
    }

    /// Add a thread-local closure that is created from the given function and will be added to the
    /// given order
    pub fn add_thread_local_fn_create_fn<F>(&mut self, order: SystemOrder, f: F)
    where
        F: FnOnce(&mut World) -> Box<dyn FnMut(&mut World)> + 'a,
    {
        self.add_general_system_create_fn::<ThreadLocalFn, F>(order, f);
    }

    /// Add a thread-local closure that is created from the given function and will be added to the
    /// given order
    pub fn with_thread_local_fn_create_fn<F>(mut self, order: SystemOrder, f: F) -> Self
    where
        F: FnOnce(&mut World) -> Box<dyn FnMut(&mut World)> + 'a,
    {
        self.add_thread_local_fn_create_fn(order, f);
        self
    }

    /// Add a system with the given type, created from the given function, and added to the given
    /// execution order
    pub fn add_general_system_create_fn<Ty, F>(&mut self, order: SystemOrder, f: F)
    where
        Ty: SystemType + 'a,
        F: FnOnce(&mut World) -> Ty::System + 'a,
    {
        self.add_entry_desc(SystemCreateFn::<Ty, F>::new(order, f));
    }

    /// Add a system with the given type, created from the given function, and added to the given
    /// execution order
    pub fn with_general_system_create_fn<Ty, F>(mut self, order: SystemOrder, f: F) -> Self
    where
        Ty: SystemType + 'a,
        F: FnOnce(&mut World) -> Ty::System + 'a,
    {
        self.add_general_system_create_fn::<Ty, F>(order, f);
        self
    }

    /// Add a system that is created from the given descriptor and will be added to the given order
    pub fn add_system_create_desc<D>(&mut self, order: SystemOrder, desc: D)
    where
        D: SystemDesc + 'a,
    {
        self.add_entry_desc(SystemCreateDesc::new(order, desc));
    }

    /// Add a system that is created from the given descriptor and will be added to the given order
    pub fn with_system_create_desc<D>(mut self, order: SystemOrder, desc: D) -> Self
    where
        D: SystemDesc + 'a,
    {
        self.add_system_create_desc(order, desc);
        self
    }

    /// Add a system bundle
    pub fn add_system_bundle<B>(&mut self, bundle: B)
    where
        B: SystemBundle + 'a,
    {
        self.add_entry_desc(SystemBundleEntry(bundle));
    }

    /// Add a system bundle
    pub fn with_system_bundle<B>(mut self, bundle: B) -> Self
    where
        B: SystemBundle + 'a,
    {
        self.add_system_bundle(bundle);
        self
    }

    /// Add a system bundle in a form of a closure
    pub fn add_system_bundle_fn<F>(&mut self, f: F)
    where
        F: FnOnce(&mut World, &mut ScheduleBuilder) + 'a,
    {
        self.add_entry_desc(SystemBundleFn::new(f));
    }

    /// Add a system bundle in a form of a closure
    pub fn with_system_bundle_fn<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut World, &mut ScheduleBuilder) + 'a,
    {
        self.add_system_bundle_fn(f);
        self
    }

    /// Add flush command at the given order
    pub fn add_flush(&mut self, order: SystemOrder) {
        self.add_entry_desc(CmdBufFlush(order))
    }

    /// Add flush command at the given order
    pub fn with_flush(mut self, order: SystemOrder) -> Self {
        self.add_flush(order);
        self
    }

    /// Add the given schedule entry descriptor
    pub fn add_entry_desc<D>(&mut self, desc: D)
    where
        D: ScheduleEntryDesc + 'a,
    {
        self.desc.push(Box::new(desc));
    }

    /// Add the given schedule entry descriptor
    pub fn with_entry_desc<D>(mut self, desc: D) -> Self
    where
        D: ScheduleEntryDesc + 'a,
    {
        self.add_entry_desc(desc);
        self
    }

    /// Build the [`Schedule`]
    ///
    /// [`Schedule`]: ../../legion/schedule/struct.Schedule.html
    pub fn build(self, world: &mut World) -> Schedule {
        self.into_data(world).into_schedule()
    }

    /// Convert this builder into static data
    pub fn into_data(self, world: &mut World) -> ScheduleData {
        let mut data = ScheduleData::new();

        for entry_desc in self.desc {
            entry_desc.insert_entry(world, &mut data);
        }

        data
    }
}

impl<'a> fmt::Debug for ScheduleBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[derive(Debug)]
        struct Vec {
            len: usize,
        }
        impl Vec {
            fn new(len: usize) -> Self {
                Vec { len }
            }
        }

        f.debug_struct("ScheduleBuilder")
            .field("desc", &Vec::new(self.desc.len()))
            .finish()
    }
}

/// Static data of the order the systems will execute
#[derive(Debug, Default)]
pub struct ScheduleData(BTreeMap<SystemOrder, Vec<ScheduleEntry>>);

impl ScheduleData {
    /// Create a new instance with empty data
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert this data into [`Schedule`] that can execute the systems
    ///
    /// [`Schedule`]: ../../legion/schedule/struct.Schedule.html
    pub fn into_schedule(self) -> Schedule {
        self.0
            .into_iter()
            .fold(Schedule::builder(), |builder, (_, entries)| {
                entries.into_iter().fold(builder, |b, entry| {
                    use ScheduleEntry::*;
                    match entry {
                        System(system) => b.add_system(system),
                        ThreadLocal(system) => b.add_thread_local(system),
                        ThreadLocalFn(f) => b.add_thread_local_fn(f),
                        Flush => b.flush(),
                    }
                })
            })
            .build()
    }

    /// Append another data
    pub fn append<I>(&mut self, new_data: I)
    where
        I: IntoIterator<Item = (SystemOrder, Vec<ScheduleEntry>)>,
    {
        for (order, entry) in new_data {
            self.0.entry(order).or_default().extend(entry);
        }
    }

    /// Insert single entry
    pub fn insert(&mut self, order: SystemOrder, entry: ScheduleEntry) {
        self.0.entry(order).or_default().push(entry);
    }

    /// Insert multiple entries in a row
    pub fn insert_batch<I>(&mut self, order: SystemOrder, entries: I)
    where
        I: IntoIterator<Item = ScheduleEntry>,
    {
        self.0.entry(order).or_default().extend(entries)
    }
}

impl IntoIterator for ScheduleData {
    type Item = (SystemOrder, Vec<ScheduleEntry>);
    type IntoIter = std::collections::btree_map::IntoIter<SystemOrder, Vec<ScheduleEntry>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<ScheduleData> for Schedule {
    fn from(data: ScheduleData) -> Self {
        data.into_schedule()
    }
}
