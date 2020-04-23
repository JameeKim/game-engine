use crate::core::systems::{ScheduleBuilder, SystemBundle, SystemOrder};
use crate::ecs::world::World;
use crate::wm::{EventsLoop, Window};
use crate::{create_window_events_system, create_window_size_control_system};

/// A system bundle for window managing
#[derive(Debug)]
pub struct WindowBundle {
    /// The system execution order of the [`WindowSizeControl`] system
    ///
    /// [`WindowSizeControl`]: ./fn.create_window_size_control_system.html
    pub size_control_order: SystemOrder,
    /// The system execution order of the [`WindowEvents`] system
    ///
    /// [`WindowEvents`]: ./fn.create_window_events_system.html
    pub events_loop_order: SystemOrder,
    /// The events loop to use for the systems
    ///
    /// If not given, this bundle will create one for you.
    pub events_loop: Option<EventsLoop>,
    /// The window to use for the systems
    ///
    /// If not given, this bundle will create one for you with the given events loop or also a new
    /// one.
    pub window: Option<Window>,
}

impl Default for WindowBundle {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowBundle {
    /// Create a new instance with arbitrary system execution orders given
    ///
    /// You most likely want to tweak the values with other methods. This is a convenience method
    /// for chaining methods to build your instance.
    pub fn new() -> Self {
        Self {
            size_control_order: SystemOrder::numbered(10, 0),
            events_loop_order: SystemOrder::from_last_number(0),
            events_loop: None,
            window: None,
        }
    }

    /// Create a new instance with the given system execution orders
    pub fn from_orders(size_control_order: SystemOrder, events_loop_order: SystemOrder) -> Self {
        Self::new()
            .with_size_control_order(size_control_order)
            .with_events_loop_order(events_loop_order)
    }

    /// Set the system order of the [`WindowSizeControl`] system
    ///
    /// [`WindowSizeControl`]: ./fn.create_window_size_control_system.html
    pub fn with_size_control_order(mut self, order: SystemOrder) -> Self {
        self.size_control_order = order;
        self
    }

    /// Set the system order of [`WindowEvents`] system
    ///
    /// [`WindowEvents`]: ./fn.create_window_events_system.html
    pub fn with_events_loop_order(mut self, order: SystemOrder) -> Self {
        self.events_loop_order = order;
        self
    }

    /// Add an existing events loop to be used
    pub fn with_events_loop(mut self, events_loop: EventsLoop) -> Self {
        self.events_loop.replace(events_loop);
        self
    }

    /// Add an existing window to be used
    pub fn with_window(mut self, window: Window) -> Self {
        self.window.replace(window);
        self
    }
}

impl SystemBundle for WindowBundle {
    fn build_systems(self, _world: &mut World, builder: &mut ScheduleBuilder<'_>) {
        let events_loop = self.events_loop.unwrap_or_else(EventsLoop::new);
        let window = self
            .window
            .unwrap_or_else(|| Window::new(&events_loop).expect("Unable to create a new window"));

        builder.add_thread_local_system_create_fn(
            self.events_loop_order,
            create_window_events_system(events_loop),
        );
        builder.add_system_create_fn(
            self.size_control_order,
            create_window_size_control_system(window),
        );
    }
}
