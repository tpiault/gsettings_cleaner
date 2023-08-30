# gsettings_cleaner

Uses GSettings to reset settings that are set to their default value so that they don't count as manually set by the user.

## Installation

```
cargo install --git https://github.com/tpiault/gsettings_cleaner/
```

## Usage

```
Usage: gsettings_cleaner [OPTIONS]

Options:
  -v, --verbose  Output key/value pairs that are reset
  -q, --quiet    Do not output anything
      --dry-run  Do not reset anything
  -h, --help     Print help
  -V, --version  Print version
```

## Example
```
$ gsettings_cleaner -q

$ gsettings_cleaner
1146 keys have been reset

$ gsettings_cleaner -v
...
/org/gnome/gedit/plugins/time/prompt-type: 'prompt-selected-format'
/org/gnome/gedit/plugins/time/custom-format: '%d/%m/%Y %H:%M:%S'
/org/gnome/gedit/plugins/time/selected-format: '%c'
1146 keys have been reset
```

## License

[MIT](https://choosealicense.com/licenses/mit/)