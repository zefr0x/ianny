<div align = center>

<h1>Ianny | عَيْنِي</h1>

Simple, light-weight, easy to use, and effective [Linux](https://en.wikipedia.org/wiki/Linux) [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) desktop utility helps preventing [repetitive strain injuries](https://en.wikipedia.org/wiki/Repetitive_strain_injury) by keeping track of usage patterns and periodically informing user to take breaks.

</div>

## Features

- ⚙ Simple settings to tweak application behavior. `[WIP]`
- 🚀 Auto start it with your desktop environment. `[WIP]`
- 🚫 [X11](https://en.wikipedia.org/wiki/X_Window_System) is not supported.
- 🚫 Microsoft Windows is definitely not supported.

## Requirements

- [GTK4](https://www.gtk.org/)
- [GTK4 Layer Shell](https://github.com/wmww/gtk4-layer-shell)
- [Adwaita](https://gitlab.gnome.org/GNOME/libadwaita/)
- Wayland compositor that support the `Layer Shell` protocol, as listed [here](https://github.com/wmww/gtk4-layer-shell#supported-desktops).

## Installation

### Download Binary From Github
For every new release a Github workflow will build a binary in Github servers and will upload it as a release asset in Github releases.

You can find the latest Github release [here](https://github.com/zer0-x/ianny/releases/latest) or the releases page [here](https://github.com/zer0-x/ianny/releases).

## Build

> **Note**
> You need to have [`cargo`](https://doc.rust-lang.org/cargo/) installed in you system.

```shell
git clone https://github.com/zer0-x/ianny.git

cd ianny

# Checkout to a release tag e.g. v1.0.1
git checkout vx.x.x

cargo build --release
```

You will find the binary in `./target/release/ianny`

## Q&A

Q: What does `Ianny` mean?
- It is an Arabic word `عَيْنِي` that could be translated to `My Eye` in english.

## Inspired by

- [KDE's RSIBreak](https://userbase.kde.org/RSIBreak)
