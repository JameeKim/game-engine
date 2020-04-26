use crate::core::systems::{types, SystemDesc};
use crate::ecs::schedule::{Runnable, Schedulable};
use crate::ecs::system::SystemBuilder;
use crate::ecs::world::World;
use crate::event_channel::EventChannel;
use crate::wm::{Event, EventsLoop, Window};
use crate::WindowSize;

/// System descriptor for window events system that gathers window events from the window and pushes
/// them into the event channel resource
///
/// This is a wrapper around [`create_window_events_system`].
///
/// [`create_window_events_system`]: ./fn.create_window_events_system.html
#[derive(Debug)]
pub struct WindowEventsSystem {
    events_loop: EventsLoop,
}

impl WindowEventsSystem {
    /// Create a new instance from the given events loop
    pub fn new(events_loop: EventsLoop) -> Self {
        Self { events_loop }
    }
}

impl SystemDesc for WindowEventsSystem {
    type SystemType = types::ThreadLocal;

    fn build(self, world: &mut World) -> Box<dyn Runnable> {
        create_window_events_system(self.events_loop)(world)
    }
}

/// Create a builder of window events system that gathers window events from the window and pushes
/// them into the event channel resource
///
/// The events loop polling takes place in this system. It is a thread local system.
pub fn create_window_events_system(
    mut events_loop: EventsLoop,
) -> impl FnOnce(&mut World) -> Box<dyn Runnable> {
    |world| {
        if !world.resources.contains::<EventChannel<Event>>() {
            world
                .resources
                .insert(EventChannel::<Event>::with_capacity(128));
        }

        SystemBuilder::new("WindowEvents")
            .write_resource::<EventChannel<Event>>()
            .build_thread_local(move |_cmd, _world, resources, _queries| {
                let channel = &mut *resources;
                events_loop.poll_events(|event| channel.single_write(event));
            })
    }
}

/// System descriptor for keeping the [`WindowSize`] resource in sync with the real window instance
///
/// This is a wrapper around [`create_window_size_control_system`].
///
/// [`WindowSize`]: ./struct.WindowSize.html
/// [`create_window_size_control_system`]: ./fn.create_window_size_control_system.html
#[derive(Debug)]
pub struct WindowSizeControlSystem {
    window: Window,
}

impl WindowSizeControlSystem {
    /// Create a new instance from the given window instance
    pub fn new(window: Window) -> Self {
        Self { window }
    }
}

impl SystemDesc for WindowSizeControlSystem {
    type SystemType = types::Parallel;

    fn build(self, world: &mut World) -> Box<dyn Schedulable> {
        create_window_size_control_system(self.window)(world)
    }
}

/// Create a builder of window size control system that updates the [`WindowSize`] resource from the
/// information from the window and vice versa
///
/// This system should be executed after any systems that might request window size change.
///
/// [`WindowSize`]: ./struct.WindowSize.html
pub fn create_window_size_control_system(
    window: Window,
) -> impl FnOnce(&mut World) -> Box<dyn Schedulable> {
    |world| {
        let dpi_factor = window.get_hidpi_factor();
        let size = window
            .get_inner_size()
            .expect("window closed before initialization finished")
            .to_physical(dpi_factor);

        world.resources.insert(WindowSize::new(size, dpi_factor));
        world.resources.insert(window);

        SystemBuilder::new("WindowSizeControl")
            .read_resource::<Window>()
            .write_resource::<WindowSize>()
            .build(|_cmd, _world, resources, _queries| {
                let (ref window, ref mut size) = *resources;

                if let Some(requested_size) = size.take_requested_size() {
                    window.set_inner_size(requested_size.to_logical(size.dpi_factor()));
                }

                size.set_dpi_factor(window.get_hidpi_factor());

                if let Some(new_size) = window.get_inner_size() {
                    size.set_logical_size(new_size);
                }
            })
    }
}
