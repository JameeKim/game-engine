//! Utility traits for creating "wrappers" of schedule builders to ensure some systems are automatically added at the
//! front and/or the end
//!
//! # Example
//!
//! ```rust
//! use game_engine::core::schedule_wrapper::{ScheduleBuilder, ScheduleBuilderWrapper, ScheduleBuilderWrapperBuilder};
//! use game_engine::ecs::prelude::*;
//! use std::marker::PhantomData;
//!
//! # // It is safe to use static mut here because the systems will be executed sequentially
//! # static mut CHAR_LIST: &mut [Option<char>; 3] = &mut [None, None, None];
//! # static mut IDX: usize = 0;
//! #
//! /// Build a system that prints the given letter to the stdout
//! fn build_print_char_system(character: char) -> Box<dyn Schedulable> {
//!     SystemBuilder::new(format!("{}", character)).build(move |_, _, _, _| {
//!         println!("{}", character);
//! #       unsafe {
//! #           assert!(CHAR_LIST[IDX].replace(character).is_none());
//! #           IDX += 1;
//! #       }
//!     })
//! }
//!
//! /// The wrapper of a schedule builder
//! ///
//! /// This wrapper adds system A at the front and system B at the end.
//! struct ScheduleABWrapper<T: ScheduleBuilder>(T);
//!
//! /// The builder of `ScheduleABWrapper`
//! struct ScheduleABWrapperBuilder<T: ScheduleBuilder>(PhantomData<T>);
//!
//! impl<T: ScheduleBuilder> ScheduleABWrapper<T> {
//!     fn builder() -> ScheduleABWrapperBuilder<T> {
//!         ScheduleABWrapperBuilder(PhantomData)
//!     }
//! }
//!
//! impl<T: ScheduleBuilder> ScheduleBuilderWrapperBuilder<T> for ScheduleABWrapperBuilder<T> {
//!     type Wrapper = ScheduleABWrapper<T>;
//!
//!     fn build_wrapper(self, inner: T) -> Self::Wrapper {
//!         // start the wrapper with system A, then flush to keep the execution order
//!         ScheduleABWrapper(inner.add_system(build_print_char_system('A')).flush())
//!     }
//! }
//!
//! impl<T: ScheduleBuilder> ScheduleBuilder for ScheduleABWrapper<T> {
//!     fn add_system<S: Into<Box<dyn Schedulable>>>(self, system: S) -> Self {
//!         ScheduleABWrapper(self.0.add_system(system))
//!     }
//!
//!     fn flush(self) -> Self {
//!         ScheduleABWrapper(self.0.flush())
//!     }
//! }
//!
//! impl<T: ScheduleBuilder> ScheduleBuilderWrapper<T> for ScheduleABWrapper<T> {
//!     fn end_wrap(self) -> T {
//!         // end the wrapper by flushing the command buffer to keep the execution order, then adding system B
//!         self.0.flush().add_system(build_print_char_system('B'))
//!     }
//! }
//!
//! fn main() {
//!     let mut world = Universe::new().create_world();
//!     let mut schedule = Schedule::builder()
//!         .wrap(ScheduleABWrapper::builder()) // this step already inserts system A
//!         .add_system(build_print_char_system('C')) // system C will run in the middle, wrapped by systems A and B
//!         .end_wrap() // end up the wrapping and return the `Builder` inside
//!         .build(); // method of `Builder` to finally build the `Schedule`
//!
//!     // This prints 'A', 'C', then 'B', each in its own line
//!     schedule.execute(&mut world);
//! #
//! #   assert_eq!(unsafe { CHAR_LIST[0] }, Some('A'));
//! #   assert_eq!(unsafe { CHAR_LIST[1] }, Some('C'));
//! #   assert_eq!(unsafe { CHAR_LIST[2] }, Some('B'));
//! }
//! ```

use crate::ecs::schedule::{Builder, Schedulable};

/// Generic trait for building a [`Schedule`]
///
/// [`Builder`] provided by [`legion`] implements this trait.
///
/// # Example
///
/// For an example usage, see the [module documentation](./index.html).
///
/// [`Schedule`]: ../../legion/schedule/struct.Schedule.html
/// [`Builder`]: ../../legion/schedule/struct.Builder.html
/// [`legion`]: ../../legion/index.html
pub trait ScheduleBuilder {
    /// Add the given system
    fn add_system<S: Into<Box<dyn Schedulable>>>(self, system: S) -> Self;

