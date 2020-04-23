use game_engine::app::{init_logger, Application};
use game_engine::core::frame_rate::{build_fps_count_system, FpsValue};
use game_engine::core::systems::{ScheduleBuilder, SystemOrder};
use game_engine::ecs::prelude::*;

fn build_fps_log_system(_: &mut World) -> Box<dyn Schedulable> {
    SystemBuilder::new("LogFps")
        .read_resource::<FpsValue>()
        .build(|_, _, res, _| {
            log::debug!("Average fps: {}", res.average_fps());
            log::debug!("Last fps: {}", res.last_fps());
        })
}

fn main() {
    init_logger().unwrap_or_else(|err| panic!("Failed to init logger: {}", err));

    let schedule_builder = ScheduleBuilder::new()
        .with_system_create_fn(SystemOrder::numbered(0, 0), build_fps_count_system)
        .with_system_create_fn(SystemOrder::numbered(0, 0), build_fps_log_system);

    let mut app = Application::builder().build(schedule_builder);

    app.run();
}
