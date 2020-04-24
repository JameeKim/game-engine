use crate::core::frame_rate::{FrameRateConfig, FrameRateKeeper};
use crate::core::time::{Time, Timer};
use crate::ecs::resource::{Fetch, FetchMut, Resource};
use crate::ecs::schedule::Schedule;
use crate::ecs::world::{Universe, World};

/// The main loop of the engine
pub struct Application {
    universe: Universe,
    main_world: World,
    systems: Schedule,
}

impl Application {
    /// Create a builder to help build the app
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }

    /// Run the main loop
    pub fn run(&mut self) {
        self.reset_timer();

        // the main loop
        while self.is_running() {
            self.systems.execute(&mut self.main_world);

            self.write_resource_default::<FrameRateKeeper>()
                .wait_next_frame();

            let duration = self.read_resource_default::<Timer>().get();
            self.write_resource_default::<Time>()
                .set_delta_duration(duration);

            self.reset_timer();
        }
    }

    fn reset_timer(&mut self) {
        self.write_resource_default::<Timer>().restart();
        self.write_resource_default::<FrameRateKeeper>().reset();
    }

    fn is_running(&self) -> bool {
        // TODO
        true
    }

    fn read_resource_default<R: Resource + Default>(&mut self) -> Fetch<'_, R> {
        self.main_world.resources.get_or_default::<R>().unwrap()
    }

    fn write_resource_default<R: Resource + Default>(&mut self) -> FetchMut<'_, R> {
        self.main_world.resources.get_mut_or_default::<R>().unwrap()
    }
}

impl std::fmt::Debug for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Application")
            .field("universe", &self.universe)
            .field("main_world", &self.main_world.id())
            .field("systems", &std::any::type_name::<Schedule>())
            .finish()
    }
}

/// Builder for [`Application`]
///
/// [`Application`]: ./struct.Application.html
pub struct ApplicationBuilder {
    universe: Universe,
    main_world: World,
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        ApplicationBuilder::new()
    }
}

impl ApplicationBuilder {
    /// Create a new instance with bare minimum foundation
    pub fn new() -> Self {
        let universe = Universe::new();
        let mut main_world = universe.create_world();

        main_world.resources.insert(Time::new());
        main_world.resources.insert(Timer::new());
        main_world.resources.insert(FrameRateKeeper::new());

        ApplicationBuilder {
            universe,
            main_world,
        }
    }

    /// Apply custom configuration of frame rate
    pub fn with_frame_rate_config(mut self, config: FrameRateConfig) -> Self {
        self.main_world
            .resources
            .insert(FrameRateKeeper::from_config(config));
        self
    }

    /// Add the given resource to the main world
    ///
    /// This silently overwrites the previously stored resource with the same type if it exists.
    pub fn add_resource<R: Resource>(mut self, resource: R) -> Self {
        self.main_world.resources.insert(resource);
        self
    }

    /// Build and create a new instance of the app from the given systems
    pub fn build(self, systems: Schedule) -> Application {
        let ApplicationBuilder {
            universe,
            main_world,
        } = self;

        Application {
            universe,
            main_world,
            systems,
        }
    }
}

impl std::fmt::Debug for ApplicationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplicationBuilder")
            .field("universe", &self.universe)
            .field("main_world", &self.main_world.id())
            .finish()
    }
}
