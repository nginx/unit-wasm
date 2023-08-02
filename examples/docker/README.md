Unit-Wasm demo
==============

## Build the docker images

From the repository root, run

```shell
$ make docker
```

This builds two docker images.

### 1. unit:wasm

This image is based on the Docker Official Images for Unit 1.30 with a fresh
build of unitd and the experimental Wasm module. Wasmtime is included as a
shared object.

### 2. unit:demo-wasm

This image is based on the new `unit:wasm` image created above. It includes
a demo application written in C and compiled to wasm.

## Run the demo

```shell
$ docker run -d -p 9000:80 unit:demo-wasm
$ curl localhost:9000
$ curl localhost:9000/echo
```
