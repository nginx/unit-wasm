# C & Rust Library & Examples for Building WebAssembly Modules for NGINX Unit

This provides a C library (lbunit-wasm) and Rust crates based on that library
to aid in the creation of WebAssembly modules in C and Rust.

It also has some demo WebAssembly modules written in C and Rust.

1. [C & Rust Library & Examples for Building WebAssembly Modules for NGINX Unit](#c---rust-library---examples-for-building-webassembly-modules-for-nginx-unit)
2. [Repository Layout](#repository-layout)
3. [Setup a Suitable Environment](#setup-a-suitable-environment)
    1. [Fedora](#fedora)
    2. [Debian / Ubuntu](#debian--ubuntu)
4. [Quickstart in Developing Rust WebAssembly Modules for Unit](#quickstart-in-developing-rust-webassembly-modules-for-unit)
5. [Working With the Repository](#working-with-the-repository)
6. [Using With Unit](#using-with-unit)
7. [Consuming the C Library](#consuming-the-c-library)
8. [License](#license)

## Repository Layout

**src/c** contains the main libunit-wasm library.

**src/rust** contains the rust version of the above.

**examples/c** contains some demo WebAssembly modules that show both the raw
interface to Unit (\*-raw.c) and also the use of libunit-wasm (luw-\*.c).

**examples/rust** contains rust versions of the above C demo modules and more.

**examples/docker** contains docker files for building Unit with WebAssembly
support and the C examples.

## Setup a Suitable Environment

To make full use of this repository you will require numerous tools/packages.

Exactly what you need and how to get it will depend on your Operating System.

This has been primarily developed and tested on Fedora & Ubuntu Linux.

### Fedora

On Fedora make sure you have the following installed

```
# dnf install make clang llvm compiler-rt lld wasi-libc-devel \
              wasi-libc-static cargo rust rustfmt rust-std-static \
              rust-std-static-wasm32-unknown-unknown \
              rust-std-static-wasm32-wasi
```

One last item you will need is the libclang wasm32-wasi runtime library, this
can be done with

```
# wget -O- https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/libclang_rt.builtins-wasm32-wasi-20.0.tar.gz | tar --strip-components=1 -xvzf - -C $(dirname $(clang -print-runtime-dir))
```

This will install the following, path may vary slightly

```
/usr/lib64/clang/16/lib/wasi/libclang_rt.builtins-wasm32.a
```

The above should also work (perhaps with slight alterations) on recent
CentOS/RHEL etc...

**NOTE:** If you get a major Clang version update, you may need to repeat
that last task.

### Debian / Ubuntu

Install the following as normal

```
# apt install wasi-libc make clang llvm lld
```

For the rest you will likely need to use [rustup](https://rustup.rs). Caveat
Emptor.

After you've completed the _rustup_ installation by following the on screen
instructions as yourself (defaults are fine), do the following

```shell
$ rustup target add wasm32-wasi
```

You will also need to grab the wasi-sysroot, this is essentially a C library
targeting WebAssembly (it is based partly on cloudlibc and musl libc) and
is required for building server side WebAssembly modules.

It's up to you where you put this.

```shell
$ wget -O- https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/wasi-sysroot-20.0.tar.gz | tar -xzf -
```

And finally similarly as Fedora (this requires a recentish clang)

```
# wget -O- https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/libclang_rt.builtins-wasm32-wasi-20.0.tar.gz | tar --strip-components=1 -xvzf - -C $(dirname $(clang -print-runtime-dir))
```

## Quickstart in Developing Rust WebAssembly Modules for Unit

1) Setup a suitable build environment as above.

2) Create a new rust project

```shell
$ cargo init --lib my-wasm-example
```

3) Add the [unit-wasm crate](https://crates.io/crates/unit-wasm) as dependency

```shell
$ cd my-wasm-example
$ cargo add unit-wasm
```

4) Set the crate type

```shell
$ printf '\n[lib]\ncrate-type = ["cdylib"]\n' >>Cargo.toml
```

5) Create an example application

To do this you can simply take a copy of our echo-request demo in this
repository

```shell
$ wget -O src/lib.rs https://raw.githubusercontent.com/nginx/unit-wasm/main/examples/rust/echo-request/src/lib.rs
```

6) Build it!

```shell
$ cargo build --target wasm32-wasi
```

You should now have a *target/wasm32-wasi/debug/my_wasm_example.wasm* file
(yes, hyphens will be turned to underscores)

You can now use this in Unit with the following config

```JSON
{
    "listeners": {
        "[::1]:8888": {
            "pass": "applications/my-wasm-example"
        }
    },

    "applications": {
        "my-wasm-example": {
            "type": "wasm",
            "module": "/path/to/my-wasm-example/target/wasm32-wasi/debug/my_wasm_example.wasm",
            "request_handler": "uwr_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler",
            "module_init_handler": "uwr_module_init_handler",
            "module_end_handler": "uwr_module_end_handler"
        }
    }
}
```

and curl command

```shell
$ curl http://localhost:8888/
```

7) Start writing your own Wasm modules in Rust!

To help you get started, you can check out the Rust examples under
[examples/rust](https://github.com/nginx/unit-wasm/tree/main/examples/rust)
and also the
[API-Rust.md](https://github.com/nginx/unit-wasm/blob/main/API-Rust.md)
for an overview of the API.

## Working With the Repository

The project uses good old _make(1)_ as the build system.

Typing _make help_ will show the list of available targets and the various
make variables you can set. E.g

```
$ make help
Available Targets:
  default /
  libunit-wasm   - Builds libunit-wasm C library
  examples       - Builds the above as well as C examples
  examples-raw   - Builds raw (non libunit-wasm) C examples
  rust           - Builds the libunit-wasm rust crate
  examples-rust  _ Builds the above and rust examples
  all            - Builds all the above
  docker         - Builds demo docker images
  clean          - Removes auto generated artifacts
  tags           - Generate ctags

Variables:
  make CC=            - Specify compiler to use
                        Defaults to clang
  make WASI_SYSROOT=  - Specify the path to the WASI sysroot
                        Defaults to autodetected
  make V=1            - Enables verbose output
  make D=1            - Enables debug builds (-O0)
  make E=1            - Enables Werror
```

If you have previously followed the steps outlined above in
[Setup a Suitable Environment](#setup-a-suitable-environment)

You can build the libunit-wasm C library, C example Wasm modules, the Rust
crates that provides Rust bindings for the libunit-wasm and the Rust example
Wasm modules.

E.g

```shell
$ make                  # Build just the C library
$ make examples         # Build the C library and C examples
$ make rust             # Build the Rust crates
$ make examples-rust    # Build the Rust examples
$ make all              # Build all the above
```

The C and Rust example Wasm modules will be located at

```
examples/c/luw-echo-request.wasm
examples/c/luw-upload-reflector.wasm
examples/rust/echo-request/target/wasm32-wasi/debug/rust_echo_request.wasm
examples/rust/hello-world/target/wasm32-wasi/debug/rust_hello_world.wasm
examples/rust/upload-reflector/target/wasm32-wasi/debug/rust_upload_reflector.wasm
```

**NOTE:** To build the C library and examples you will need to specify the
wasi-sysroot.

E.g

```shell
$ make WASI_SYSROOT=/path/to/wasi-sysroot all
```

**However** if you are on Fedora and installed the _wasi-*_ packages listed
above in the [Setup a Suitable Environment](#fedora) then the wasi-sysroot
path will be autodetected and set by the Makefile. You can override the
autodetection by explicitly specifying it as above.

## Using With Unit

If you have all the above built, you are now ready to test it out with Unit.

We won't go into the details of building Unit from source and enabling the Unit
WebAssembly language module here (see the
[HOWTO.md](https://github.com/nginx/unit-wasm/blob/main/HOWTO.md) in the
repository root for more details) but will instead assume you already have a
Unit with the WebAssembly language module already running, perhaps installed
via a package.

Create the following Unit config

```JSON
{
    "listeners": {
        "[::1]:8888": {
            "pass": "routes"
        }
    },

    "settings": {
        "http": {
            "max_body_size": 1073741824
        }
    },

    "routes": [
        {
            "match": {
                "uri": "/echo*"
            },
            "action": {
                "pass": "applications/luw-echo-request"
            }
        },
        {
            "match": {
                "uri": "/upload*"
            },
            "action": {
                "pass": "applications/luw-upload-reflector"
            }
        },
        {
            "match": {
                "uri": "/rust-echo*"
            },
            "action": {
                "pass": "applications/rust-echo-request"
            }
        },
        {
            "match": {
                "uri": "/rust-upload*"
            },
            "action": {
                "pass": "applications/rust-upload-reflector"
            }
        },
        {
            "match": {
                "uri": "/hello-world*"
            },
            "action": {
                "pass": "applications/rust-hello-world"
            }
        }
    ],

    "applications": {
        "luw-echo-request": {
            "type": "wasm",
            "module": "/path/to/unit-wasm/examples/c/luw-echo-request.wasm",
            "request_handler": "luw_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler",
            "module_init_handler": "luw_module_init_handler",
            "module_end_handler": "luw_module_end_handler"
        },
        "luw-upload-reflector": {
            "type": "wasm",
            "module": "/path/to/unit-wasm/examples/c/luw-upload-reflector.wasm",
            "request_handler": "luw_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler",
            "request_end_handler": "luw_request_end_handler",
            "response_end_handler": "luw_response_end_handler"
        },
        "rust-echo-request": {
            "type": "wasm",
            "module": "/path/to/unit-wasm/examples/rust/echo-request/target/wasm32-wasi/debug/rust_echo_request.wasm",
            "request_handler": "uwr_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler",
            "module_init_handler": "uwr_module_init_handler",
            "module_end_handler": "uwr_module_end_handler"
        },
        "rust-upload-reflector": {
            "type": "wasm",
            "module": "/path/to/unit-wasm/examples/rust/upload-reflector/rust_upload_reflector.wasm",
            "request_handler": "uwr_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler",
            "request_end_handler": "uwr_request_end_handler",
            "response_end_handler": "uwr_response_end_handler"
        },
        "rust-hello-world": {
            "type": "wasm",
            "module": "/path/to/unit-wasm/examples/rust/hello-world/target/wasm32-wasi/debug/rust_hello_world.wasm",
            "request_handler": "uwr_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler"
        }
    }
}
```

Load this config then you should be ready to try it.

```
$ curl -X POST -d "Hello World" --cookie "mycookie=hmmm" http://localhost:8888/echo/?q=a
 *** Welcome to WebAssembly on Unit! [libunit-wasm (0.1.0/0x00010000)] ***

[Request Info]
REQUEST_PATH = /echo/?q=a
METHOD       = POST
VERSION      = HTTP/1.1
QUERY        = q=a
REMOTE       = ::1
LOCAL_ADDR   = ::1
LOCAL_PORT   = 8080
SERVER_NAME  = localhost

[Request Headers]
Host = localhost:8080
User-Agent = curl/8.0.1
Accept = */*
Cookie = mycookie=hmmm
Content-Length = 11
Content-Type = application/x-www-form-urlencoded

[POST data]
Hello World
```

```
$ curl -v -X POST --data-binary @audio.flac -H "Content-Type: audio/flac" http://localhost:8888/upload-reflector/ -o wasm-test.dat
...
> Content-Type: audio/flac
> Content-Length: 60406273
...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  115M  100 57.6M  100 57.6M  47.6M  47.6M  0:00:01  0:00:01 --:--:-- 95.2M
...
< Content-Type: audio/flac
< Content-Length: 60406273
...
$ sha1sum audio.flac wasm-test.dat
ef5c9c228544b237022584a8ac4612005cd6263e  audio.flac
ef5c9c228544b237022584a8ac4612005cd6263e  wasm-test.dat
```

## Consuming the C Library

If **unit/unit-wasm.h** and **libunit.a** are installed into standard
include/library directories then

Include the libunit-wasm header file

```C
....
#include <unit/unit-wasm.h>
...
```

Link against libunit-wasm

```shell
$ clang ... -o myapp.wasm myapp.c -lunit-wasm
```

See [API-C.md](https://github.com/nginx/unit-wasm/blob/main/API-C.md) for an
overview of the API.

## License

This project is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
