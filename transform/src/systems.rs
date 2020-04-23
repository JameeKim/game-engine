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
//! 1. Do late updates that do not touch anything among [`Position`], [`Rotation`], and [`Parent`]
//!    components
//! 1. Do thread-local stuff like rendering that uses [`WorldTransform`]
//!
//! # That is too complicated
//!
//! If you are adding these systems to the [`Schedule`] [`Builder`] provided by [`legion`], consider
//! using the [`TransformBundle`] helper struct. For an example usage, see the example below.
//!
//! # Examples
//!
//! ```rust
//! use game_engine::core::systems::{ScheduleBuilder, SystemPriority, SystemOrder};
//! use game_engine::ecs::prelude::*;
//! use game_engine::math::{UnitQuaternion, Vector3};
//! use game_engine::transform::components::*;
//! use game_engine::transform::systems::TransformBundle;
//! use std::f32::EPSILON;
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
//! fn build_linear_movement_system(_: &mut World) -> Box<dyn Schedulable> {
//!     SystemBuilder::new("LinearMovement")
//!         .with_query(<(Read<LinearVelocity>, Write<Position>)>::query())
//!         .build(|_, world, _, query| {
//!             for (velocity, mut position) in query.iter(world) {
//!                 *position += velocity.0 * DELTA_SECOND;
//!             }
//!         })
//! }
//!
//! fn build_angular_movement_system(_: &mut World) -> Box<dyn Schedulable> {
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
//! let mut schedule = ScheduleBuilder::new()
//!     .with_system_bundle(
//!         TransformBundle::new_flush(SystemOrder::numbered(1, 0))
//!     )
//!     .with_system_create_fn(SystemOrder::numbered(0, 0), build_linear_movement_system)
//!     .with_system_create_fn(SystemOrder::numbered(0, 0), build_angular_movement_system)
//!     .with_flush(SystemOrder::numbered(0, 0))
//!     .build(&mut world);
//!
//! // The above is the same as below
//! // let mut schedule = Schedule::builder()
//! //     .add_system(build_linear_movement_system(&mut world))
//! //     .add_system(build_angular_movement_system(&mut world))
//! //     .flush()
//! //     .add_system(build_hierarchy_sync_system(&mut world))
//! //     .add_system(build_world_transform_update_system(&mut world))
//! //     .add_system(build_parent_transform_update_system(&mut world))
//! //     .flush()
//! //     .add_system(build_hierarchical_transform_update_system(&mut world))
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
//! [`TransformBundle`]: ./struct.TransformBundle.html

mod hierarchical_transform_update;
mod hierarchy_sync;
mod simple_transform_update;

pub use self::hierarchical_transform_update::build_hierarchical_transform_update_system;
pub use self::hierarchy_sync::build_hierarchy_sync_system;
pub use self::simple_transform_update::{
    build_parent_transform_update_system, build_world_transform_update_system,
};
use crate::core::systems::{ScheduleBuilder, SystemBundle, SystemOrder};
use crate::ecs::world::World;

/// [`SystemBundle`] that adds transform and hierarchy related systems
///
/// [`SystemBundle`]: ../../game_engine_core/systems/trait.SystemBundle.html
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformBundle {
    /// The [`SystemOrder`] for first three systems
    ///
    /// [`SystemOrder`]: ../../game_engine_core/systems/struct.SystemOrder.html
    pub start_order: SystemOrder,
    /// The [`SystemOrder`] for the last system
    ///
    /// [`SystemOrder`]: ../../game_engine_core/systems/struct.SystemOrder.html
    pub end_order: SystemOrder,
    /// Whether to automatically add flush command right before adding the last system
    pub flush_before_end: bool,
}

impl TransformBundle {
    /// Create a new instance with the given system execution order
    pub fn new(start_order: SystemOrder) -> Self {
        Self {
            start_order,
            end_order: start_order,
            flush_before_end: false,
        }
    }

    /// Create a new instance with the given system execution order with automatic flushing
    pub fn new_flush(start_order: SystemOrder) -> Self {
        Self {
            start_order,
            end_order: start_order,
            flush_before_end: true,
        }
    }

    /// Chain method to construct the bundle; sets the order of the first three systems
    pub fn with_start_order(mut self, start_order: SystemOrder) -> Self {
        self.start_order = start_order;
        self
    }

    /// Chain method to construct the bundle; sets the order of the last system
    pub fn with_end_order(mut self, end_order: SystemOrder) -> Self {
        self.end_order = end_order;
        self
    }

    /// Chain method to construct the bundle; automatically add flush command before the last system
    pub fn with_flush(mut self) -> Self {
        self.flush_before_end = true;
        self
    }

    /// Chain method to construct the bundle; do not add flush command before the last system
    pub fn without_flush(mut self) -> Self {
        self.flush_before_end = false;
        self
    }
}

impl SystemBundle for TransformBundle {
    fn build_systems(self, _world: &mut World, builder: &mut ScheduleBuilder) {
        builder.add_system_create_fn(self.start_order, build_hierarchy_sync_system);
        builder.add_system_create_fn(self.start_order, build_world_transform_update_system);
        builder.add_system_create_fn(self.start_order, build_parent_transform_update_system);

        if self.flush_before_end {
            builder.add_flush(self.end_order);
        }
        builder.add_system_create_fn(self.end_order, build_hierarchical_transform_update_system);
    }
}
