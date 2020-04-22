use crate::ecs::entity::Entity;
use std::ops::{Deref, DerefMut};

/// Component that stores the entity that was previously set as the parent of this entity
///
/// This component is not intended to be used by end users. It is used to track changes in [`Parent`] component. If you
/// want to change the hierarchy in some way, you need to make changes to the [`Parent`] component.
///
/// This component is tracked and updated by [`HierarchySync`] system.
///
/// [`Parent`]: ./struct.Parent.html
/// [`HierarchySync`]: ../systems/fn.build_hierarchy_sync_system.html
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PreviousParent(Option<Entity>);

impl PreviousParent {
    /// Create a new instance with `None` as its inner value
    pub fn new() -> PreviousParent {
        PreviousParent::default()
    }

    /// Create a new instance with the given entity as the inner value
    pub fn from_entity(entity: Entity) -> PreviousParent {
        PreviousParent(Some(entity))
    }

    /// Get the value stored inside
    pub fn inner(self) -> Option<Entity> {
        self.0
    }
}

impl From<Entity> for PreviousParent {
    fn from(entity: Entity) -> Self {
        PreviousParent::from_entity(entity)
    }
}

impl From<Option<Entity>> for PreviousParent {
    fn from(option: Option<Entity>) -> Self {
        PreviousParent(option)
    }
}

impl From<PreviousParent> for Option<Entity> {
    fn from(value: PreviousParent) -> Self {
        value.0
    }
}

impl Deref for PreviousParent {
    type Target = Option<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PreviousParent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
