//! logger library
use flexi_logger::Logger;
use flexi_logger::{AdaptiveFormat, Cleanup, Criterion, FileSpec, Naming};
pub use log;
use log::{Level, LevelFilter, Metadata, Record};

mod err;

pub use err::{Error, Result};

/// initialize a simple logger
pub fn simple() -> Result<()> {
  log::set_logger(&SimpleLogger).map(|()| log::set_max_level(LevelFilter::Trace))?;
  Ok(())
}

/// a simple logger
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

/// Initialize a `Logger` with a specified logging level
pub fn flexi(level: &str) -> Result<()> {
  Logger::try_with_env_or_str(level)?
    .format(flexi_logger::colored_default_format)
    .set_palette("196;208;50;7;8".to_string())
    .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
    .adaptive_format_for_stdout(AdaptiveFormat::Default)
    .log_to_stdout()
    .start()?;
  Ok(())
}

/// Initialize `Logger` and log to a file instead of stdout
pub fn file(env: &str, log_path: &str, log_name: &str) -> Result<()> {
  Logger::try_with_env_or_str(env)? // Write all messages
    .log_to_file(
      FileSpec::default()
        .directory(log_path)
        .basename(log_name)
        .suffix("log"),
    )
    .print_message()
    .append()
    .rotate(
      Criterion::Size(1024 * 1000 * 1),
      Naming::Numbers,
      Cleanup::Never,
    )
    .start()?;
  Ok(())
}

#[cfg(test)]
mod tests;
