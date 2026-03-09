#[derive(Debug)]
#[repr(u16)]
pub enum CliError {
  EzaFailed(String) = 1,
  PagerFailed(String),
  Config(String),
}

impl From<figment::Error> for CliError {
  fn from(value: figment::Error) -> Self {
    CliError::Config(format!("{}", value))
  }
}
