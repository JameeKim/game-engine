use game_engine::app::{init_logger, Application};
use game_engine::core::frame_rate::build_fps_count_system;
use game_engine::ecs::schedule::Schedule;
use game_engine::ecs::system::SystemBuilder;
use game_engine_core::frame_rate::FpsValue;

fn main() {
    init_logger().unwrap_or_else(|err| panic!("Failed to init logger: {}", err));

    let schedule = Schedule::builder()
        .add_system(build_fps_count_system())
        .add_system(
            SystemBuilder::new("LogFps")
                .read_resource::<FpsValue>()
                .build(|_, _, res, _| {
                    log::debug!("Average fps: {}", res.average_fps());
                    log::debug!("Last fps: {}", res.last_fps());
                }),
        )
        .build();

    let mut app = Application::builder()
        .add_resource(FpsValue::new())
        .build(schedule);

    app.run();
}
