//! Defines the config structure. All values are required and have explicit
//! defaults. Use the `load` function to merge together all partial config
//! sources into a fully defined config.

use std::path::PathBuf;

use anyhow::{Context, Result};
use figment::Figment;
use figment::providers::{Format, Serialized, Toml};
use serde::{Deserialize, Serialize};

use crate::cli::Cli;
use crate::config::partial::{PageWhen, PartialConfig};

pub mod partial;

/// Creates a vec of `String`s defined as string literals. Makes use of the
/// existing `vec!` macro
macro_rules! string_vec {
  ($($item:literal),* $(,)?) => (vec![$($item.to_string()),*]);
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
  pub style: Style,
  pub long: bool,
  pub ignore: bool,
  pub eza: EzaConfig,
  pub pager: PagerConfig,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      style: Default::default(),
      long: Default::default(),
      ignore: true,
      eza: Default::default(),
      pager: Default::default(),
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, clap::ValueEnum, PartialEq)]
#[serde(rename_all = "lowercase")]
/// In eza, grid and tree style are exclusive
pub enum Style {
  /// display style wasn't set in lx, pass nothing to eza
  #[default]
  Unset,
  Grid,
  Tree,
  Oneline,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EzaConfig {
  /// Args that are always forwarded to eza
  pub args: Vec<String>,

  /// Args that are forwarded when a pager is used
  pub interactive_args: Vec<String>,
}

impl Default for EzaConfig {
  fn default() -> Self {
    Self {
      args: string_vec![
        "-a",
        "--icons=always",
        "--color-scale=all",
        "--color-scale-mode=gradient",
        "--header",
        "--binary",
        "--group",
        "--git",
        "--level=5",
      ],
      interactive_args: string_vec!["--color=always"],
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PagerConfig {
  pub when: PageWhen,
  pub bin: String,
  pub args: Vec<String>,
}

impl Default for PagerConfig {
  fn default() -> Self {
    Self {
      when: Default::default(),
      bin: "less".into(),
      // -F = don't use pager if output fits in one screen, just print
      // -R = print escape sequences as-is (preserves formatting/colors in terminal)
      args: string_vec!["-FR"],
    }
  }
}

fn config_path() -> Result<Option<PathBuf>> {
  let path = dirs::config_dir()
    .context("Failed to find default config directory")?
    .join("lx")
    .join("config.toml");

  Ok(if !path.exists() { None } else { Some(path) })
}

/// Loads config sources and merges them together. Returns a well defined config
/// struct.
pub fn load(cli: &Cli) -> Result<Config> {
  // load with defaults (non-partial, every config option must have a default)
  let mut figment = Figment::new().merge(Serialized::defaults(Config::default()));

  // merge with config file (partial, unset options should not override values)
  if let Some(path) = config_path()? {
    figment = figment.merge(Toml::file(path));
  };

  // merge with cli options (partial, unset options should not override values)
  figment = figment.merge(Serialized::defaults(PartialConfig::from(cli)));

  let config = figment.extract()?;
  Ok(config)
}