    /// Flush the command buffer
    fn flush(self) -> Self;

    /// Wrap into a wrapper by giving it a builder for the wrapper
    fn wrap<W: ScheduleBuilderWrapperBuilder<Self>>(self, wrapper_builder: W) -> W::Wrapper
    where
        Self: Sized,
    {
        wrapper_builder.build_wrapper(self)
    }
}

/// Trait for wrappers of any [`ScheduleBuilder`]s
///
/// # Example
///
/// For an example usage, see the [module documentation](./index.html).
///
/// [`ScheduleBuilder`]: ./trait.ScheduleBuilder.html
pub trait ScheduleBuilderWrapper<T: ScheduleBuilder>: ScheduleBuilder {
    /// Execute end procedures of the wrapper and return the inner builder
    fn end_wrap(self) -> T;
}

/// Trait for builders of [`ScheduleBuilderWrapper`]s
///
/// # Example
///
/// For an example usage, see the [module documentation](./index.html).
///
/// [`ScheduleBuilderWrapper`]: ./trait.ScheduleBuilderWrapper.html
pub trait ScheduleBuilderWrapperBuilder<T: ScheduleBuilder> {
    /// The wrapper this builder creates
    type Wrapper: ScheduleBuilderWrapper<T>;

    /// Build the wrapper
    fn build_wrapper(self, inner: T) -> Self::Wrapper;
}

impl ScheduleBuilder for Builder {
    fn add_system<S: Into<Box<dyn Schedulable>>>(self, system: S) -> Self {
        self.add_system(system)
    }

    fn flush(self) -> Self {
        self.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::{ScheduleBuilder, ScheduleBuilderWrapper, ScheduleBuilderWrapperBuilder};
    use crate::ecs::prelude::*;
    use std::marker::PhantomData;
    use std::sync::{Arc, Mutex};

    #[test]
    fn schedule_builder_wrapper() {
        struct Wrapper<T: ScheduleBuilder> {
            inner: T,
            end: Box<dyn Schedulable>,
        }

        struct WrapperBuilder<T: ScheduleBuilder> {
            start: Box<dyn Schedulable>,
            end: Box<dyn Schedulable>,
            marker: PhantomData<T>,
        }

        impl<T: ScheduleBuilder> ScheduleBuilder for Wrapper<T> {
            fn add_system<S: Into<Box<dyn Schedulable>>>(mut self, system: S) -> Self {
                self.inner = self.inner.add_system(system);
                self
            }

            fn flush(mut self) -> Self {
                self.inner = self.inner.flush();
                self
            }
        }

        impl<T: ScheduleBuilder> ScheduleBuilderWrapper<T> for Wrapper<T> {
            fn end_wrap(self) -> T {
                self.inner.flush().add_system(self.end)
            }
        }

        impl<T: ScheduleBuilder> ScheduleBuilderWrapperBuilder<T> for WrapperBuilder<T> {
            type Wrapper = Wrapper<T>;

            fn build_wrapper(self, inner: T) -> Self::Wrapper {
                Wrapper {
                    inner: inner.add_system(self.start).flush(),
                    end: self.end,
                }
            }
        }

        impl<T: ScheduleBuilder> Wrapper<T> {
            fn builder(
                start: Box<dyn Schedulable>,
                end: Box<dyn Schedulable>,
            ) -> WrapperBuilder<T> {
                WrapperBuilder {
                    start,
                    end,
                    marker: PhantomData,
                }
            }
        }

        fn create_system(number: usize, data: Arc<Mutex<Vec<usize>>>) -> Box<dyn Schedulable> {
            SystemBuilder::new(format!("System{}", number)).build(move |_, _, _, _| {
                data.lock().unwrap().push(number);
            })
        }

        let list = Arc::new(Mutex::new(Vec::with_capacity(3)));

        let system1 = create_system(1, Arc::clone(&list));
        let system2 = create_system(2, Arc::clone(&list));
        let system3 = create_system(3, Arc::clone(&list));

        let mut schedule = Schedule::builder()
            .wrap(Wrapper::builder(system1, system3))
            .add_system(system2)
            .end_wrap()
            .build();

        let mut world = World::new();
        schedule.execute(&mut world);

        assert_eq!(*list.lock().unwrap(), [1, 2, 3]);
    }
}
