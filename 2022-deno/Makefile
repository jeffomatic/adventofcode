folders = $(shell find . -maxdepth 1 -type d -name "day*" | sed 's|^\./||')
release_versions = $(patsubst %, %-release, $(folders))

.PHONY: no_default $(folders) $(release_versions)
no_default:

$(folders):
	@ if [ -f $@/input ]; then cat $@/input | deno run --allow-read $@/main.ts; else deno run --allow-read $@/main.ts; fi
