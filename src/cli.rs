use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
  /// Display in grid style
  #[arg(short, long, overrides_with = "tree")]
  grid: bool,

  /// Display in tree style
  #[arg(short, long, overrides_with = "grid")]
  tree: bool,

  /// Long listing
  #[arg(short, long)]
  pub long: bool,

  /// Interactive mode
  #[arg(short, long)]
  pub interactive: bool,

  /// The directory to list
  pub dir: Option<String>,
}

#[derive(Debug)]
/// In eza, grid and tree style are exclusive
pub enum Style {
  Grid,
  Tree,
}

impl Cli {
  pub fn style(&self) -> Style {
    if self.tree { Style::Tree } else { Style::Grid }
  }
}
