# cbmp-rs

A CLI for converting cursor SVG files to PNG files.

This is a partial port of [cbmp](https://github.com/ful1e5/cbmp) by [ful1e5](https://github.com/ful1e5). It currently only supports usage through JSON configuration files and does not support animated SVG files. Other features are not planned currently.

## Usage

```
Usage: cbmp <CONFIG>

Arguments:
  <CONFIG>  Path to JSON configuration file

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Configuration files

Configuration files use the following format:

```json
{
  "Sample Task": {     <-- Task name
    "dir": "svg",      <-- Specify input directory contianing SVG files
    "out": "bitmaps",  <-- Specify output directory
    "colors": [        <-- (Optional) Specify a list of color replacements
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
