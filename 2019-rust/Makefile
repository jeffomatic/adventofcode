folders = $(shell find . -maxdepth 1 -type d -name "day*" | sed 's|^\./||')
executables = $(patsubst %, %/target/debug/%, $(folders))
debug_targets = $(patsubst %, debug-%, $(folders))

.PHONY: no_default $(folders) $(debug_targets)
no_default:

$(folders):
	@ cargo build --manifest-path $@/Cargo.toml
	@ if [ -f $@/input ]; then cat $@/input | RUST_BACKTRACE=1 $@/target/debug/$@; else RUST_BACKTRACE=1 $@/target/debug/$@; fi
