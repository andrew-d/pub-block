.SUFFIXES:


.PHONY: all
all: lib example


.PHONY: lib
lib:
	@cargo build

.PHONY: example
example: lib
	rustc -o target/test -L target examples/test.rs
