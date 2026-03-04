//! Defines the config structure. All values are required and have explicit defaults. Use the `load`
//! function to merge together all partial config sources into a fully defined config.

use std::path::PathBuf;

use figment::Figment;
use figment::providers::{Format, Serialized, Toml};
use serde::{Deserialize, Serialize};

use crate::cli::Cli;
use crate::config::partial::PartialConfig;
use crate::{CliError, CliResult};

pub mod partial;

/// Creates a vec of `String`s defined as string literals. Makes use of the existing `vec!` macro
macro_rules! string_vec {
  ($($item:literal),* $(,)?) => (vec![$($item.to_string()),*]);
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
  #[serde(default)]
  pub style: Style,

  #[serde(default)]
  pub long: bool,

  #[serde(default)]
  pub interactive: bool,

  #[serde(default)]
  pub eza: EzaConfig,

  #[serde(default)]
  pub pager: PagerConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
/// In eza, grid and tree style are exclusive
pub enum Style {
  Grid,
  Tree,
}

impl Default for Style {
  fn default() -> Self {
    Self::Grid
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EzaConfig {
  /// Args that are always forwarded to eza
  pub args: Vec<String>,
  /// Args that are forwarded in long mode only
  pub long_args: Vec<String>,
  /// Args that are forwarded in tree mode only
  pub tree_args: Vec<String>,
  /// Args that are forwarded in interactive mode only
  pub interactive_args: Vec<String>,
}

impl Default for EzaConfig {
  fn default() -> Self {
    Self {
      args: string_vec![
        "-a",
        "--git-ignore",
        "--icons=always",
        "--color-scale=all",
        "--color-scale-mode=gradient",
      ],
      long_args: string_vec!["--header", "--binary", "--group", "--git"],
      tree_args: string_vec!["--level=5"],
      interactive_args: string_vec!["--color=always"],
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PagerConfig {
  pub bin: String,
  pub args: Vec<String>,
}

impl Default for PagerConfig {
  fn default() -> Self {
    Self {
      bin: "less".into(),
      args: string_vec!["-R"],
    }
  }
}

fn config_path() -> Option<PathBuf> {
  let dir = directories::ProjectDirs::from("", "", "lx")?;
  Some(dir.config_dir().join("config.toml"))
}

/// Loads config sources and merges them together. Returns a well defined config struct.
pub fn load(cli: &Cli) -> CliResult<Config> {
  // load with defaults (non-partial, every config option must have a default)
  let mut figment = Figment::new().merge(Serialized::defaults(Config::default()));

  // merge with config file (partial, unset options should not override values)
  if let Some(path) = config_path() {
    figment = figment.merge(Toml::file(path));
  };

  // merge with cli options (partial, unset options should not override values)
  figment = figment.merge(Serialized::defaults(PartialConfig::from(cli)));

  figment
    .extract()
    .map_err(|e| CliError::Config(format!("{}", e)))
}
