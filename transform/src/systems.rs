//! Functions that build systems needed for transforms and hierarchies to work
//!
//! # System Execution Order
//!
//! 1. Do your stuff with [`Position`], [`Rotation`], and [`Parent`] components
//! 1. **Flush**
//! 1. Combination of
//!    - `hierarchy_sync`
//!    - `world_transform_update`
//!    - `parent_transform_update`
//! 1. **Flush**
//! 1. `hierarchical_transform_update`
//! 1. **Flush**
//! 1. Do late updates that do not touch anything among [`Position`], [`Rotation`], and [`Parent`] components
//! 1. Do thread-local stuff like rendering that uses [`WorldTransform`]
//!
//! # That is too complicated
//!
//! If you are adding these systems to the [`Schedule`] [`Builder`] provided by [`legion`], consider using the
//! [`TransformSchedule`] helper struct. For an example usage, see the example below.
//!
//! # Example
//!
//! ```rust
//! use std::f32::EPSILON;
//! use game_engine::core::schedule_wrapper::*; // traits from this module are needed
//! use game_engine::ecs::prelude::*;
//! use game_engine::math::{UnitQuaternion, Vector3};
//! use game_engine::transform::components::*;
//! use game_engine::transform::systems::TransformSchedule;
//!
//! /// Linear velocity
//! #[derive(Clone, Copy, Debug, PartialEq)]
//! pub struct LinearVelocity(pub Vector3<f32>);
//!
//! /// Angular velocity in axis-angle form
//! #[derive(Clone, Copy, Debug, PartialEq)]
//! pub struct AngularVelocity(pub Vector3<f32>);
//!
//! const DELTA_SECOND: f32 = 0.01;
//!
//! fn build_linear_movement_system() -> Box<dyn Schedulable> {
//!     SystemBuilder::new("LinearMovement")
//!         .with_query(<(Read<LinearVelocity>, Write<Position>)>::query())
//!         .build(|_, world, _, query| {
//!             for (velocity, mut position) in query.iter(world) {
//!                 *position += velocity.0 * DELTA_SECOND;
//!             }
//!         })
//! }
//!
//! fn build_angular_movement_system() -> Box<dyn Schedulable> {
//!     SystemBuilder::new("AngularMovement")
//!         .with_query(<(Read<AngularVelocity>, Write<Rotation>)>::query())
//!         .build(|_, world, _, query| {
//!             for (velocity, mut rotation) in query.iter(world) {
//!                 *rotation *= UnitQuaternion::new(velocity.0 * DELTA_SECOND);
//!             }
//!         })
//! }
//!
//! let mut world = World::new();
//!
//! let mut schedule = Schedule::builder()
//!     .add_system(build_linear_movement_system())
//!     .add_system(build_angular_movement_system())
//!     .wrap(TransformSchedule::builder())
//!     // Here, you can add more systems that can run together with `hierarchy_sync`,
//!     // `world_transform_update`, and `parent_transform_update` systems and before
//!     // `hierarchical_transform_update` system
//!     .end_transform_systems()
//!     .build();
//!
//! // The above is the same as below
//! // let mut schedule = Schedule::builder()
//! //     .add_system(build_linear_movement_system())
//! //     .add_system(build_angular_movement_system())
//! //     .flush()
//! //     .add_system(build_hierarchy_sync_system())
//! //     .add_system(build_world_transform_update_system())
//! //     .add_system(build_parent_transform_update_system())
//! //     // Add more systems here
//! //     .flush()
//! //     .add_system(build_hierarchical_transform_update_system())
//! //     .build();
//!
//! let moving = world.insert((), Some((
//!     LinearVelocity(Vector3::new(10.0, -2.0, 0.0)),
//!     Position::zero(),
//! )))[0];
//!
//! let rotating = world.insert((), Some((
//!     AngularVelocity(Vector3::new(0.0, 3.0, 0.0)),
//!     Rotation::identity(),
//!     Parent::new(moving),
//! )))[0];
//!
//! schedule.execute(&mut world);
//!
//! // `WorldTransform` is inserted by `world_transform_update` system
//! let moving_transform = *world.get_component::<WorldTransform>(moving).unwrap();
//! assert!((moving_transform.m14 - 0.1).abs() < EPSILON); // x coordinate
//! assert!((moving_transform.m24 - -0.02).abs() < EPSILON); // y coordinate
//! assert_eq!(moving_transform.m34, 0.0); // z coordinate
//!
//! // The position follows the parent entity
//! let rotating_transform = *world.get_component::<WorldTransform>(rotating).unwrap();
//! assert!((rotating_transform.m14 - 0.1).abs() < EPSILON); // x coordinate
//! assert!((rotating_transform.m24 - -0.02).abs() < EPSILON); // y coordinate
//! assert_eq!(rotating_transform.m34, 0.0); // z coordinate
//!
//! // The rotation is propagated to the `WorldTransform`
//! let mut rotation = *rotating_transform;
//! rotation.m14 -= rotating_transform.m14;
//! rotation.m24 -= rotating_transform.m24;
//! rotation.m34 -= rotating_transform.m34;
//! assert_eq!(rotation, UnitQuaternion::new(Vector3::new(0.0, 0.03, 0.0)).to_homogeneous());
//!
//! // The position relative to the parent is still at zero
//! let parent_transform = *world.get_component::<ParentTransform>(rotating).unwrap();
//! assert_eq!(
//!     [parent_transform.m14, parent_transform.m24, parent_transform.m34],
//!     [0.0, 0.0, 0.0]
//! );
//! ```
//!
//! [`Position`]: ../components/struct.Position.html
//! [`Rotation`]: ../components/struct.Rotation.html
//! [`Parent`]: ../components/struct.Parent.html
//! [`WorldTransform`]: ../components/struct.WorldTransform.html
//! [`Schedule`]: ../../legion/schedule/struct.Schedule.html
//! [`Builder`]: ../../legion/schedule/struct.Builder.html
//! [`legion`]: ../../legion/index.html
//! [`TransformSchedule`]: ./struct.TransformSchedule.html

