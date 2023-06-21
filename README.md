<div align = center>

<h1>Ianny | عَيْنِي</h1>

Simple, light-weight, easy to use, and effective [Linux](https://en.wikipedia.org/wiki/Linux) [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) desktop utility helps preventing [repetitive strain injuries](https://en.wikipedia.org/wiki/Repetitive_strain_injury) by keeping track of usage patterns and periodically informing user to take breaks.

</div>

## Features

- ⚙ Simple config to tweak application behavior. `[WIP]`
- 🔒Optionally lock your screen with your desktop environment`s locker. `[WIP]`
- 🚀 Auto start it with your desktop environment. `[WIP]`
- 🚫 [X11](https://en.wikipedia.org/wiki/X_Window_System) is not supported.
- 🚫 Microsoft Windows is definitely not supported.

## Requirements

- [Notification Daemon](https://wiki.archlinux.org/title/Desktop_notifications#Notification_servers) that implements [`org.freedesktop.Notifications`](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html)
- [Wayland Compositor](https://en.wikipedia.org/wiki/Wayland_(protocol)#Wayland_compositors)
- [Systemd](https://en.wikipedia.org/wiki/Systemd) for the lock functionality `(Optional)`

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
