/// Temporary function to initialize logger
///
/// This will likely change in the future to provide configuring
pub fn init_logger() -> Result<(), log::SetLoggerError> {
    fern::Dispatch::new()
        .format(|out, msg, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                msg
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
}
