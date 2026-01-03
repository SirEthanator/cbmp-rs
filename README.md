# cbmp-rs

A CLI for converting cursor SVG files to PNG files.

This is a partial port of [cbmp](https://github.com/ful1e5/cbmp) by [ful1e5](https://github.com/ful1e5). It currently only supports usage through JSON configuration files and does not support animated SVG files. Other features are not planned currently.

## Installation

### AUR

Arch Linux users can install cbmp-rs via the `cbmp-rs` AUR package.

```sh
git clone 'https://aur.archlinux.org/cbmp-rs.git'
cd cbmp-rs
makepkg -si
```

Or feel free to use an AUR helper, such as yay or paru.

### Manual

Manual installation is pretty straightforward.

```sh
git clone 'https://github.com/SirEthanator/cbmp-rs.git'
cd cbmp-rs
cargo install --path .
```

Ensure that `$HOME/.cargo/bin` is in your PATH for Linux and macOS. For windows, make sure `%USERPROFILE%\.cargo\bin` is in your Path.

## Usage

```
Usage: cbmp [OPTIONS] <CONFIG>

Arguments:
  <CONFIG>  Path to JSON configuration file

Options:
  -q, --quiet    Show less logs
  -h, --help     Print help
  -V, --version  Print version
```

### Configuration files

Configuration files use the following format:

```json
{
  "Sample Task": {            <-- Task name
    "dir": "svg",             <-- Specify input directory contianing SVG files
    "out": "bitmaps",         <-- Specify output directory
    "preserveSubdirs": true,  <-- (Optional) Specify if subdirs in dir should be preserved in out (defaults to false)
    "colors": [               <-- (Optional) Specify a list of color replacements
      { "match": "#00FF00", "replace": "#FFFFFF" }
    ]
  }
}
```

Here is an example configuration:

```json
{
  "Bibata-Modern-Amber": {
    "dir": "svg/modern",
    "out": "bitmaps/Bibata-Modern-Amber",
    "colors": [
      { "match": "#00FF00", "replace": "#FF8300" },
      { "match": "#0000FF", "replace": "#FFFFFF" },
      { "match": "#FF0000", "replace": "#001524" }
    ]
  },
  "Bibata-Modern-Classic": {
    "dir": "svg/modern",
    "out": "bitmaps/Bibata-Modern-Classic",
    "colors": [
      { "match": "#00FF00", "replace": "#000000" },
      { "match": "#0000FF", "replace": "#FFFFFF" },
      { "match": "#FF0000", "replace": "#000000" }
    ]
  },
  "Bibata-Original-Amber": {
    "dir": "svg/original",
    "out": "bitmaps/Bibata-Original-Amber",
    "colors": [
      { "match": "#00FF00", "replace": "#FF8300" },
      { "match": "#0000FF", "replace": "#FFFFFF" },
      { "match": "#FF0000", "replace": "#001524" }
    ]
  }
}
```

## Why?

I built this port for two reasons: cbmp was very slow to install (~10 minutes in my experience), and since my setup dynamically generates cursors when the wallpaper changes, I wanted something faster.
