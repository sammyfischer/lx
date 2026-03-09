#![feature(exit_status_error)]

use std::process::{Command, Stdio};

use clap::Parser;

use crate::cli::Cli;
use crate::config::{Config, Style};
use crate::error::CliError;

mod cli;
mod config;
mod error;

/// Await a child process and forward a particular error using try-expression
macro_rules! await_child {
  ($child:expr, $err:expr) => {
    $child
      .wait()
      .map_err(|e| $err(format!("{}", e)))?
      .exit_ok()
      .map_err(|e| $err(format!("{}", e)))?;
  };
}

pub type CliResult<T = ()> = Result<T, CliError>;

fn main() -> CliResult {
  let cli = Cli::parse();
  let config = config::load(&cli)?;
  let eza_args = eza_args(&config, &cli.rest);

  if cli.dry_run {
    println!("{}", dry_run(&config, &eza_args));
    return Ok(());
  }

  let mut eza_proc = Command::new("eza")
    .args(eza_args)
    .stdout(if should_use_pager(&config) {
      // pipe into pager
      Stdio::piped()
    } else {
      // print normally
      Stdio::inherit()
    })
    .spawn()
    .map_err(|e| CliError::EzaFailed(format!("{}", e)))?;

  if should_use_pager(&config) {
    // grab and redirect stdout to less
    let eza_out = eza_proc
      .stdout
      .take()
      .ok_or(CliError::EzaFailed("Failed to get eza output".into()))?;

    let mut pager_proc = Command::new(config.pager.bin)
      .stdin(eza_out)
      .args(config.pager.args)
      .spawn()
      .map_err(|e| CliError::PagerFailed(format!("{}", e)))?;

    // wait on pager
    await_child!(pager_proc, CliError::PagerFailed);
  }

  // wait on eza
  await_child!(eza_proc, CliError::EzaFailed);

  Ok(())
}

/// Creates a list of args to forward to eza
fn eza_args(config: &Config, rest: &Vec<String>) -> Vec<String> {
  let mut args = config.eza.args.clone();

  match config.style {
    config::Style::Unset => (),
    config::Style::Grid => args.push("--grid".into()),
    config::Style::Tree => args.push("--tree".into()),
    config::Style::Oneline => args.push("--oneline".into()),
  }

  if config.long {
    args.push("--long".into());
  }

  if should_use_pager(config) {
    for arg in &config.eza.interactive_args {
      args.push(arg.into());
    }
  }

  for arg in rest {
    args.push(arg.into());
  }

  args
}

/// Creates a descriptive output representing what the cli would do if it actually ran
fn dry_run(config: &Config, eza_args: &[String]) -> String {
  let mut buf = String::new();

  buf.push_str(&format!(
    r"Configured eza args:
  {}",
    eza_args.join("\n  "),
  ));

  if should_use_pager(config) {
    buf.push_str(&format!(
      r"
[Interactive mode options]
Pager: {}
Pager args:
  {}",
      config.pager.bin,
      config.pager.args.join("\n  ")
    ));
  }

  buf.push_str("\n\nRun yourself:\n");

  let mut eza_cmd = "eza".to_string();
  if !eza_args.is_empty() {
    eza_cmd.push_str(&format!(" {}", eza_args.join(" ")));
  }
  buf.push_str(&eza_cmd);

  if should_use_pager(config) {
    let mut pager_cmd = config.pager.bin.clone();
    if !config.pager.args.is_empty() {
      pager_cmd.push_str(&format!(" {}", config.pager.args.join(" ")));
    }
    buf.push_str(&format!(" | {}", pager_cmd));
  }

  buf
}

fn should_use_pager(config: &Config) -> bool {
  // user set it to false
  if !config.interactive {
    return false;
  }

  // paging breaks grid style for some reason
  if config.style == Style::Grid {
    return false;
  }

  // this is also grid style
  if config.style == Style::Unset && !config.long {
    return false;
  }

  true
}
