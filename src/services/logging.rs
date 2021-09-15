use anyhow::Result;
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

        let logger = fern::Dispatch::new().level(level).chain(std::io::stdout());

        logger.apply()?;

        Ok(())
    }
}
