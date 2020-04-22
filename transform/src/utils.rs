//! Utility things

use crate::components::{Position, Rotation};
use crate::ecs::entity::Entity;
use crate::ecs::system::SubWorld;
use crate::ecs::world::World;

macro_rules! entity_should_have_transform_fn {
    (
        $( #[$attrs:meta] )*
        pub fn $fn_name:ident in $world:ident (entity: Entity) -> bool;
    ) => {
        $( #[$attrs] )*
        pub fn $fn_name(entity: Entity, world: &$world) -> bool {
            world.get_component::<Position>(entity).is_some()
                || world.get_component::<Rotation>(entity).is_some()
        }
    };
}

entity_should_have_transform_fn! {
    /// Check if this entity has any of components that represent transform in the game world, using
    /// the world
    pub fn entity_should_have_transform_in_world in World (entity: Entity) -> bool;
}

entity_should_have_transform_fn! {
    /// Check if this entity has any of components that represent transform in the game world, using
    /// a sub-world
    pub fn entity_should_have_transform_in_sub_world in SubWorld (entity: Entity) -> bool;
}
