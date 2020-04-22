use crate::ecs::entity::Entity;

/// Component that indicates the entity has a parent
///
/// The entity that is stored in this component is the parent of the entity that has this component. Thus, this
/// component indicates that the entity **HAS** a parent, not that the entity is a parent of some other entities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Parent(Entity);

impl Parent {
    /// Create a new instance with the given entity as the parent entity
    pub fn new(parent: Entity) -> Parent {
        Parent(parent)
    }

    /// Get the entity stored inside
    pub fn entity(self) -> Entity {
        self.0
    }

    /// Replace the entity that is stored, and return the previous value
    pub fn replace(&mut self, new_parent: Entity) -> Entity {
        std::mem::replace(&mut self.0, new_parent)
    }
}
