# Get all directories under src/ (excluding files)
DIRS := $(shell find . -maxdepth 1 -type d -exec basename {} \;)

.PHONY: no_default $(DIRS)
no_default:

# Pattern rule: run any directory target
$(DIRS):
	npx tsx $@/main.ts
