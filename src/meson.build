if get_option('cargo-home') == ''
  cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]
else
  cargo_env = [ 'CARGO_HOME=' + get_option('cargo-home') ]
endif

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]
if rustc_target != ''
  cargo_options += [ '--target', rustc_target ]
endif

# When cargo-home already contain needed deps and we want to build offline.
if get_option('offline-build')
  cargo_options += [ '--offline' ]
endif

if buildtype == 'plain' or buildtype == 'release' or buildtype == 'minsize'
  cargo_profile = 'release'
  cargo_options += [ '--release' ]
else
  cargo_profile = 'debug'
  message('Building in debug mode')
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'src' / rustc_target / cargo_profile / meson.project_name(), '@OUTPUT@',
  ]
)
