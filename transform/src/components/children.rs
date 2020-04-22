use crate::ecs::entity::Entity;
use smallvec::{smallvec, SmallVec};
use std::ops::{Deref, DerefMut};

const CHILDREN_SIZE: usize = 4;

/// Component that stores what children this entity has
///
/// This component is not in sync when the user has made changes. It is only in sync when [`HierarchySync`] system is
/// run. Thus, the data in this component should **NOT** be trusted by the end users.
///
/// This component is used in [`HierarchicalTransformUpdate`] system.
///
/// [`HierarchySync`]: ../systems/fn.build_hierarchy_sync_system.html
/// [`HierarchicalTransformUpdate`]: ../systems/fn.build_hierarchical_transform_update_system.html
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Children(SmallVec<[Entity; CHILDREN_SIZE]>);

impl Children {
    /// Create a new instance with nothing in the list
    pub fn new() -> Children {
        Children::default()
    }

    /// Create a new instance with only one entity in the list
    pub fn from_single(entity: Entity) -> Children {
        Children(smallvec![entity])
    }
}

impl From<SmallVec<[Entity; CHILDREN_SIZE]>> for Children {
    fn from(vec: SmallVec<[Entity; 4]>) -> Self {
        Children(vec)
    }
}

impl From<Children> for SmallVec<[Entity; CHILDREN_SIZE]> {
    fn from(value: Children) -> Self {
        value.0
    }
}

impl From<[Entity; CHILDREN_SIZE]> for Children {
    fn from(slice: [Entity; CHILDREN_SIZE]) -> Self {
        Children(slice.into())
    }
}

impl From<&[Entity]> for Children {
    fn from(slice: &[Entity]) -> Self {
        Children(slice.iter().copied().collect())
    }
}

impl Deref for Children {
    type Target = SmallVec<[Entity; CHILDREN_SIZE]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Children {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
