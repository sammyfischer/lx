#![feature(exit_status_error)]

use std::io::{IsTerminal, stdout};
use std::process::{Command, Stdio};

use anyhow::{Result, anyhow};
use clap::Parser;

use crate::cli::Cli;
use crate::config::partial::PageWhen;
use crate::config::{Config, Style};

mod cli;
mod config;

fn main() -> Result<()> {
  let cli = Cli::parse();
  let config = config::load(&cli)?;
  let eza_args = eza_args(&config, &cli.args);

  if cli.debug {
    println!("{}", dry_run(&config, &eza_args));
    return Ok(());
  }

  let mut eza = Command::new("eza")
    .args(eza_args)
    .stdout(if should_use_pager(&config) {
      // pipe into pager
      Stdio::piped()
    } else {
      // print normally
      Stdio::inherit()
    })
    .spawn()?;

  if should_use_pager(&config) {
    // grab and redirect stdout to pager
    let eza_out = eza
      .stdout
      .take()
      .ok_or(anyhow!("Failed to get eza output"))?;

    let mut pager = Command::new(config.pager.bin)
      .stdin(eza_out)
      .args(config.pager.args)
      .spawn()?;

    // wait on pager
    if let Err(e) = pager.wait()?.exit_ok() {
      eprintln!("{}", e);
    };
  }

  // wait on eza
  if let Err(e) = eza.wait()?.exit_ok() {
    eprintln!("{}", e);
  }

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

  if config.ignore {
    args.push("--git-ignore".into());
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

/// Creates a descriptive output representing what the cli would do if it
/// actually ran
fn dry_run(config: &Config, eza_args: &[String]) -> String {
  let mut buf = String::new();

  buf.push_str(&format!("eza\n  {}", eza_args.join("\n  "),));

  if should_use_pager(config) {
    buf.push_str(&format!(
      r"
{}
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
  if config.pager.when == PageWhen::Always {
    return true;
  }

  if config.pager.when == PageWhen::Never {
    return false;
  }

  // automatic paging

  if config.style == Style::Grid || config.style == Style::Unset {
    return false;
  }

  stdout().is_terminal()
}
