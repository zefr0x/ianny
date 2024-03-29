project('ianny', 'rust',
  version : '1.0.0',
  license : 'GPL3',
  default_options : ['warning_level=2'])

application_id = 'io.github.zefr0x.ianny'

# Get options
profile = get_option('profile')

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')

rustc_target = get_option('rustc_target')

# Check for deps
dbus_dep = dependency('dbus-1', version : '>=1.6.0')

desktop_utils = find_program('desktop-file-validate', required: false)
cargo = find_program('cargo', required: true)

# Desktop file
desktop_file ='@0@.desktop'.format(application_id)

if desktop_utils.found()
  test('Validate desktop file', desktop_utils,
    args: [desktop_file]
  )
endif

# Build summaries
summary({
    'Profile': get_option('profile'),
  },
  section: 'Build options',
)

# Other meson.build files
subdir('src/')
subdir('po/')


# Install .desktop files
install_data(
  desktop_file,
  install_dir: 'share/applications'
)

install_data(
  desktop_file,
  install_dir: '/etc/xdg/autostart/'
)
