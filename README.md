# C & Rust Library & Examples for Building WebAssembly Modules for NGINX Unit

This provides a C library (lbunit-wasm) and a Rust crate based on that library
to aid in the creation of WebAssembly modules in C and Rust.

It also has some demo WebAssembly modules written in C and Rust.

1. [C & Rust Library & Examples for Building WebAssembly Modules for NGINX Unit](#c---rust-library---examples-for-building-webassembly-modules-for-nginx-unit)
2. [Repository Layout](#repository-layout)
3. [Quickstart in Developing Rust WebAssembly Modules for Unit](#quickstart-in-developing-rust-webassembly-modules-for-unit)
4. [Getting Started](#getting-started)
    1. [Requirements](#requirements)
        * [wasi-sysroot](#wasi-sysroot)
        * [libclang_rt.builtins-wasm32-wasi](#libclang_rtbuiltins-wasm32-wasi)
    2. [Building libunit-wasm and C Examples](#building-libunit-wasm-and-c-examples)
    3. [Building the Rust libunit-wasm Crate and Examples](#building-the-rust-libunit-wasm-crate-and-examples)
    4. [Using With Unit](#using-with-unit)
5. [Consuming the C Library](#consuming-the-c-library)
6. [License](#license)

## Repository Layout

**src/c** contains the main libunit-wasm library.

**src/rust** contains the rust version of the above.

**examples/c** contains some demo WebAssembly modules that show both the raw
interface to Unit (\*-raw.c) and also the use of libunit-wasm (luw-\*.c).

**examples/rust** contains rust versions of the above C demo modules.

**examples/docker** contains docker files for building Unit with WebAssembly
support and the C examples.

## Quickstart in Developing Rust WebAssembly Modules for Unit

1) Have a suitable rust/wasm environment setup, See
[Building the Rust libunit-wasm Crate and Examples](#building-the-rust-libunit-wasm-crate-and-examples) for some details

2) Create a new rust project

```
$ cargo init --lib my-wasm-example
```

3) Add the [unit-wasm crate](https://crates.io/crates/unit-wasm) as dependency

```
$ cd my-wasm-example
$ cargo add unit-wasm
```

4) Create the following _Cargo.toml_ file

```
[package]
name = "my-wasm-example"
version = "0.1.0"
edition = "2021"

[dependencies]
unit-wasm = { version = "0.1.0" }

[lib]
crate-type = ["cdylib"]
```

5) Create an example application

To do this you can simply take a copy of our echo-request demo in this
repository

```
$ wget -O src/lib.rs https://raw.githubusercontent.com/nginx/unit-wasm/master/examples/rust/echo-request/src/lib.rs
```

6) Built it!

```
$ cargo build --target wasm32-wasi
```

You should now have a *target/wasm32-wasi/debug/my_wasm_example.wasm* file
(yes, hyphens will be turned to underscores)

You can now use this in Unit with the following application config snippet

```JSON
        "my-wasm-example": {
            "type": "wasm",
            "module": "/path/to/my-wasm-example/target/wasm32-wasi/debug/my_wasm_example.wasm",
            "request_handler": "luw_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler",
            "module_init_handler": "luw_module_init_handler",
            "module_end_handler": "luw_module_end_handler"
        },
```

7) Enjoy!

## Getting Started

### Requirements

To build the C library and demo modules you will need make, clang, llvm,
compiler-rt & lld 8.0+. (GCC does not currently have any support for
WebAssembly).

#### wasi-sysroot

This is essentially a C library (based at least partly on cloudlibc and musl
libc) for WebAssembly and is required for the _wasm32-wasi_ target.

Distributions are starting to package this. On Fedora for example you can
install the

```
wasi-libc-devel
wasi-libc-static
```

packages.

On FreeBSD you can install the

```
wasi-libc
```

package.

The Makefiles will pick this up automatically.

Where _wasi-sysroot_ is not available you can grab it from
[here](https://github.com/WebAssembly/wasi-sdk/releases). Just grab the latest
wasi-sysroot-VERSION.tar.gz tarball.

Untar the wasi-sysroot package someplace.

#### libclang_rt.builtins-wasm32-wasi

You will probably also need to grab the latest
libclang\_rt.builtins-wasm32-wasi-VERSION.tar.gz tarball from the same
location and keep it handy.

### Building libunit-wasm and C Examples

Once you have the above sorted you can simply try doing

```
$ make WASI_SYSROOT=/path/to/wasi-sysroot examples
```

**NOTE:**

The Makefiles will look for an already installed wasi-sysroot on Fedora &
FreeBSD, so you may not need to specify it as above.

If you do, you can set the WASI\_SYSROOT environment variable in your shell so
you don't need to specify it here.

This will attempt to build libunit-wasm and the two example WebAssembly
modules, _luw-echo-request.wasm_ & _luw-upload-reflector.wasm_.

If the above fails (which currently there is a good chance it will) with an
error message like

```
wasm-ld: error: cannot open /usr/lib64/clang/16/lib/wasi/libclang_rt.builtins-wasm32.a: No such file or directory
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

Then this is where that other tarball you downloaded comes in. Extract the
*libclang\_rt.builtins-wasm32.a* from it and copy it into the location
mentioned in the above error message.

Try the make command again...

### Building the Rust libunit-wasm Crate and Examples

To build the rust stuff you will of course need rust and also cargo and the
rust wasm/wasi stuff. On Fedora this is the relevant packages I have installed

```
cargo
rust
rust-std-static
rust-std-static-wasm32-unknown-unknown
rust-std-static-wasm32-wasi
```

Install with $PKGMGR.

If you have also completed the above building of the C library and examples
you should now be good to go.

```
$ make examples-rust
```

If you need to specify the *WASI_SYSROOT*, specify it in the make command as
above.

This will build the libunit-wasm rust crate and rust example modules.

### Using With Unit

Now that you have all the above built, you are now ready to test it out with
Unit.

If you created both the C and rust examples you will now have the following
WebAssembly modules

```
examples/c/luw-echo-request.wasm
examples/c/luw-upload-reflector.wasm
examples/rust/echo-request/target/wasm32-wasi/debug/rust_echo_test.wasm
examples/rust/upload-reflector/target/wasm32-wasi/debug/rust_upload_reflector.wasm
```

We won't go into the details of building Unit from source and enabling the
Unit WebAssembly language module here (see the [HOWTO.md](https://github.com/nginx/unit-wasm/blob/master/HOWTO.md) in the repository root for more details) but will
instead assume you already have a Unit with the WebAssembly language module
already running.

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
                "pass": "applications/rust-echo-test"
            }
        },
        {
            "match": {
                "uri": "/rust-upload*"
            },
            "action": {
                "pass": "applications/rust-upload-reflector"
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
        "rust-echo-test": {
            "type": "wasm",
            "module": "/path/to/unit-wasm/examples/rust/echo-request/target/wasm32-wasi/debug/rust_echo_test.wasm",
            "request_handler": "luw_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler",
            "module_init_handler": "luw_module_init_handler",
            "module_end_handler": "luw_module_end_handler"
        },
        "rust-upload-reflector": {
            "type": "wasm",
            "module": "/path/to/unit-wasm/examples/rust/upload-reflector/rust_upload_reflector.wasm",
            "request_handler": "luw_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler",
            "request_end_handler": "luw_request_end_handler",
            "response_end_handler": "luw_response_end_handler"
        }
    }
}
```

Load this config then you should be ready to try it.

```shell
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

```shell
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

```
$ clang ... -o myapp.wasm myapp.c -lunit-wasm
```

See [API-C.md](https://github.com/nginx/unit-wasm/blob/master/API-C.md) for an
overview of the API.

## License

This project is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
