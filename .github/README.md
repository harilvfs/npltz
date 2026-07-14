<p align="center">
  <h1 align="center">npltz</h1>
</p>

<div align="center">

[![Built With Ratatui][ratatui]][ratatui-link]
[![Build Status][check]][check-link]
[![Crates.io][crates]][crates-link]
[![Downloads][downloads]][downloads-link]
[![License][license]][license-link]

</div>

**[npltz](https://github.com/harilvfs/npltz)** a terminal-based Nepali calendar (Bikram Sambat) written in Rust using the [`ratatui`](https://github.com/ratatui-org/ratatui). It's designed to be simple: view Nepali dates with AD (English) support in your terminal.

> [!WARNING]
> npltz is in its early stages. If you run into bugs, please [open an issue](https://github.com/harilvfs/npltz/issues).

<div align="center">

[Installation](#installation) •
[Usage](#usage) •
[Contributing](#contributing) •
[Changelog ⇢](https://github.com/harilvfs/npltz/blob/main/CHANGELOG.md)

</div>

## Preview

<img src="https://raw.githubusercontent.com/harilvfs/npltz/refs/heads/main/.github/preview.gif"/>

> Preview by: [vhs](https://github.com/charmbracelet/vhs)

## Installation

### Linux / macOS / Termux

```sh
curl -fsSL npltz.chalisehari.com.np/install | sh
```

The install script auto-detects your platform and installs the correct binary.

### Windows

Download the `.exe` from [releases](https://github.com/harilvfs/npltz/releases/latest) and add it to your PATH.

### Cargo Binstall (All Platforms)

Downloads pre-built binaries instead of compiling:

```sh
cargo binstall npltz
```

### Cargo Install (All Platforms)

Compiles from source (slower):

```sh
cargo install npltz
```

### Build from Source

```sh
git clone https://github.com/harilvfs/npltz.git
cd npltz
cargo build --release
./build/release/npltz
```

> [!CAUTION]
> npltz has not been tested on macOS or Windows. If you run into any issues, please [open an issue](https://github.com/harilvfs/npltz/issues).

## Usage

npltz is a simple TUI built with ratatui. Keybinds are shown on screen to navigate, switch years, go to a specific year, change themes, and more.

Press `?` inside the TUI to open the help screen with all keyboard shortcuts.

### Commands

```
npltz --help
```

#### Date Conversion

```sh
npltz convert 2024-04-13       # AD → BS
npltz convert-bs 2081-01-01    # BS → AD
npltz show                     # Today's date
npltz show --date 2024-04-13   # Convert a specific AD date to BS
npltz show --bs 2081-01-01     # Convert a specific BS date to AD
npltz show --json              # Output as JSON
npltz show --upcoming 10       # Print the next 10 BS dates (replace 10 with any number)
```

#### Week

```sh
npltz week                     # Show the current BS week (7 days)
```

#### Export

```sh
npltz export                   # Export current BS month to .ics
npltz export --month 2083-04   # Export a specific BS month
npltz export --count 3         # Export 3 months starting from current
npltz export -o calendar.ics   # Custom output file
```

#### Themes

```sh
npltz --set-theme catppuccin-mocha
npltz --set-theme default       # Reset to default
```

Available themes: `catppuccin-mocha`, `dracula`, `gruvbox`, `nord`, `rose-pine`

#### Setup (Completions & Man Pages)

Install shell completions and man page to system paths:

```sh
npltz setup
npltz setup --dry-run           # Preview without installing
```

This detects your platform (Linux, macOS, Termux) and installs to the correct directories. Not required on Windows.

#### Check for Updates

```sh
npltz check-update
```

#### Update

```sh
npltz update
```

Follows the same installation method you originally used: cargo, cargo-binstall, or install script.

#### Uninstall

```sh
npltz uninstall
```

Removes the binary, completions, man pages, and config files. Selecting "cargo(c)" will also remove binaries installed by `cargo binstall`, since both install to `~/.cargo/bin/`.

### Completions & Man Pages (Manual)

Generate shell completions manually:

```sh
npltz completions bash
npltz completions zsh
npltz completions fish
```

Generate man page from source:

```sh
git clone https://github.com/harilvfs/npltz.git
cd npltz
cargo xtask man-pages
```

The generated man page is at `man/npltz.1`.

## How it Works

npltz uses calendar data from the [`bikram-sambat`](https://github.com/medic/bikram-sambat) project by [medic](https://github.com/medic). The data (`calendar_data.json`) is compiled into the binary at build time via `include_str!()`, so no external JSON files are needed at runtime. It contains day counts for each month of each BS year (1975-2090) and is used for converting AD (English) dates to BS (Bikram Sambat/Nepali) and vice versa, as well as knowing how many days to render for a given month in the TUI. Star his repo if you find it useful: <https://github.com/medic/bikram-sambat>.

## Contributing

See [CONTRIBUTING.md](https://github.com/harilvfs/npltz/blob/main/.github/CONTRIBUTING.md) for guidelines.

## Community

Join the [Discord](https://discord.com/invite/8NJWstnUHd) for discussions and yapping.

## Thanks

Special thanks to [medic](https://github.com/medic) for the [`bikram-sambat`](https://github.com/medic/bikram-sambat) library that provided the verified calendar data, and to other open-source devs maintaining the awesome crates and tools that npltz depends on. Appreciate that.

## License

npltz is under the MIT license. See [LICENSE](https://github.com/harilvfs/npltz/blob/main/LICENSE) for details.

<!-- Badges -->

[ratatui]: https://img.shields.io/badge/Built_With-Ratatui-000?logo=ratatui&logoColor=fff&labelColor=000&color=fff
[ratatui-link]: https://ratatui.rs/
[check]: https://img.shields.io/github/actions/workflow/status/harilvfs/npltz/ci.yml?branch=main&style=flat&color=fff&labelColor=000&logo=GitHub%20Actions&logoColor=fff&label=CI
[check-link]: https://github.com/harilvfs/npltz/actions/workflows/ci.yml
[downloads]: https://img.shields.io/github/downloads/harilvfs/npltz/total?style=flat&color=fff&logoColor=fff&labelColor=000&logo=github
[downloads-link]: https://github.com/harilvfs/npltz/releases/latest
[crates]: https://img.shields.io/crates/v/npltz?style=flat&logo=rust&color=fff&logoColor=fff&labelColor=000
[crates-link]: https://crates.io/crates/npltz
[license]: https://img.shields.io/github/license/harilvfs/npltz?color=fff&labelColor=000&style=flat&logo=github&logoColor=fff
[license-link]: https://github.com/harilvfs/npltz/blob/main/LICENSE
