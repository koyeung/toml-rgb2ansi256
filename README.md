# toml-rgb2ansi256

Convert RGB hex values to [ANSI 256 color code](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit) in [TOML](https://toml.io/en/) document.

## Install
After [Rust Installation](https://www.rust-lang.org/tools/install),

```sh
$ cargo install --git https://github.com/koyeung/toml-rgb2ansi256.git
```


## Usage
[Helix](https://helix-editor.com) supports both RGB truecolor and ANSI 256 color index in [themes](https://docs.helix-editor.com/themes.html). However, some terminal emulators still have no truecolor support. Below command could convert the hex RGB values into ANSI 256 color code:

```sh
$ cp $HOMEBREW_PREFIX/opt/helix/libexec/runtime/themes/github_dark.toml .
$ tail github_dark.toml
"severe.muted" = "#db6d2866"
"severe.subtle" = "#db6d2826"
"sponsors.emphasis" = "#bf4b8a"
"sponsors.fg" = "#db61a2"
"sponsors.muted" = "#db61a266"
"sponsors.subtle" = "#db61a226"
"success.emphasis" = "#238636"
"success.fg" = "#3fb950"
"success.muted" = "#2ea04366"
"success.subtle" = "#2ea04326"
$
$ toml-rgb2ansi256 < github_dark.toml > github_dark_ansi256.toml
$
$ tail github_dark_ansi256.toml
"severe.muted" = "167"
"severe.subtle" = "167"
"sponsors.emphasis" = "132"
"sponsors.fg" = "169"
"sponsors.muted" = "169"
"sponsors.subtle" = "169"
"success.emphasis" = "29"
"success.fg" = "71"
"success.muted" = "71"
"success.subtle" = "71"
$
```

Put `github_dark_ansi256.toml` into `~/.config/helix/themes/`. `:theme` command should apply the theme with approximate colors.


## Thanks
Thanks to [rgb2ansi256](https://crates.io/crates/rgb2ansi256) for color code conversion and [toml_edit](https://crates.io/crates/toml_edit) for format-preserving parsing.


## License
Published under the Apache License Version 2.0.
