_default:
	@just --list

lint_all:
	pre-commit run --all-files

pot:
	# Generate .pot file from sorce code using `xtr` (https://github.com/woboq/tr).
	xtr --output po/io.github.zefr0x.ianny.pot --package-name Ianny  src/main.rs
	sed -i 1,2d po/io.github.zefr0x.ianny.pot

update_po:
	for lang in `cat ./po/LINGUAS`; do \
	msgmerge --update ./po/${lang}.po ./po/io.github.zefr0x.ianny.pot; \
	done

todo:
	rg "(.(TODO|FIXME|FIX|HACK|WARN|PREF|NOTE): )|(todo!)" --glob !{{ file_name(justfile()) }}

# vim: set ft=make :
