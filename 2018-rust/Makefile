folders = $(shell find . -type d -name "day*" -d 1 | sed 's|^\./||')

.PHONY: no_default $(folders)
no_default:

$(folders):
	@ cargo build --manifest-path $@/Cargo.toml
	@ if [ -f $@/input ]; then cat $@/input | $@/target/debug/$@; else $@/target/debug/$@; fi