mod hierarchical_transform_update;
mod hierarchy_sync;
mod simple_transform_update;

pub use self::hierarchical_transform_update::build_hierarchical_transform_update_system;
pub use self::hierarchy_sync::build_hierarchy_sync_system;
pub use self::simple_transform_update::{
    build_parent_transform_update_system, build_world_transform_update_system,
};

use crate::core::schedule_wrapper::{
    ScheduleBuilder, ScheduleBuilderWrapper, ScheduleBuilderWrapperBuilder,
};
use crate::ecs::schedule::Schedulable;
use std::marker::PhantomData;

/// Helper struct to easily add systems for transform and hierarchy to a schedule builder
///
/// # Example
///
/// For an example usage, see the example at the [module documentation](./index.html).
///
pub struct TransformSchedule<T: ScheduleBuilder> {
    inner: T,
    flush_before_end: bool,
}

/// Builder of [`TransformSchedule`]
///
/// [`TransformSchedule`]: ./struct.TransformSchedule.html
#[derive(Debug)]
pub struct TransformScheduleBuilder<T: ScheduleBuilder> {
    flush_before_start: bool,
    flush_before_end: bool,
    marker: PhantomData<T>,
}

impl<T: ScheduleBuilder> Default for TransformScheduleBuilder<T> {
    fn default() -> Self {
        TransformScheduleBuilder {
            flush_before_start: true,
            flush_before_end: true,
            marker: PhantomData,
        }
    }
}

impl<T: ScheduleBuilder> TransformScheduleBuilder<T> {
    /// Create a new builder with default configurations
    ///
    /// By default, the flushing of the command buffer is enabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new builder that does not call `flush` anywhere
    pub fn new_no_flush() -> Self {
        Self::new().enforce_flush_all(false)
    }

    /// Enable or disable automatic flushing of the command buffer before adding systems at the start
    pub fn enforce_flush_before_start(mut self, enforce: bool) -> Self {
        self.flush_before_start = enforce;
        self
    }

    /// Enable or disable automatic flushing of the command buffer before adding systems at the end
    pub fn enforce_flush_before_end(mut self, enforce: bool) -> Self {
        self.flush_before_end = enforce;
        self
    }

    /// Enable or disable automatic flushing of the command buffer before adding systems at both the start and the end
    pub fn enforce_flush_all(self, enforce: bool) -> Self {
        self.enforce_flush_before_start(enforce)
            .enforce_flush_before_end(enforce)
    }
}

impl<T: ScheduleBuilder> TransformSchedule<T> {
    /// Create a builder for this struct to help configure
    pub fn builder() -> TransformScheduleBuilder<T> {
        TransformScheduleBuilder::new()
    }

    /// Create a builder that does not automatically flush the command buffers
    pub fn builder_no_flush() -> TransformScheduleBuilder<T> {
        TransformScheduleBuilder::new_no_flush()
    }

    /// Add systems to add at the end and return the underlying schedule builder
    ///
    /// This is the same as calling [`ScheduleBuilderWrapper::end_wrap`], but a separate method is declared so that it
    /// is easily distinguishable what wrapper has ended its turn.
    ///
    /// [`ScheduleBuilderWrapper::end_wrap`]: ../../game_engine_core/schedule_wrapper/trait.ScheduleBuilderWrapper.html#method.end_wrap
    pub fn end_transform_systems(self) -> T {
        let mut inner = self.inner;

        if self.flush_before_end {
            inner = inner.flush();
        }

        inner.add_system(build_hierarchical_transform_update_system())
    }
}

impl<T: ScheduleBuilder> ScheduleBuilder for TransformSchedule<T> {
    fn add_system<S: Into<Box<dyn Schedulable>>>(mut self, system: S) -> Self {
        self.inner = self.inner.add_system(system);
        self
    }

    fn flush(mut self) -> Self {
        self.inner = self.inner.flush();
        self
    }
}

impl<T: ScheduleBuilder> ScheduleBuilderWrapper<T> for TransformSchedule<T> {
    fn end_wrap(self) -> T {
        self.end_transform_systems()
    }
}

impl<T: ScheduleBuilder> ScheduleBuilderWrapperBuilder<T> for TransformScheduleBuilder<T> {
    type Wrapper = TransformSchedule<T>;

    fn build_wrapper(self, inner: T) -> Self::Wrapper {
        let TransformScheduleBuilder {
            flush_before_start,
            flush_before_end,
            ..
        } = self;

        let inner = if flush_before_start {
            inner.flush()
        } else {
            inner
        };

        let inner = inner
            .add_system(build_hierarchy_sync_system())
            .add_system(build_world_transform_update_system())
            .add_system(build_parent_transform_update_system());

        TransformSchedule {
            inner,
            flush_before_end,
        }
    }
}

impl<T: ScheduleBuilder> Clone for TransformScheduleBuilder<T> {
    fn clone(&self) -> Self {
        TransformScheduleBuilder {
            flush_before_start: self.flush_before_start,
            flush_before_end: self.flush_before_end,
            marker: PhantomData,
        }
    }
}

impl<T: ScheduleBuilder> std::fmt::Debug for TransformSchedule<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransformSchedule")
            .field("inner", &std::any::type_name::<T>())
            .field("flush_before_end", &self.flush_before_end)
            .finish()
    }
}
