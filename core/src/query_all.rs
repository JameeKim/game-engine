use crate::ecs::entity::Entity;
use crate::ecs::filter::filter_fns::passthrough;
use crate::ecs::filter::{EntityFilterTuple, Passthrough};
use crate::ecs::query::View;
use crate::ecs::query::{DefaultFilter, ReadOnly};
use crate::ecs::storage::{ArchetypeData, Component, ComponentStorage, ComponentTypeId};

/// A struct implementing [`IntoQuery`] to get all entities regardless of the archetype
///
/// # Example
/// ```rust
/// # use game_engine::core::All;
/// # use game_engine::ecs::prelude::*;
/// #
/// # let mut world = World::new();
/// #
/// // insert 2 entities with `u8`, 1 entity with `usize`, and 1 entity with both `u8` and `usize`
/// world.insert((), vec![(1u8,), (5u8,)]);
/// world.insert((), vec![(5usize,)]);
/// world.insert((), vec![(2u8, 10usize)]);
///
/// // entities with `u8`
/// let u8_query = All::query().filter(component::<u8>());
/// assert_eq!(u8_query.iter_immutable(&world).count(), 3);
///
/// // entities without `u8`
/// let no_u8_query = All::query().filter(!component::<u8>());
/// assert_eq!(no_u8_query.iter_immutable(&world).count(), 1);
///
/// // entities with `usize`
/// let usize_query = All::query().filter(component::<usize>());
/// assert_eq!(usize_query.iter_immutable(&world).count(), 2);
/// ```
///
/// [`IntoQuery`]: ../legion/query/trait.IntoQuery.html
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct All;

impl<'a> View<'a> for All {
    type Iter = AllIter<'a>;

    fn fetch(
        _archetype: &'a ArchetypeData,
        chunk: &'a ComponentStorage,
        _chunk_index: usize,
    ) -> Self::Iter {
        AllIter(chunk.entities().iter())
    }

    fn validate() -> bool {
        true
    }

    fn reads<T: Component>() -> bool {
        false
    }

    fn writes<T: Component>() -> bool {
        false
    }

    fn read_types() -> Vec<ComponentTypeId> {
        Vec::with_capacity(0)
    }

    fn write_types() -> Vec<ComponentTypeId> {
        Vec::with_capacity(0)
    }
}

impl DefaultFilter for All {
    type Filter = EntityFilterTuple<Passthrough, Passthrough, Passthrough>;

    fn filter() -> Self::Filter {
        passthrough()
    }
}

impl ReadOnly for All {}

#[derive(Debug)]
pub struct AllIter<'a>(std::slice::Iter<'a, Entity>);

impl<'a> Iterator for AllIter<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()
    }
}

impl<'a> ExactSizeIterator for AllIter<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::All;
    use crate::ecs::prelude::*;

    fn make_world() -> World {
        let mut world = World::new();

        let _e_0_t = world.insert((), Some((0u8, true)))[0];
        let _e_1_f = world.insert((), Some((1u8, false)))[0];
        let _e_2_1 = world.insert((), Some((2u8, 1f32)))[0];
        let _e_3_8 = world.insert((), Some((3u8, 8f32)))[0];
        let _e_4_t_1 = world.insert((), Some((4u8, true, 1f32)))[0];

        world
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn all_entities_query() {
        let world = make_world();

        let query = All::query();

        let [mut count_t, mut count_f, mut count_1, mut count_8] = [0u8; 4];
        let mut sum = 0f32;

        for entity in query.iter_immutable(&world) {
            if let Some(b) = world.get_component::<bool>(entity) {
                if *b {
                    count_t += 1;
                } else {
                    count_f += 1;
                }
            }

            if let Some(f) = world.get_component::<f32>(entity) {
                sum += *f;
                if *f == 1f32 {
                    count_1 += 1;
                } else if *f == 8f32 {
                    count_8 += 1;
                }
            }
        }

        assert_eq!(count_t, 2);
        assert_eq!(count_f, 1);
        assert_eq!(count_1, 2);
        assert_eq!(count_8, 1);
        assert_eq!(sum, 10f32);
    }

    #[test]
    fn all_entities_query_with_filter() {
        let world = make_world();
        let query = All::query().filter(!component::<bool>());
        let count = query.iter_immutable(&world).count();
        assert_eq!(count, 2);
    }
}
