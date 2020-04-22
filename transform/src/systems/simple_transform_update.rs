use crate::components::{Parent, ParentTransform, Position, Rotation, WorldTransform};
use crate::ecs::filter::filter_fns::{changed, component};
use crate::ecs::query::{IntoQuery, Read, TryRead, Write};
use crate::ecs::schedule::Schedulable;
use crate::ecs::system::SystemBuilder;

macro_rules! transform_update_system_fn {
    (
        $( #[$attrs:meta] )*
        pub fn $fn_name:ident<$comp:ident>($system_name:expr) -> Box<dyn Schedulable> {
            filters: {
                $( update: $update_filter:expr, )?
                $( add: $add_filter:expr, )?
            }
        }
    ) => {
        $( #[$attrs] )*
        pub fn $fn_name() -> Box<dyn Schedulable> {
            SystemBuilder::new($system_name)
                // Position only
                .with_query(
                    <(Write<$comp>, Read<Position>)>::query()
                        .filter((!component::<Rotation>() & changed::<Position>()) $( & $update_filter )?),
                )
                // Rotation only
                .with_query(
                    <(Write<$comp>, Read<Rotation>)>::query()
                        .filter((!component::<Position>() & changed::<Rotation>()) $( & $update_filter )?),
                )
                // Position & Rotation
                .with_query(
                    <(Write<$comp>, Read<Position>, Read<Rotation>)>::query()
                        .filter((changed::<Position>() | changed::<Rotation>()) $( & $update_filter )?),
                )
                // Transform should be added
                .with_query(
                    <(TryRead<Position>, TryRead<Rotation>)>::query().filter(
                        !component::<$comp>()
                            & (component::<Position>() | component::<Rotation>())
                            $( & $add_filter )?
                    ),
                )
                .build(|cmd, world, _resources, queries| {
                    let (
                        position_only,
                        rotation_only,
                        position_and_rotation,
                        transforms_to_add,
                    ) = queries;
                    rayon::scope(|s| {
                        // Position only
                        s.spawn(|_| unsafe {
                            position_only.for_each_unchecked(world, |(mut transform, position)| {
                                *transform = position.to_homogeneous().into();
                            });
                        });
                        // Rotation only
                        s.spawn(|_| unsafe {
                            rotation_only.for_each_unchecked(world, |(mut transform, rotation)| {
                                *transform = rotation.to_homogeneous().into();
                            });
                        });
                        // Position and Rotation
                        s.spawn(|_| unsafe {
                            position_and_rotation.for_each_unchecked(
                                world,
                                |(mut transform, position, rotation)| {
                                    *transform = rotation
                                        .to_homogeneous()
                                        .append_translation(&position.vector)
                                        .into();
                                },
                            );
                        });
                        // Transform should be added
                        s.spawn(|_| unsafe {
                            transforms_to_add.for_each_entities_unchecked(
                                world,
                                |(entity, (position, rotation))| {
                                    let mut transform = if let Some(rotation) = rotation {
                                        $comp::from(rotation.to_homogeneous())
                                    } else {
                                        $comp::new()
                                    };
                                    if let Some(position) = position {
                                        transform.append_translation_mut(&position.vector);
                                    }
                                    cmd.add_component(entity, transform);
                                },
                            );
                        });
                    });
                })

        }
    };
}

transform_update_system_fn! {
    /// Build a system that updates [`ParentTransform`] component from [`Position`] and/or [`Rotation`] components
    ///
    /// This system only updates entities with [`Parent`] component. It also adds any missing [`ParentTransform`]
    /// components for entities that have [`Parent`] component and at least one of [`Position`] and [`Rotation`]
    /// components.
    ///
    /// [`ParentTransform`]: ../components/struct.ParentTransform.html
    /// [`Position`]: ../components/struct.Position.html
    /// [`Rotation`]: ../components/struct.Rotation.html
    /// [`Parent`]: ../components/struct.Parent.html
    pub fn build_parent_transform_update_system<ParentTransform>(
        "ParentTransformUpdate"
    ) -> Box<dyn Schedulable> {
        filters: {
            add: component::<Parent>(),
        }
    }
}

transform_update_system_fn! {
    /// Build a system that updates [`WorldTransform`] component from [`Position`] and/or [`Rotation`] components
    ///
    /// This system does **NOT** update the component for entities that have [`Parent`] component. This means that only
    /// entities at the root of the hierarchy tree are updated. It also adds any missing [`WorldTransform`] components
    /// for entities that have at least one of [`Position`] and [`Rotation`] components.
    ///
    /// [`WorldTransform`]: ../components/struct.WorldTransform.html
    /// [`Position`]: ../components/struct.Position.html
    /// [`Rotation`]: ../components/struct.Rotation.html
    /// [`Parent`]: ../components/struct.Parent.html
    pub fn build_world_transform_update_system<WorldTransform>(
        "WorldTransformUpdate"
    ) -> Box<dyn Schedulable> {
        filters: {
            update: !component::<Parent>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::build_world_transform_update_system as build_system;
    use crate::components::*;
    use crate::ecs::prelude::*;
    use crate::math::Vector3;
    use std::f32::consts::FRAC_PI_3;

    #[test]
    fn world_transform_update_system() {
        let mut world = World::new();
        let system = build_system();

        let t = WorldTransform::new();
        let p = Position::from_xyz(1.0, 2.0, 3.0);
        let r = Rotation::from_axis_angle(Vector3::new(0.0, 0.0, FRAC_PI_3));

        let pos = world.insert((), Some((t, p)))[0];
        let rot = world.insert((), Some((t, r)))[0];
        let pos_rot = world.insert((), Some((t, p, r)))[0];
        let pos_w_parent = world.insert((), Some((t, p, Parent::new(pos_rot))))[0];
        let no_t = world.insert((), Some((p, r)))[0];

        system.run(&world);
        system.command_buffer_mut().write(&mut world);

        assert_eq!(
            **world.get_component::<WorldTransform>(pos).unwrap(),
            p.to_homogeneous()
        );
        assert_eq!(
            **world.get_component::<WorldTransform>(rot).unwrap(),
            r.to_homogeneous()
        );
        assert_eq!(
            **world.get_component::<WorldTransform>(pos_rot).unwrap(),
            r.to_homogeneous().append_translation(&p.vector)
        );
        assert_eq!(
            *world.get_component::<WorldTransform>(pos_w_parent).unwrap(),
            t // should not update the entities with `Parent` component
        );
        assert_eq!(
            **world.get_component::<WorldTransform>(no_t).unwrap(),
            r.to_homogeneous().append_translation(&p.vector)
        );
    }
}
