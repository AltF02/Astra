use anyhow::Result;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

pub trait Logger {
    fn setup() -> Result<()>;
}

impl Logger for fern::Dispatch {
    fn setup() -> Result<()> {
        #[cfg(not(debug_assertions))]
        let level = LevelFilter::Info;
        #[cfg(debug_assertions)]
        let level = LevelFilter::Debug;

        let colors = ColoredLevelConfig::new().debug(Color::Magenta);

        let logger = fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{}][{}]{} {}",
                    colors.color(record.level()),
                    record.target(),
                    chrono::Utc::now().format("[%Y-%m-%d %H:%M:%S]"),
                    message
                ))
            })
            .level(level)
            .level_for("tracing", LevelFilter::Warn)
            .level_for("sqlx", LevelFilter::Warn)
            .level_for("serenity::client::dispatch", LevelFilter::Warn)
            .level_for("reqwest", LevelFilter::Debug);

        logger.chain(std::io::stdout()).apply()?;

        Ok(())
    }
}
