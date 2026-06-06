# Markup Swift

A minimal, fast markdown editor built with [egui](https://github.com/emilk/egui) and Rust.

## Features

- Split-pane editing with live HTML preview
- Syntax-highlighted markdown preview
- Multi-tab editing with grid-style Ctrl+Tab switcher
- Writing and Focus modes
- Custom dark theme
- Auto-save and file dialogs

## Build

```sh
./build.sh          # Linux AppImage
./build.sh --win    # Windows .exe (cross-compile)
```

Requires Rust 2021 edition.

## Usage

| Shortcut | Action |
|----------|--------|
| `Ctrl+O` | Open file |
| `Ctrl+S` | Save |
| `Ctrl+N` | New tab |
| `Ctrl+W` | Close tab |
| `Ctrl+Tab` | Open tab switcher |
| `Ctrl+Shift+F` / `F11` | Toggle Focus mode |

## License

CC BY-NC-SA 4.0 — You're free to share and adapt, as long as you credit me and don't use it commercially.
