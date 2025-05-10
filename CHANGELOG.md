# Unreleased

# 2.1.0

## Added

- Add option to ignore idle inhibitors

## Improved

- Prettier and classified log messages.

# 2.0.0

## Breaking Changes

- Remove support for deprecated [`org_kde_kwin_idle`](https://wayland.app/protocols/kde-idle) wayland protocol.

## Fixed

- Relay on system time, not just thread sleeping. Now the timer will make progress even while the device is suspended.

# 1.0.0

## Fixed

- Don't specify an icon, since there is none yet.

## Dependencies

- All dependencies are up to date.

# 1.0.0rc

## Improved

- Better quality and handle more errors.

## Changed

- Use system's gettext instead of compiling it.

## Dependencies

- All dependencies are up to date.

# 1.0.0beta.1

## Added

- `notification:show_progress_bar` and `notification:minimum_update_delay` config options.
- Error message when no Wayland compositor is detected.

## Fixed

- No ensured delay between notification updates.
- No progress bar handling for durations less than 100 seconds.

# 1.0.0beta.0

## Improved

- Reduce release builds binary size.
- Just use one idel interface if both are implemented in the wayland compositor.

# 0.1.0alpha.2

## Added

- i18n support.

## Changed

- Use seconds rather then minutes as a time unit. <sup>`Breaking Change`</sup>
- Fix typo in a config option, replace `long_break_tiemout` with `long_break_timeout`. <sup>`Breaking Change`</sup>

## Localization

- Add the Arabic language.

## Dependencies

- All dependencies are up to date.

# 0.1.0alpha.1

## Added

- Config file support.

## Dependencies

- All dependencies are up to date.

# 0.1.0alpha.0

First release.
