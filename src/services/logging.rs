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
                    "[{}]{} {}",
                    colors.color(record.level()),
                    chrono::Utc::now().format("[%Y-%m-%d %H:%M:%S]"),
                    message
                ))
            })
            .level(level)
            .chain(std::io::stdout());

        logger.apply()?;

        Ok(())
    }
}
