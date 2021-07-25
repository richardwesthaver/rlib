use flexi_logger::Logger;
use flexi_logger::{AdaptiveFormat, Age, Cleanup, Criterion, FileSpec, Naming};
pub use log;
use log::{Level, LevelFilter, Metadata, Record};

mod err;

pub use crate::err::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn simple() -> Result<()> {
  log::set_logger(&SimpleLogger).map(|()| log::set_max_level(LevelFilter::Trace))?;
  Ok(())
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      println!("{} - {}", record.level(), record.args());
    }
  }

  fn flush(&self) {}
}

pub fn flexi() -> Result<()> {
  Logger::try_with_env_or_str("trace")?
    .format(flexi_logger::colored_default_format)
    .set_palette("196;208;22;7;8".to_string())
    .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
    .adaptive_format_for_stdout(AdaptiveFormat::Default)
    .log_to_stdout()
    .start()?;
  Ok(())
}

pub fn file() -> Result<()> {
  Logger::try_with_str("trace")? // Write all error, warn, and info messages
    // use a simple filename without a timestamp
    .log_to_file(FileSpec::default())
    .rotate(Criterion::Age(Age::Day), Naming::Timestamps, Cleanup::Never)
    .append()
    .start()?;

  Ok(())
}

#[cfg(test)]
mod tests;
