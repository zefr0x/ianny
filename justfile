_default:
	@just --list

lint_all:
	pre-commit run --all-files

todo:
	rg "(.(TODO|FIXME|FIX|HACK|WARN|PREF|NOTE): )|(todo!)" --glob !{{ file_name(justfile()) }}

# vim: set ft=make :
