Trying it out
=============

For a quick and simple 'hello world' experience, you can use docker with

```shell
$ make docker
```

which will create two images:

 1. `unit:wasm` (based on the Docker Official image, with Wasm Module)
 2. `unit:demo-wasm` (based on the Wasm image, with demo application)

Manual build instructions below.

## Prerequisites and Assumptions

You will need:
 * Modern Linux platform (might work on others, not yet tested).
 * Ability to build Unit from source.
   If you haven't done this before, please first run through the
[Building From Source how-to guide](https://unit.nginx.org/howto/source/).
 * Additional build tools (required for the demo Wasm Module)
   - clang
   - llvm
   - lld

## Building the Wasm Language Module

0. Do a test build of Unit from source ([see docs](https://unit.nginx.org/howto/source/)) with this PR/patch applied. The following steps assume you're
starting in the `unit` directory and used `./configure --prefix=$PWD/build`.

2. Download and extract the Wasmtime C API (newer versions may or may not
work). Notice that we use `$(arch)` to substitute-in the appropriate CPU
architecture. This works for **x86_64** and **aarch64** (ARM) platforms.
```
wget -O- https://github.com/bytecodealliance/wasmtime/releases/download/v11.0.0/wasmtime-v11.0.0-$(arch)-linux-c-api.tar.xz | tar Jxfv -
```

3. Configure the Wasm Language Module for Unit
```
./configure wasm --include-path=$PWD/wasmtime-v11.0.0-$(arch)-linux-c-api/include \
                 --lib-path=$PWD/wasmtime-v11.0.0-$(arch)-linux-c-api/lib --rpath
```

4. Build the Wasm Language Module
```
make
```

5. Test that **unitd** Can Load the Language Module

Run `unitd` in the foreground (attached to the console) to check that Unit
can discover and load the `wasm` Language Module at startup. You should see
console output similar to this:
```
$ $PWD/build/sbin/unitd --no-daemon --log /dev/stderr
2023/06/15 11:29:31 [info] 1#1 unit 1.31.0 started
2023/06/15 11:29:31 [info] 43#43 discovery started
2023/06/15 11:29:31 [notice] 43#43 module: wasm 0.1 "/path/to/modules/wasm.unit.so"
```

## Building the demo application

From a suitable directory...

Clone the [unit-wasm](https://github.com/nginx/unit-wasm) repository

```shell
$ git clone https://github.com/nginx/unit-wasm.git
```

Download and extract the wasi-sysroot from the [WASI SDK](https://github.com/WebAssembly/wasi-sdk)

```shell
wget -O- https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/wasi-sysroot-20.0.tar.gz | tar zxfv -
```

Next Compile the C demo Wasm Modules to `.wasm` files. This requires at least
the following; make and clang, llvm, compiler-rt, and lld from LLVM 9.0+

```shell
$ cd unit-wasm
$ make WASI_SYSROOT=../wasi-sysroot examples
```

If the above fails like

```
wasm-ld: error: cannot open /usr/lib/llvm-11/lib/clang/11.0.1/lib/wasi/libclang_rt.builtins-wasm32.a: No such file or directory
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```
Then you need to download the wasm32 clang runtime and copy it into the
location mentioned in the error message.

E.g

In the above case we would untar
*libclang_rt.builtins-wasm32-wasi-20.0.tar.gz* into
*/usr/lib/llvm-11/lib/clang/11.0.1/*

On Fedora this would be more like */usr/lib64/clang/16/*

Adjust the tar '-C ...' option accordingly below...

```shell
wget -O- https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/libclang_rt.builtins-wasm32-wasi-20.0.tar.gz | sudo tar -xvzf - -C /usr/lib/llvm-11/lib/clang/11.0.1
```

With recent enough versions of Clang (that support the -print-runtime-dir
option) you can use the following command (as root)

```shell
wget -O- https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/libclang_rt.builtins-wasm32-wasi-20.0.tar.gz | tar --strip-components=1 -xvzf - -C $(dirname $(clang -print-runtime-dir))
```

Then try again...

If everything built OK then you should have the following two WASM modules

```
examples/c/luw-echo-request.wasm
examples/c/luw-upload-reflector.wasm
```

## Configure Unit to run the demo application

```json
  {
    "listeners": {
        "[::1]:8080": {
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
            "module_end_handler": "luw_module_end_handler",
            "access": {
                "filesystem": [
                    "/tmp",
                    "/foo/bar"
                ]
            }
        },
        "luw-upload-reflector": {
            "type": "wasm",
            "module": "/path/to/unit-wasm/examples/c/luw-upload-reflector.wasm",
            "request_handler": "luw_request_handler",
            "malloc_handler": "luw_malloc_handler",
            "free_handler": "luw_free_handler",
            "request_end_handler": "luw_request_end_handler",
            "response_end_handler": "luw_response_end_handler"
        }
    }
}

```

Apply the above configuration to the **/config** URI of Unit's Control API.
With the JSON in a file, you can use the CLI to apply it.
```
cat conf.json | tools/unitc /config
```

The following messages should then appear in the Unit log file (or console if
running with `--no-daemon`).
```
2023/07/26 13:28:14 [info] 182585#182585 "luw-echo-request" prototype started
2023/07/26 13:28:14 [info] 182590#182590 "luw-echo-request" application started
2023/07/26 13:28:14 [info] 182591#182591 "luw-upload-reflector" prototype started
2023/07/26 13:28:14 [info] 182596#182596 "luw-upload-reflector" application started
```

Now make a request to the demo application.
```
curl http://localhost:8080/echo
```
