#![feature(exit_status_error)]

use std::process::{Command, Stdio};

use clap::Parser;

use crate::cli::Cli;

mod cli;

/// Await a child process and forward a particular error using try-expression
macro_rules! await_child {
  ($child:expr, $err:expr) => {
    $child
      .wait()
      .map_err(|_| $err)?
      .exit_ok()
      .map_err(|_| $err)?;
  };
}

#[derive(Debug)]
enum CliError {
  EzaFailed = 1,
  PagerFailed,
}

type CliResult<T = ()> = Result<T, CliError>;

fn main() -> CliResult {
  let cli = Cli::parse();

  let forward_args = forward_args(&cli);

  let mut eza_proc = Command::new("eza")
    .args(forward_args)
    .stdout(if cli.interactive {
      // pipe into pager
      Stdio::piped()
    } else {
      // print normally
      Stdio::inherit()
    })
    .spawn()
    .map_err(|_| CliError::EzaFailed)?;

  if cli.interactive {
    // grab and redirect stdout to less
    let eza_out = eza_proc.stdout.take().ok_or(CliError::EzaFailed)?;

    let pager_args = vec!["-R"];
    // TODO: header is only supported in less 600, which is prerelease
    // if cli.long {
    //   long listing prints a header, make it stick to top of screen in less
    //   pager_args.push("--header=1");
    // }

    let mut pager_proc = Command::new("less")
      .stdin(eza_out)
      .args(pager_args)
      .spawn()
      .map_err(|_| CliError::PagerFailed)?;

    // wait on less
    await_child!(pager_proc, CliError::PagerFailed);
  }

  // wait on eza
  await_child!(eza_proc, CliError::EzaFailed);

  Ok(())
}

/// Creates a list of args to forward to eza
fn forward_args(cli: &Cli) -> Vec<String> {
  let default_args = vec![
    "-a",
    "--git-ignore",
    "--icons=always",
    "--color-scale=all",
    "--color-scale-mode=gradient",
    // long-list args
    "--header",
    "--binary",
    "--group",
    "--git",
    // tree args
    "--level=5",
  ];

  let mut forward_args: Vec<String> = Vec::new();
  for arg in default_args {
    forward_args.push(arg.into());
  }

  match cli.style() {
    cli::Style::Grid => forward_args.push("--grid".into()),
    cli::Style::Tree => forward_args.push("--tree".into()),
  }

  if cli.long {
    forward_args.push("-l".into());
  }

  if cli.interactive {
    // color is automatically disabled by eza when stdout isn't a tty
    forward_args.push("--color=always".into());
  }

  if let Some(dir) = &cli.dir {
    forward_args.push(dir.clone());
  }

  forward_args
}
