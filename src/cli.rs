use clap::Parser;

use crate::config::Style;
use crate::config::partial::{PartialConfig, PartialEzaConfig, PartialPagerConfig};

#[derive(Debug, Parser)]
pub struct Cli {
  /// Displays a shell-like representation of the processes that would be run
  #[arg(long = "dry-run", visible_alias = "dry")]
  pub dry_run: bool,

  /// Display in grid style
  #[arg(
    short,
    long,
    num_args = 0..=1,
    require_equals = true,
    default_missing_value = "true",
    overrides_with = "tree"
  )]
  grid: Option<bool>,

  /// Display in tree style
  #[arg(
    short,
    long,
    num_args = 0..=1,
    require_equals = true,
    default_missing_value = "true",
    overrides_with = "grid"
  )]
  tree: Option<bool>,

  /// Long listing
  #[arg(short, long, num_args = 0..=1, require_equals = true, default_missing_value = "true")]
  pub long: Option<bool>,

  /// Interactive mode
  #[arg(short, long, num_args = 0..=1, require_equals = true, default_missing_value = "true")]
  pub interactive: Option<bool>,

  /// Remaining args, which get forwarded to eza
  #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
  pub rest: Vec<String>,
}

impl From<&Cli> for PartialConfig {
  /// Create a partial config from cli options.
  fn from(value: &Cli) -> Self {
    // `-g` and `-t` override each other in the command line. Only the last one set can have a
    // `Some` value.
    //
    // If it was explicitly set to `true`, use that display style.
    // If it was explicitly set to `false`, use the other display style.
    // If neither were set, use `None` so it defaults to the config file or default value
    let style = if let Some(yes) = value.tree {
      if yes {
        Some(Style::Tree)
      } else {
        Some(Style::Grid)
      }
    } else if let Some(yes) = value.grid {
      if yes {
        Some(Style::Grid)
      } else {
        Some(Style::Tree)
      }
    } else {
      None
    };

    PartialConfig {
      style,
      long: value.long,
      interactive: value.interactive,
      // eza and pager args can't be specified by command line, just use defaults
      eza: PartialEzaConfig::default(),
      pager: PartialPagerConfig::default(),
    }
  }
}
