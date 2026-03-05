use clap::{ArgGroup, Parser};

use crate::config::Style;
use crate::config::partial::{PartialConfig, PartialEzaConfig, PartialPagerConfig};

#[derive(Debug, Parser)]
#[command(group(
  ArgGroup::new("style")
    .args(["grid", "tree", "oneline"])
))]
pub struct Cli {
  /// Describes what lx will do with the given configuration
  #[arg(long = "dry-run", visible_alias = "dry")]
  pub dry_run: bool,

  /// Display in grid style
  #[arg(short, long)]
  grid: bool,

  /// Display in tree style
  #[arg(short, long)]
  tree: bool,

  /// Display in single-line style
  #[arg(short = '1', long)]
  oneline: bool,

  /// Long listing
  #[arg(short, long)]
  pub long: bool,

  /// Interactive mode
  #[arg(short, long)]
  pub interactive: bool,

  /// Remaining args, which get forwarded to eza
  #[arg(
    trailing_var_arg = true,
    allow_hyphen_values = true,
    value_name = "ARGS"
  )]
  pub rest: Vec<String>,
}

impl From<&Cli> for PartialConfig {
  /// Create a partial config from cli options.
  fn from(value: &Cli) -> Self {
    let style = if value.tree {
      Some(Style::Tree)
    } else if value.grid {
      Some(Style::Grid)
    } else if value.oneline {
      Some(Style::Oneline)
    } else {
      None
    };

    PartialConfig {
      style,
      long: if value.long { Some(true) } else { None },
      interactive: if value.interactive { Some(true) } else { None },
      // eza and pager args can't be specified by command line, just use defaults
      eza: PartialEzaConfig::default(),
      pager: PartialPagerConfig::default(),
    }
  }
}
