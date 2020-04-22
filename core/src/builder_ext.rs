use crate::ecs::schedule::Schedulable;
use crate::schedule_wrapper::ScheduleBuilder;

/// Trait to add convenience methods to schedule builders
pub trait BuilderExt {
    /// Add multiple systems at once from an iterator
    fn add_systems<S: IntoIterator<Item = Box<dyn Schedulable>>>(self, systems: S) -> Self;
}

impl<T: ScheduleBuilder> BuilderExt for T {
    fn add_systems<S: IntoIterator<Item = Box<dyn Schedulable>>>(self, systems: S) -> Self {
        systems
            .into_iter()
            .fold(self, |prev, system| prev.add_system(system))
    }
}
