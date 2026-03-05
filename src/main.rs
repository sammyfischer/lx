#![feature(exit_status_error)]

use std::process::{Command, Stdio};

use clap::Parser;

use crate::cli::Cli;
use crate::config::Config;

mod cli;
mod config;

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

#[derive(Debug)]
#[repr(u16)]
pub enum CliError {
  EzaFailed(String) = 1,
  PagerFailed(String),
  Config(String),
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
    .stdout(if config.interactive {
      // pipe into pager
      Stdio::piped()
    } else {
      // print normally
      Stdio::inherit()
    })
    .spawn()
    .map_err(|e| CliError::EzaFailed(format!("{}", e)))?;

  if config.interactive {
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
    config::Style::Grid => args.push("--grid".into()),
    config::Style::Tree => {
      args.push("--tree".into());
      for arg in &config.eza.tree_args {
        args.push(arg.into());
      }
    }
  }

  if config.long {
    args.push("-l".into());
    for arg in &config.eza.long_args {
      args.push(arg.into());
    }
  }

  if config.interactive {
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
fn dry_run(config: &Config, eza_args: &Vec<String>) -> String {
  let mut buf = String::new();

  buf.push_str(&format!(
    r"Configured eza args:
  {}
",
    eza_args.join("\n  "),
  ));

  if config.interactive {
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

  buf.push_str(
    r"

Run yourself:
",
  );

  let mut eza_cmd = "eza".to_string();
  if eza_args.len() != 0 {
    eza_cmd.push_str(&format!(" {}", eza_args.join(" ")));
  }
  buf.push_str(&eza_cmd);

  if config.interactive {
    let mut pager_cmd = config.pager.bin.clone();
    if config.pager.args.len() != 0 {
      pager_cmd.push_str(&format!(" {}", config.pager.args.join(" ")));
    }
    buf.push_str(&format!(" | {}", pager_cmd));
  }

  buf
}
