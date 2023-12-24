<div align = center>

<h1>Ianny | عَيْنِي</h1>

[![release](https://github.com/zefr0x/ianny/actions/workflows/release.yml/badge.svg)](https://github.com/zefr0x/ianny/actions/workflows/release.yml)

Simple, light-weight, easy to use, and effective [Linux](https://en.wikipedia.org/wiki/Linux) [Wayland](<https://en.wikipedia.org/wiki/Wayland_(protocol)>) desktop utility helps preventing [repetitive strain injuries](https://en.wikipedia.org/wiki/Repetitive_strain_injury) by keeping track of usage patterns and periodically informing user to take breaks.

</div>

## Features

- ⚙ Simple config to tweak it's behavior.
- 🚀 Auto start it with your desktop environment.
- 🚫 [X11](https://en.wikipedia.org/wiki/X_Window_System) is not supported.
- 🚫 Microsoft Windows is definitely not supported.

## Requirements

- [Wayland Compositor](<https://en.wikipedia.org/wiki/Wayland_(protocol)#Wayland_compositors>)
- [Notification Daemon](https://wiki.archlinux.org/title/Desktop_notifications#Notification_servers) that implements [`org.freedesktop.Notifications`](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html)
- [libdbus-1.so](https://www.freedesktop.org/wiki/Software/dbus/) installed in your system
- [Linux libc](https://en.wikipedia.org/wiki/C_standard_library) via either [glibc](https://www.gnu.org/software/libc/) or [musl libc](https://musl.libc.org/)

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/ianny.svg?columns=3)](https://repology.org/project/ianny/versions)

### Download Binary From Github

For every new release a Github workflow will build a binary in Github servers and will upload it as a release asset in Github releases.

You can find the latest Github release [here](https://github.com/zefr0x/ianny/releases/latest) or the releases page [here](https://github.com/zefr0x/ianny/releases).

## Build

> [!Note]
> You need to have [`cargo`](https://doc.rust-lang.org/cargo/), [`meson`](https://mesonbuild.com/) and [`libdbus-1-dev`](https://www.freedesktop.org/wiki/Software/dbus/) installed in you system.

```shell
git clone https://github.com/zefr0x/ianny.git

cd ianny

# Checkout to a release tag e.g. v1.0.1
git checkout vx.x.x

meson build
meson compile -C build
```

You will find the binary in `./build/src/ianny`

To install:

```shell
meson install -C build
```

# Usage

You just need to execute the binary eather direcrly or by enabling it to auto-start with your desktop environment's settings, since it provides a `.desktop` file for auto-start.

# Config

The defaults might not fit your needs, so you can change them via a config file.

The config file is `$XDG_CONFIG_HOME/io.github.zefr0x.ianny/config.toml` or by default `~/.config/io.github.zefr0x.ianny/config.toml`. Just create it and specify the options you need with the [toml format](https://toml.io/):

```toml
[timer]
idle_timeout = 240
short_break_timeout = 1200
long_break_timeout = 3840
short_break_duration = 120
long_break_duration = 240

[notification]
show_progress_bar = true
minimum_update_delay = 1
```

> [!Note]
> Time specified in seconds

## Q&A

Q: What does `Ianny` mean?

- It is an Arabic word `عَيْنِي` that could be translated to `My Eye` in english.

## Inspired by

- [KDE's RSIBreak](https://userbase.kde.org/RSIBreak)
