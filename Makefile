MAKE_OPTS = --no-print-directory

.PHONY: libunit-wasm
libunit-wasm:
	@echo "Building: libunit-wasm"
	@$(MAKE) $(MAKE_OPTS) -C src/c

.PHONY: examples
examples: libunit-wasm
	@echo "Building: examples"
	@$(MAKE) $(MAKE_OPTS) -C examples/c examples-luw

.PHONY: examples-raw
examples-raw: libunit-wasm
	@echo "Building: raw examples"
	@$(MAKE) $(MAKE_OPTS) -C examples/c examples-raw

.PHONY: rust
rust: libunit-wasm
	@echo "Building: libunit-wasm-rust"
	@$(MAKE) $(MAKE_OPTS) -C src/rust

.PHONY: examples-rust
examples-rust: rust
	@echo "Building: rust examples"
	@$(MAKE) $(MAKE_OPTS) -C examples/rust

.PHONY: all
all: libunit-wasm examples examples-raw rust examples-rust

.PHONY: docker
docker:
	docker build -t unit:wasm -f examples/docker/unit-wasm.Dockerfile .
	docker build -t unit:demo-wasm -f examples/docker/demo-wasm.Dockerfile .

.PHONY: clean
clean:
	@echo "Cleaning: libunit-wasm"
	@$(MAKE) $(MAKE_OPTS) -C src/c clean
	@echo "Cleaning: rust"
	@$(MAKE) $(MAKE_OPTS) -C src/rust clean
	@echo "Cleaning: examples"
	@$(MAKE) $(MAKE_OPTS) -C examples/c clean
	@echo "Cleaning: rust examples"
	@$(MAKE) $(MAKE_OPTS) -C examples/rust clean

.PHONY: tags
tags:
	@echo "Generating ctags..."
	@ctags -R src/ examples/

.PHONY: help
help:
	@echo "Available Targets:"
	@echo "  default / "
	@echo "  libunit-wasm	 - Builds libunit-wasm C library"
	@echo "  examples	 - Builds the above as well as C examples"
	@echo "  examples-raw	 - Builds raw (non libunit-wasm) C examples"
	@echo "  rust		 - Builds the libunit-wasm rust crate"
	@echo "  examples-rust	 _ Builds the above and rust examples"
	@echo "  all		 - Builds all the above"
	@echo "  docker  	 - Builds demo docker images"
	@echo "  clean		 - Removes auto generated artifacts"
	@echo "  tags		 - Generate ctags"
	@echo
	@echo "Variables:"
	@echo "  make CC=            - Specify compiler to use"
	@echo "                        Defaults to clang"
	@echo "  make WASI_SYSROOT=  - Specify the path to the WASI sysroot"
	@echo "                        Defaults to autodetected"
	@echo "  make V=1            - Enables verbose output"
	@echo "  make D=1            - Enables debug builds (-O0)"
	@echo "  make E=1            - Enables Werror"
