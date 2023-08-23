# Some common Makefile stuff

# Look for wasi-sysroot in some common places, falling back
# to provided WASI_SYSROOT
ifneq ("$(wildcard /usr/wasm32-wasi)", "")
        # Fedora
        WASI_SYSROOT ?= /usr/wasm32-wasi
else ifneq ("$(wildcard /usr/local/share/wasi-sysroot)", "")
        # FreeBSD
        WASI_SYSROOT ?= /usr/local/share/wasi-sysroot
endif

export WASI_SYSROOT

# By default compiler etc output is hidden, use
#   make V=1 ...
# to show it
v = @
ifeq ($V,1)
        v =
endif

# Optionally enable debugging builds with
#   make D=1 ...
# -g is always used, this just changes the optimisation level.
# On GCC this would be -Og, however according to the clang-16(1)
# man page, -O0 'generates the most debuggable code'.
ifeq ($D,1)
        CFLAGS += -O0
else
        CFLAGS += -O2
endif

# Optionally enable Werror with
#   make E=1 ...
ifeq ($E,1)
        CFLAGS += -Werror
endif

# Pretty print compiler etc actions...
PP_CC		= @echo '  CC    '
PP_AR		= @echo '  AR    '
PP_CCLNK	= @echo '  CCLNK '
PP_GEN		= @echo '  GEN   '

CC       = clang
CFLAGS  += -Wall -Wextra -Wdeclaration-after-statement -Wvla \
           -Wmissing-prototypes -Wstrict-prototypes -Wold-style-definition \
           -Wimplicit-function-declaration -Wimplicit-int -Wint-conversion \
           -std=gnu11 -g -fno-common -fno-strict-aliasing \
           --target=wasm32-wasi --sysroot=$(WASI_SYSROOT)
LDFLAGS  = -Wl,--no-entry,--export=__heap_base,--export=__data_end,--export=malloc,--export=free,--stack-first,-z,stack-size=$$((8*1024*1024)) \
           -mexec-model=reactor --rtlib=compiler-rt
