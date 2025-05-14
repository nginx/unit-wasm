include ../../shared.mk

TARGET    = wasm32-wasip1
CARGO_CMD = cargo build --target=$(TARGET)

SDIR = examples/rust

examples: rust-echo-request \
	  rust-upload-reflector \
	  rust-hello-world \
	  rust-large-upload

rust-echo-request: echo-request/Cargo.toml echo-request/src/lib.rs
	$(PP_GEN) $(SDIR)/echo-request/target/$(TARGET)/
	$(v)cd echo-request; $(CARGO_CMD)

rust-upload-reflector: upload-reflector/Cargo.toml upload-reflector/src/lib.rs
	$(PP_GEN) $(SDIR)/upload-reflector/target/$(TARGET)/
	$(v)cd upload-reflector; $(CARGO_CMD)

rust-hello-world: hello-world/Cargo.toml hello-world/src/lib.rs
	$(PP_GEN) $(SDIR)/hello-world/target/$(TARGET)/
	$(v)cd hello-world; $(CARGO_CMD)

rust-large-upload: large-upload/Cargo.toml large-upload/src/lib.rs
	$(PP_GEN) $(SDIR)/large-upload/target/$(TARGET)/
	$(v)cd large-upload; $(CARGO_CMD)

clean:
	rm -f */Cargo.lock
	rm -rf */target
