use crate::components::{Children, Parent, PreviousParent};
use crate::core::systems::{types, SystemDesc, SystemType};
use crate::ecs::command::CommandBuffer;
use crate::ecs::entity::Entity;
use crate::ecs::filter::filter_fns::{changed, component};
use crate::ecs::query::{IntoQuery, Read, TryRead};
use crate::ecs::schedule::Schedulable;
use crate::ecs::system::{SubWorld, SystemBuilder};
use crate::ecs::world::World;
use std::collections::{HashMap, HashSet};

/// System descriptor(builder) for maintaining hierarchy information from [`Parent`] components
///
/// This is a wrapper for [`build_hierarchy_sync_system`].
///
/// [`Parent`]: ../components/struct.Parent.html
/// [`build_hierarchy_sync_system`]: ./fn.build_hierarchy_sync_system.html
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, SystemDesc)]
#[system_desc(type(types::Parallel), fn(build_hierarchy_sync_system))]
pub struct HierarchySyncSystem;

/// Build a system that maintains the hierarchy information collected from [`Parent`] components
///
/// This system is responsible for
/// 1. Updating [`Children`] component of previous parents of entities which [`Parent`] components are removed
/// 1. Doing the same for entities whose [`Parent`] components are added or modified, and update [`Children`] components
///    of the new parents, adding the component if it does not exist
/// 1. While doing the above steps, detecting any deleted entities that were parents of some other entities, and
///    deleting those entities
/// 1. For the time being, checking every [`Parent`] component to see if any parent entities got deleted
///
/// The last one will be removed for the sake of performance when [`legion`] provides detection of entity deletion.
///
/// [`Parent`]: ../components/struct.Parent.html
/// [`Children`]: ../components/struct.Children.html
/// [`legion`]: ../../legion/index.html
pub fn build_hierarchy_sync_system(_: &mut World) -> Box<dyn Schedulable> {
    SystemBuilder::new("HierarchySync")
        // entities whose `Parent` component are removed
        .with_query(<Read<PreviousParent>>::query().filter(!component::<Parent>()))
        // entities whose `Parent` components are added or modified
        .with_query(<(Read<Parent>, TryRead<PreviousParent>)>::query().filter(changed::<Parent>()))
        // entities whose `Parent` components are not modified
        .with_query(<Read<Parent>>::query().filter(!changed::<Parent>()))
        .write_component::<Children>()
        .build(|cmd, world, _, queries| {
            let (parent_removed_query, parent_changed_query, check_parent_alive_query) = queries;

            // Entities with `Parent` components removed
            // This is for entities that got out of the hierarchy, not those whose parents are deleted
            for (entity, previous_parent) in parent_removed_query.iter_entities(world) {
                if let Some(previous_parent) = previous_parent.inner() {
                    // This entity was previously attached to other entity
                    if let Some(mut children) = world.get_component_mut::<Children>(previous_parent)
                    {
                        // Update the `Children` component of the previous parent since it exists
                        children.retain(|e| *e != entity);
                    }

                    // update the `PreviousParent` component
                    cmd.add_component(entity, PreviousParent::new());
                }
            }

            // Any entities to remove, including their children
            let mut entities_to_remove = HashSet::<Entity>::new();

            // Entities that need `Children` component to be newly inserted
            let mut children_to_insert = HashMap::<Entity, Children>::new();

            // Entities that have their `Parent` components added or modified
            for (entity, (parent, previous_parent)) in parent_changed_query.iter_entities(world) {
                let parent = parent.entity();

                if let Some(previous_parent) = previous_parent {
                    // This entity has `PreviousParent` component
                    if let Some(previous_parent) = previous_parent.inner() {
                        // This entity was previously attached to other entity
                        if parent == previous_parent {
                            // The component storage was marked changed but the value did not actually change, so ignore
                            // ...unless the parent entity is deleted
                            if !world.is_alive(parent) {
                                entities_to_remove.insert(entity);
                            }
                            continue;
                        }

                        if let Some(mut children) =
                            world.get_component_mut::<Children>(previous_parent)
                        {
                            // Update the `Children` component of the previous parent since it exists
                            children.retain(|e| *e != entity);
                        }
                    }
                }

                // update the `PreviousParent` component
                cmd.add_component(entity, PreviousParent::from_entity(parent));

                if world.is_alive(parent) {
                    // The new parent entity is alive
                    if let Some(children) = world
                        .get_component_mut::<Children>(parent) // get a real attached component
                        .as_deref_mut()
                        .or_else(|| children_to_insert.get_mut(&parent))
                    // get a new component to be added later
                    {
                        // There is already a `Children` component to modify
                        children.push(entity);
                    } else {
                        // Create a new `Children` component and save it for later
                        children_to_insert.insert(parent, Children::from_single(entity));
                    }
                } else {
                    // The new parent is deleted, so save this entity to delete later
                    entities_to_remove.insert(entity);
                }
            }

            // This is a hack to detect any entities that are parents of some other entities that got deleted.
            // It iterates through all entities that have `Parent` components and check if each parent entity is alive.
            // This needs to be changed when deleted component detection becomes available in legion.
            for (entity, parent) in check_parent_alive_query.iter_entities(world) {
                if !world.is_alive(parent.entity()) {
                    // The parent is delete, so save this entity to delete later
                    entities_to_remove.insert(entity);
                }
            }

            // This is not needed if deleting the same entity twice is okay
            let mut entities_removed = HashSet::<Entity>::with_capacity(entities_to_remove.len());
            // Delete the saved entities and their children
            delete_entities_and_children_recursive(
                cmd,
                world,
                &children_to_insert,
                &mut entities_removed,
                entities_to_remove,
            );

            // Add the new `Children` components
            for (entity, children) in children_to_insert.into_iter() {
                cmd.add_component(entity, children);
            }
        })
}

