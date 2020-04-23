//! Things related to schedule entry, which is a data of what kind of system to add to the schedule

#![allow(missing_debug_implementations)]

use crate::ecs::world::World;
use crate::systems::{
    ScheduleBuilder, ScheduleData, SystemBundle, SystemDesc, SystemOrder, SystemType,
};
use legion::schedule::{Runnable, Schedulable};
use std::fmt;
use std::marker::PhantomData;

/// Entry of systems or flush command for building [`Schedule`]
///
/// [`Schedule`]: ../../legion/schedule/struct.Schedule.html
pub enum ScheduleEntry {
    /// Regular system
    System(Box<dyn Schedulable>),
    /// Thread local system as a trait object
    ThreadLocal(Box<dyn Runnable>),
    /// Thread local system as a closure
    ThreadLocalFn(Box<dyn FnMut(&mut World)>),
    /// Flush the command buffer
    Flush,
}

impl fmt::Debug for ScheduleEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduleEntry::System(_) => f.debug_tuple("Parallel").finish(),
            ScheduleEntry::ThreadLocal(_) => f.debug_tuple("ThreadLocal").finish(),
            ScheduleEntry::ThreadLocalFn(_) => f.debug_tuple("ThreadLocalFn").finish(),
            ScheduleEntry::Flush => f.debug_tuple("Flush").finish(),
        }
    }
}

impl From<Box<dyn Schedulable>> for ScheduleEntry {
    fn from(system: Box<dyn Schedulable>) -> Self {
        Self::System(system)
    }
}

impl From<Box<dyn Runnable>> for ScheduleEntry {
    fn from(system: Box<dyn Runnable>) -> Self {
        Self::ThreadLocal(system)
    }
}

impl From<Box<dyn FnMut(&mut World)>> for ScheduleEntry {
    fn from(f: Box<dyn FnMut(&mut World)>) -> Self {
        Self::ThreadLocalFn(f)
    }
}

/// Descriptor(builder) for [`ScheduleEntry`]
///
/// [`ScheduleEntry`]: ./enum.ScheduleEntry.html
pub trait ScheduleEntryDesc {
    /// Insert schedule entry into the given schedule data
    fn insert_entry(self: Box<Self>, world: &mut World, data: &mut ScheduleData);
}

/// Schedule entry descriptor for the given type of system at the given system execution order,
/// with the system created from the given function or closure
///
/// # Type Parameters
///
/// - `Ty`: The type of the system; one of [`Parallel`], [`ThreadLocal`], or [`ThreadLocalFn`]
/// - `F`: The function or closure to create the system
///
/// [`Parallel`]: ../types/struct.Parallel.html
/// [`ThreadLocal`]: ../types/struct.ThreadLocal.html
/// [`ThreadLocalFn`]: ../types/struct.ThreadLocalFn.html
pub struct SystemCreateFn<Ty, F>(pub SystemOrder, pub F, PhantomData<Ty>)
where
    Ty: SystemType,
    F: FnOnce(&mut World) -> Ty::System;

impl<Ty, F> SystemCreateFn<Ty, F>
where
    Ty: SystemType,
    F: FnOnce(&mut World) -> Ty::System,
{
    /// Create a new instance
    pub fn new(order: SystemOrder, f: F) -> Self {
        Self(order, f, PhantomData)
    }
}

impl<Ty, F> ScheduleEntryDesc for SystemCreateFn<Ty, F>
where
    Ty: SystemType,
    F: FnOnce(&mut World) -> Ty::System,
{
    fn insert_entry(self: Box<Self>, world: &mut World, data: &mut ScheduleData) {
        data.insert(self.0, (self.1)(world).into());
    }
}

/// Schedule entry descriptor from the given system descriptor at the given system execution order
///
/// # Type Parameters
///
/// - `D`: The system descriptor
pub struct SystemCreateDesc<D>(pub SystemOrder, pub D)
where
    D: SystemDesc;

impl<D> SystemCreateDesc<D>
where
    D: SystemDesc,
{
    /// Create a new instance
    pub fn new(order: SystemOrder, desc: D) -> Self {
        Self(order, desc)
    }
}

impl<D> ScheduleEntryDesc for SystemCreateDesc<D>
where
    D: SystemDesc,
{
    fn insert_entry(self: Box<Self>, world: &mut World, data: &mut ScheduleData) {
        data.insert(self.0, self.1.build(world).into());
    }
}

/// Schedule entry descriptor from the given system bundle
///
/// # Type Parameters
///
/// - `B`: The given system bundle
pub struct SystemBundleEntry<B>(pub B)
where
    B: SystemBundle;

impl<B> SystemBundleEntry<B>
where
    B: SystemBundle,
{
    /// Create a new instance
    pub fn new(bundle: B) -> Self {
        Self(bundle)
    }
}

impl<B> ScheduleEntryDesc for SystemBundleEntry<B>
where
    B: SystemBundle,
{
    fn insert_entry(self: Box<Self>, world: &mut World, data: &mut ScheduleData) {
        let mut builder = ScheduleBuilder::new();
        self.0.build_systems(world, &mut builder);
        data.append(builder.into_data(world));
    }
}

/// Schedule entry descriptor from the given function that acts as a system bundle
///
/// # Type Parameters
///
/// - `F`: The given function to act as a system bundle
pub struct SystemBundleFn<F>(pub F)
where
    F: FnOnce(&mut World, &mut ScheduleBuilder);

impl<F> SystemBundleFn<F>
where
    F: FnOnce(&mut World, &mut ScheduleBuilder),
{
    /// Create a new instance
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F> ScheduleEntryDesc for SystemBundleFn<F>
where
    F: FnOnce(&mut World, &mut ScheduleBuilder),
{
    fn insert_entry(self: Box<Self>, world: &mut World, data: &mut ScheduleData) {
        let mut builder = ScheduleBuilder::new();
        (self.0)(world, &mut builder);
        data.append(builder.into_data(world));
    }
}

/// Schedule entry descriptor that adds flush command to the command buffer
#[derive(Clone, Copy, Debug)]
pub struct CmdBufFlush(pub SystemOrder);

impl CmdBufFlush {
    /// Create a new instance
    pub fn new(order: SystemOrder) -> Self {
        Self(order)
    }
}

impl ScheduleEntryDesc for CmdBufFlush {
    fn insert_entry(self: Box<Self>, _world: &mut World, data: &mut ScheduleData) {
        data.insert(self.0, ScheduleEntry::Flush);
    }
}
