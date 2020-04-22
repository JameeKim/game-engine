use crate::components::{Children, Parent, ParentTransform, Position, Rotation, WorldTransform};
use crate::ecs::command::CommandBuffer;
use crate::ecs::entity::Entity;
use crate::ecs::filter::filter_fns::component;
use crate::ecs::query::{IntoQuery, Read};
use crate::ecs::schedule::Schedulable;
use crate::ecs::system::{SubWorld, SystemBuilder};
use crate::utils::entity_should_have_transform_in_sub_world;

/// Build a system that updates [`WorldTransform`] component of entities in hierarchy
///
/// [`WorldTransform`]: ../components/struct.WorldTransform.html
pub fn build_hierarchical_transform_update_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("HierarchicalTransformUpdate")
        .with_query(
            <(Read<Children>, Read<WorldTransform>)>::query().filter(!component::<Parent>()),
        )
        .read_component::<Children>()
        .read_component::<ParentTransform>()
        .read_component::<Position>()
        .read_component::<Rotation>()
        .build(|cmd, world, _resources, query| {
            for (children, world_transform) in query.iter(world) {
                for child in children.iter() {
                    update_world_transform_recursive(cmd, world, *world_transform, *child);
                }
            }
        })
}

fn update_world_transform_recursive(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    parent_world_transform: WorldTransform,
    entity: Entity,
) {
    if !entity_should_have_transform_in_sub_world(entity, world) {
        // This entity does not need to be updated
        // Also skip for children
        return;
    }

    let parent_transform = if let Some(t) = world.get_component::<ParentTransform>(entity) {
        *t
    } else {
        log::warn!(
            "ParentTransform not found for an entity in a hierarchy: {}",
            entity
        );
        return;
    };

    let world_transform = parent_world_transform * parent_transform;
    cmd.add_component(entity, world_transform);

    if let Some(children) = world.get_component::<Children>(entity) {
        for child in children.iter() {
            update_world_transform_recursive(cmd, world, world_transform, *child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::build_hierarchical_transform_update_system as build_system;
    use crate::components::*;
    use crate::ecs::prelude::*;
    use crate::math::Vector3;
    use smallvec::smallvec;
    use std::f32::consts::FRAC_PI_4;

    #[test]
    fn hierarchical_transform_update_system() {
        let mut world = World::new();
        let system = build_system();

        // Define positions, rotations, and parent transforms to use

        let wt = WorldTransform::identity();

        let root_pos = Position::from_y(1.0);

        let left_pos = Position::from_x(-1.0);
        let left_rot = Rotation::from_axis_angle(Vector3::new(0.0, 0.0, 1.0) * FRAC_PI_4);
        let left_pt = ParentTransform::from(left_rot * left_pos);

        let right_pos = Position::from_x(1.0);
        let right_rot = Rotation::from_axis_angle(Vector3::new(0.0, 0.0, -1.0) * FRAC_PI_4);
        let right_pt = ParentTransform::from(right_rot * right_pos);

        let left_weapon_pos = Position::from_x(-1.0);
        let left_weapon_rot = Rotation::from_axis_angle(Vector3::new(-1.0, 0.0, 0.0) * FRAC_PI_4);
        let left_weapon_pt = ParentTransform::from(left_weapon_rot * left_weapon_pos);

        let right_weapon_pos = Position::from_x(-1.0);
        let right_weapon_rot = Rotation::from_axis_angle(Vector3::new(-1.0, 0.0, 0.0) * FRAC_PI_4);
        let right_weapon_pt = ParentTransform::from(right_weapon_rot * right_weapon_pos);

        // Add entities to the world

        let root = world.insert((), Some((wt, root_pos)))[0];

        let (left, right) = {
            let entities = world.insert(
                (),
                vec![
                    (wt, left_pt, Parent::new(root), left_pos, left_rot),
                    (wt, right_pt, Parent::new(root), right_pos, right_rot),
                ],
            );
            (entities[0], entities[1])
        };
        world.add_component(root, Children::from(smallvec![left, right]));

        let (left_weapon, right_weapon) = {
            let entities = world.insert(
                (),
                vec![
                    (
                        wt,
                        left_weapon_pt,
                        Parent::new(left),
                        left_weapon_pos,
                        left_weapon_rot,
                    ),
                    (
                        wt,
                        right_weapon_pt,
                        Parent::new(right),
                        right_weapon_pos,
                        right_weapon_rot,
                    ),
                ],
            );
            (entities[0], entities[1])
        };
        world.add_component(left, Children::from(smallvec![left_weapon]));
        world.add_component(right, Children::from(smallvec![right_weapon]));

        // Run the system to check if it updates all `WorldTransform` components in the hierarchy
        system.run(&world);
        system.command_buffer_mut().write(&mut world);

        // Assertions
        assert_eq!(
            *world.get_component::<WorldTransform>(left).unwrap(),
            wt * left_pt
        );
        assert_eq!(
            *world.get_component::<WorldTransform>(right).unwrap(),
            wt * right_pt
        );
        assert_eq!(
            *world.get_component::<WorldTransform>(left_weapon).unwrap(),
            wt * left_pt * left_weapon_pt
        );
        assert_eq!(
            *world.get_component::<WorldTransform>(right_weapon).unwrap(),
            wt * right_pt * right_weapon_pt
        );
    }
}
