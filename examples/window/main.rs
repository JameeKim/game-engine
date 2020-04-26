use game_engine::app::{init_logger, Application};
use game_engine::core::systems::{ScheduleBuilder, SystemOrder};
use game_engine::ecs::prelude::*;
use game_engine::event_channel::EventChannel;
use game_engine::window::wm::{ElementState, Event, WindowEvent};
use game_engine::window::WindowBundle;

fn build_window_event_log_system(world: &mut World) -> Box<dyn Schedulable> {
    let mut reader = world
        .resources
        .get_mut::<EventChannel<Event>>()
        .expect("event channel not inserted")
        .register_reader();

    SystemBuilder::new("WindowEventLog")
        .read_resource::<EventChannel<Event>>()
        .build(move |_cmd, _world, resources, _queries| {
            let channel = &*resources;

            for event in channel.read(&mut reader) {
                log_window_event(event);
            }
        })
}

fn main() {
    init_logger().expect("Logger init failed");

    let schedule_builder = ScheduleBuilder::new()
        .with_system_bundle(WindowBundle::new())
        .with_system_create_fn(SystemOrder::numbered(0, 0), build_window_event_log_system);

    let mut app = Application::builder().build(schedule_builder);

    app.run();
}

fn log_window_event(event: &Event) {
    if let Event::WindowEvent { event, .. } = event {
        match event {
            WindowEvent::Resized(size) => {
                log::debug!("Window sized changed to ({}, {})", size.width, size.height);
            }
            WindowEvent::CloseRequested => {
                log::info!("Window close is requested!");
            }
            WindowEvent::Destroyed => {
                log::info!("Window is destroyed!");
            }
            WindowEvent::ReceivedCharacter(c) => {
                log::debug!("Received character '{}'", c);
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                log::debug!("Mouse button '{:?}' is pressed", button);
            }
            _ => {}
        }
    }
}
