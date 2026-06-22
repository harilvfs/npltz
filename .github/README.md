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
> npltz is in its very early stages. If you run into bugs, please [open an issue](https://github.com/harilvfs/npltz/issues).

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

### Linux / Termux

```sh
curl -fsSL npltz.chalisehari.com.np/install | sh
```

### Cargo (Linux, macOS, Windows)

```sh
cargo install npltz
```

As a Linux user, I don't know how installation works directly on macOS or Windows, so I'm not providing a direct install script for them. Even though macOS supports Unix-like environments and shell scripts can run there, I've never used macOS myself, so without basic knowledge I don't want to talk shit. Windows I don't give a shit either, but you can still install with Cargo.

### Build from Source

```sh
git clone https://github.com/harilvfs/npltz.git
cd npltz
cargo build --release
./build/release/npltz
```

### Uninstall

```sh
cargo uninstall npltz
```

If installed via install script then, remove files manually: 

On Linux

```sh
sudo rm /usr/local/bin/npltz
sudo rm /usr/share/bash-completion/completions/npltz
sudo rm /usr/share/zsh/site-functions/_npltz
sudo rm /usr/share/fish/vendor_completions.d/npltz.fish
sudo rm /usr/share/man/man1/npltz.1
sudo rm /usr/share/applications/npltz.desktop
rm -rf ~/.config/npltz
```

On Termux

```sh
rm $PREFIX/bin/npltz
rm $PREFIX/share/bash-completion/completions/npltz
rm $PREFIX/share/zsh/site-functions/_npltz
rm $PREFIX/share/fish/vendor_completions.d/npltz.fish
rm $PREFIX/share/man/man1/npltz.1
rm -rf ~/.config/npltz
```

> [!CAUTION]
> npltz hasn't been tested on macOS or Windows, so no guarantees it'll work on your OS. Use at your own risk.

## Usage

npltz is a simple TUI built with ratatui. Keybinds are shown on screen to navigate, switch years, go to a specific year, change themes, and more.

### Commands

npltz supports some CLI commands too. See them with:

```
npltz --help
```

CLI commands provide AD-to-BS (`convert`) and BS-to-AD (`convert-bs`) conversion. If you want to know a specific date in AD or BS, you can do it directly from the CLI.

npltz also supports themes you can set via the CLI or inside the TUI (press `c`):

```
npltz --set-theme <theme-name>
```

### Completions / Man Pages

Generate shell completions:

```
npltz completions <shell>
```

Example:

```
npltz completions bash
```

Man pages are generated via xtask using `clap_mangen`:

```
cargo xtask man-pages
```

## How it Works

npltz uses calendar data from the [`nepali-date-converter`](https://crates.io/crates/nepali-date-converter) project by [dhurbachy](https://github.com/dhurbachy). The data (`calendar_data.json`) is compiled into the binary at build time via `include_str!()`, so no external JSON files are needed at runtime. It contains day counts for each month of each BS year (1975-2099) and is used for converting AD (English) dates to BS (Bikram Sambat/Nepali) and vice versa, as well as knowing how many days to render for a given month in the TUI. Star his repo if you find it useful: <https://github.com/dhurbachy/Nepali-Date-Converter>.

## Contributing

See [CONTRIBUTING.md](https://github.com/harilvfs/npltz/blob/main/.github/CONTRIBUTING.md) for guidelines.

## Thanks

Special thanks to [dhurbachy](https://github.com/dhurbachy) for the [`Nepali-Date-Converter`](https://github.com/dhurbachy/Nepali-Date-Converter) library that made this tool possible, and to other open-source devs maintaining the awesome crates and tools that npltz depends on. Appreciate that.

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
