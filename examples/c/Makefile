include ../../shared.mk

CFLAGS	+= -I../../src/c/include
LIBS     = -L../../src/c -lunit-wasm

SDIR = examples/c

LUW_SRCDIR = ../../src/c

luw_deps = $(LUW_SRCDIR)/libunit-wasm.a \
	   $(LUW_SRCDIR)/include/unit/unit-wasm.h

examples: examples-luw

examples-luw: luw-echo-request.wasm luw-upload-reflector.wasm

examples-raw: echo-request-raw.wasm upload-reflector-raw.wasm

luw-echo-request.wasm: luw-echo-request.c $(luw_deps)
	$(PP_CCLNK) $(SDIR)/$@
	$(v)$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $< $(LIBS)

luw-upload-reflector.wasm: luw-upload-reflector.c $(luw_deps)
	$(PP_CCLNK) $(SDIR)/$@
	$(v)$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $< $(LIBS)

unit-wasm-raw.o: unit-wasm-raw.c unit-wasm-raw.h
	$(PP_CC) $(SDIR)/$@
	$(v)$(CC) $(CFLAGS) -c $<

echo-request-raw.wasm: echo-request-raw.c unit-wasm-raw.o
	$(PP_CCLNK) $(SDIR)/$@
	$(v)$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $< unit-wasm-raw.o

upload-reflector-raw.wasm: upload-reflector-raw.c unit-wasm-raw.o
	$(PP_CCLNK) $(SDIR)/$@
	$(v)$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $< unit-wasm-raw.o

clean:
	rm -f *.wasm *.o *.gch
