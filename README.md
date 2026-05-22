# ez

My personal wrapper for the `eza` command line utility.

## Feature Overview

- better defaults
- automatic paging
- config file

## Install

Clone this repository anywhere. If you have just, you can do `just install`, otherwise run `cargo install --path .`. Requires rust to be installed.

## Usage

### Dependencies

Requires eza and a pager to be found in PATH. The default pager is less but can be configured.

### Command line

> Tip: `ez --help` will display the help message. `ez -- --help` will display eza's help message.

There are a only a few command line options:

- `-g` Display in grid mode. Sets `--grid` when calling eza.
- `-t` Display in tree mode. Sets `--tree` when calling eza.
- `-1` Display in single line mode. Sets `--oneline` when calling eza.
- `-l` Enable long-listing. Sets `--long` when calling eza.
- `-i` Respect `.gitignore`. Sets `--git-ignore` when calling eza.
- `-p <WHEN>` When to page output. Can be `auto`, `always`, or `never`.
- `-debug` Enable debug mode. Describes the current configuration with all config and options merged, but doesn't actually call eza.

Some other notes:

- `-g`, `-t`, and `-1` are exclusive. Only one may be set, or none.
  - The default display mode is handled by eza.
  - If no display mode is set, it defaults to grid.
  - If no display mode is set AND `-l` is set, then oneline is used.
  - eza supports specifying both `-g` and `-l`.
- In debug mode, if you don't see the pager command/args listed, that means ez decided not to page output.

If you want to override any eza options manually, pass them in after `--`:

```bash
ez -t -- --color=never
ez ~/.Downloads -- --hyperlink
ez -l ~/.config -- -o
```

### Config file

You can create a user config file in the standard location for your platform:

- linux: `$HOME/.config/ez/config.toml`
- windows: `$HOME\AppData\Roaming\ez\config.toml`
- mac: `$HOME/Library/Application Support/ez/config.toml`

These options override defaults, but cli options override everything.

An example config file can be found [here](config.toml), with all the default values.

## Motivation

### Defaults

eza has a lot of options, so I wanted my favorite defaults to be available automatically without using aliases.

### Pager support

eza and ls don't support automatic paging with colors, which is a common feature for modern user-focused cli tools.

### Config file

I prefer to have a lighter `.bashrc` when convenient, since it has to run every time bash is started. It's better to have a config file that's only loaded when the specific tool is used.

### Why eza?

I'm currently using eza as my ls alternative. It has a lot more features than just being a basic ls wrapper (e.g. tree display, gradient coloring, icons), so I decided not to reinvent the wheel.
