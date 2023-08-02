include ../../shared.mk

SDIR = src/rust

rustlib:
	$(PP_GEN) $(SDIR)/target/wasm32-wasi
	$(v)cargo build --target=wasm32-wasi

clean:
	rm -f Cargo.lock unit-wasm-sys/Cargo.lock
	rm -rf target/ unit-wasm-sys/target/
