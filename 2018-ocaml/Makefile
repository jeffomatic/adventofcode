folders = $(shell find . -name main.ml | xargs -n1 dirname | sed 's|^\./||')
executables = $(patsubst %, %/main, $(folders))
debug_targets = $(patsubst %, debug-%, $(folders))

.PHONY: no_default $(folders) $(debug_targets)
no_default:

$(folders): %:%/main
	@ if [ -f $@/input ]; then cat $@/input | OCAMLRUNPARAM=b $@/main; else OCAMLRUNPARAM=b $@/main; fi

$(executables): %:%.ml
	@ ocamlfind ocamlopt -package batteries -package core -thread -linkpkg $@.ml -g -o $@

$(debug_targets): debug-%:%/main
	ocamldebug -cd $(@:debug-%=%) main
