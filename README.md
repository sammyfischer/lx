# lx

My personal wrapper for the `eza` command line utility.

## Feature Overview

- configure default eza options for different modes (tree, long)
- more convenient pager support with color
- user config file in platform-standard location

## Usage

### Command line

> Tip: `lx -h` will display the help message

There are a only a few command line options:

- `-g` Display in grid mode. Sets `--grid` when calling eza.
- `-t` Display in tree mode. Sets `--tree` when calling eza.
- `-1` Display in tree mode. Sets `--oneline` when calling eza.
- `-l` Enable long-listing. Sets `--long` when calling eza.
- `-i` Enable interactive mode. This forwards eza output into a pager.
- `--dry-run` Enable dry-run mode. Displays an output describing how eza and the pager will be run, but does nothing.

Some other notes:

- `-g`, `-t`, and `-1` are exclusive. Only one may be set, or none.
  - The default display mode is handled by eza. If no display mode is set, it defaults to grid. If no display mode is set AND `-l` is set, then oneline is used.
- In dry-run mode, pager information is only displayed if interactive mode is enabled. Use this to debug whether interactive mode is being enabled correctly

If you want to override any eza options manually, simply pass them after any of the above options, like so:

```bash
lx -ti ~/.config
lx -ti --color=never
lx -ti --color=never ~/.Downloads
```

For clarity, you can use `--` to separate lx args from eza args. This isn't usually required but if you get some issues, it may be useful.

```bash
lx -ti -- ~/.config
lx -ti -- --color=never
lx -ti -- --hyperlink ~/.Downloads
```

### Config file

You can create a user config file in the standard location for your platform:

- linux: `$HOME/.config/lx/config.toml`
- windows: `$HOME\AppData\Roaming\lx\config.toml`
- mac: `$HOME/Library/Application Support/lx/config.toml`

These options override defaults, but cli options override everything.

An example config file can be found [here](config.toml), with all the default values.

## Motivation

### Defaults

eza has a lot of options, so I wanted my favorite defaults to be available automatically without using aliases.

### Pager support

Using eza (and ls) with pagers like less automatically remove colors from output. Additionally, less doesn't show color by default anyway. It instead prints the literal escape characters. To work around this, you need to specify `eza --color=always` and `less -R`.

First, this is complicated and messy when using bash aliases. Second, it's a lot more to type, and I would rather have a quick and easy `-i` option.

### Config file

I don't like polluting my `.bashrc` with aliases. Using a config file to specify defaults is nicer and more modular.

### Why eza?

I'm currently using eza as my ls alternative. It has a lot more features than just being a basic ls wrapper (e.g. tree display, gradient coloring), so I decided not to reinvent the wheel.
