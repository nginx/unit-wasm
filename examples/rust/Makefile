include ../../shared.mk

SDIR = examples/rust

examples: rust-echo-request rust-upload-reflector

rust-echo-request: echo-request/Cargo.toml echo-request/src/lib.rs
	$(PP_GEN) $(SDIR)/echo-request/target/wasm32-wasi/
	$(v)cd echo-request; cargo build --target=wasm32-wasi

rust-upload-reflector: upload-reflector/Cargo.toml upload-reflector/src/lib.rs
	$(PP_GEN) $(SDIR)/upload-reflector/target/wasm32-wasi/
	$(v)cd upload-reflector; cargo build --target=wasm32-wasi

clean:
	rm -f echo-request/Cargo.lock upload-reflector/Cargo.lock
	rm -rf echo-request/target upload-reflector/target
