use clap::{ArgGroup, Parser};

use crate::config::Style;
use crate::config::partial::{PageWhen, PartialConfig, PartialEzaConfig, PartialPagerConfig};

#[derive(Debug, Parser)]
#[command(group(
  ArgGroup::new("style")
    .args(["grid", "tree", "oneline"])
))]
pub struct Cli {
  /// Describes what lx will do with the given configuration. Useful to test
  /// your config.
  #[arg(short, long)]
  pub debug: bool,

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

  /// Respect gitignore
  #[arg(
    short,
    long,
    num_args = 0..=1,
    require_equals = true,
    default_missing_value = "true"
  )]
  pub ignore: Option<bool>,

  /// When to page output
  #[arg(short, long, default_value = "auto")]
  pub paging: PageWhen,

  /// Remaining args, which get forwarded to eza
  pub args: Vec<String>,
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

    let mut pager = PartialPagerConfig::default();
    pager.when = value.paging;

    PartialConfig {
      style,
      long: if value.long { Some(true) } else { None },
      ignore: value.ignore,
      eza: PartialEzaConfig::default(),
      pager,
    }
  }
}
