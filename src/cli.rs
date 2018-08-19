use clap_flags;
use failure::ResultExt;
use structopt;

/// Command line parser.
#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Cli {
  #[structopt(flatten)]
  logger: clap_flags::Log,
  #[structopt(flatten)]
  verbosity: clap_flags::Verbosity,
  #[structopt(default_value = ".")]
  path: String,
}

impl Cli {
  /// Initialize a logger.
  #[inline]
  pub fn log(&self, name: &str) -> ::Result<()> {
    self
      .logger
      .log(self.verbosity.log_level(), name)
      .context(::ErrorKind::Log)?;
    Ok(())
  }

  /// Access the path.
  #[inline]
  pub fn path(&self) -> &str {
    &self.path
  }
}
