include ../../shared.mk

SDIR = examples/rust

examples: rust-echo-request \
	  rust-upload-reflector \
	  rust-hello-world \
	  rust-large-upload

rust-echo-request: echo-request/Cargo.toml echo-request/src/lib.rs
	$(PP_GEN) $(SDIR)/echo-request/target/wasm32-wasi/
	$(v)cd echo-request; cargo build --target=wasm32-wasi

rust-upload-reflector: upload-reflector/Cargo.toml upload-reflector/src/lib.rs
	$(PP_GEN) $(SDIR)/upload-reflector/target/wasm32-wasi/
	$(v)cd upload-reflector; cargo build --target=wasm32-wasi

rust-hello-world: hello-world/Cargo.toml hello-world/src/lib.rs
	$(PP_GEN) $(SDIR)/hello-world/target/wasm32-wasi/
	$(v)cd hello-world; cargo build --target=wasm32-wasi

rust-large-upload: large-upload/Cargo.toml large-upload/src/lib.rs
	$(PP_GEN) $(SDIR)/large-upload/target/wasm32-wasi/
	$(v)cd large-upload; cargo build --target=wasm32-wasi

clean:
	rm -f */Cargo.lock
	rm -rf */target
