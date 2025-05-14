include ../../shared.mk

SDIR = src/rust

rustlib:
	$(PP_GEN) $(SDIR)/target/wasm32-wasip1
	$(v)cargo build --target=wasm32-wasip1

clean:
	rm -f Cargo.lock unit-wasm-sys/Cargo.lock
	rm -rf target/ unit-wasm-sys/target/