fn delete_entities_and_children_recursive<I>(
    cmd: &mut CommandBuffer,
    world: &mut SubWorld,
    children_to_insert: &HashMap<Entity, Children>,
    removed: &mut HashSet<Entity>,
    entities_to_remove: I,
) where
    I: IntoIterator<Item = Entity>,
{
    for entity in entities_to_remove {
        if removed.contains(&entity) {
            continue;
        }

        cmd.delete(entity);
        removed.insert(entity);

        let mut entities_to_remove = None;

        if let Some(children) = world
            .get_component_mut::<Children>(entity)
            .as_deref()
            .or_else(|| children_to_insert.get(&entity))
        {
            entities_to_remove.replace(children.iter().copied().collect::<Vec<_>>());
        }

        if let Some(entities_to_remove) = entities_to_remove {
            delete_entities_and_children_recursive(
                cmd,
                world,
                children_to_insert,
                removed,
                entities_to_remove,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::build_hierarchy_sync_system;
    use crate::components::{Children, Parent, PreviousParent};
    use crate::ecs::prelude::*;

    #[test]
    fn hierarchy_sync_system() {
        let mut world = World::new();
        let system = build_hierarchy_sync_system(&mut world);

        // initial hierarchy
        // <-- parent                 child -->
        // ┌─ r[0](1) ─┬─ g1[0](3) ─── g2[0](7)
        // │           └─ g1[1](4)
        // └─ r[1](2) ─┬─ g1[2](5) ─┬─ g2[1](8)
        //             │            └─ g2[2](9)
        //             └─ g1[3](6)
        let r = world.insert((), vec![(1,), (2,)]).to_vec();
        let p1 = Parent::new(r[0]);
        let p2 = Parent::new(r[1]);
        let g1 = world
            .insert((), vec![(3, p1), (4, p1), (5, p2), (6, p2)])
            .to_vec();
        let p3 = Parent::new(g1[0]);
        let p5 = Parent::new(g1[2]);
        let g2 = world.insert((), vec![(7, p3), (8, p5), (9, p5)]).to_vec();

        system.run(&world);
        system.command_buffer_mut().write(&mut world);

        assert_eq!(***world.get_component::<Children>(r[0]).unwrap(), g1[0..2]);
        assert_eq!(***world.get_component::<Children>(r[1]).unwrap(), g1[2..4]);
        assert_eq!(***world.get_component::<Children>(g1[0]).unwrap(), g2[0..1]);
        assert!(world.get_component::<Children>(g1[1]).is_none());
        assert_eq!(***world.get_component::<Children>(g1[2]).unwrap(), g2[1..3]);
        assert!(world.get_component::<Children>(g1[3]).is_none());
        assert!(g2
            .iter()
            .all(|e| world.get_component::<Children>(*e).is_none()));

        // changed hierarchy
        // <-- parent                  child -->
        // ┌─  r[1](2) ─── g1[3](6) ─── g2[2](9)
        // └─ g1[2](5) ─┬─ g2[1](8)
        //              └─  r[0](1) ─── g1[1](4)
        // - g1[0](3) is deleted, and so is its child g2[0](7) by the system
        // - g2[2](9) has become a child of g1[3](6), not g1[2](5)
        // - g1[2](6) has become a root entity, not a child of r[1](2)
        // -  r[0](1) has become a child of g1[2](5), not a root entity
        world.delete(g1[0]);
        world
            .get_component_mut::<Parent>(g2[2])
            .unwrap()
            .replace(g1[3]);
        world.remove_component::<Parent>(g1[2]);
        world.add_component(r[0], Parent::new(g1[2]));

        system.run(&world);
        system.command_buffer_mut().write(&mut world);

        assert!(!world.is_alive(g2[0]));
        assert_eq!(***world.get_component::<Children>(g1[3]).unwrap(), [g2[2]]);
        assert!(world
            .get_component::<PreviousParent>(g1[2])
            .unwrap()
            .is_none());
        assert_eq!(
            ***world.get_component::<Children>(g1[2]).unwrap(),
            [g2[1], r[0]]
        );
        assert_eq!(***world.get_component::<Children>(r[1]).unwrap(), [g1[3]]);
    }
}
