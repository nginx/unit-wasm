include ../../shared.mk

CFLAGS	+= -Iinclude

SDIR = src/c

TARGETS = libunit-wasm.o libunit-wasm.a

.PHONY: libunit-wasm
libunit-wasm: $(TARGETS)

libunit-wasm.o: libunit-wasm.c include/unit/unit-wasm.h
	$(PP_CC) $(SDIR)/$@
	$(v)$(CC) $(CFLAGS) -fvisibility=hidden -c $<

libunit-wasm.a: libunit-wasm.o
	$(PP_AR) $(SDIR)/$@
	$(v)llvm-ar rcs $@ $<

clean:
	rm -f *.o *.a *.gch
