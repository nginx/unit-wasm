# 'Rusty' Rust API

Rusty is a more native Rust wrapper around the auto-generated bindings to
[libunit-wasm](https://github.com/nginx/unit-wasm/blob/main/API-C.md).

```Rust
use unit_wasm::rusty::*;
```

If using

```Rust
uwr_http_hdr_iter();
```

```Rust
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_void;
```

## Naming

You will see references to functions etc starting with *luw_* or *LUW_* and
*uwr_".

**luw/LUW** (libunit-wasm) come from the underlying C library and in the Rust
case are the auto-generated bindings with a few manual additions.

**uwr** (Unit Wasm Rust aka '_rusty_') is a more Rust native wrapper ontop of
the bindings.

In _rusty_ the luw/LUW API is generally the low level stuff like the library
version macros and the various function handlers where they can be used as is
and there isn't a real need to create wrappers specifically for them.

1. ['Rusty' Rust API](#rusty-rust-api)
  * [Naming](#naming)
2. [Macros](#macros)
  * [Version](#version)
  * [String Conversion](#string-conversion)
  * [uwr_write_str!](#uwr_write_str)
3. [Function Handlers](#function-handlers)
  * [Optional](#optional)
    - [luw_module_init_handler](#luw_module_init_handler)
    - [luw_module_end_handler](#luw_module_end_handler)
    - [luw_request_init_handler](#luw_request_init_handler)
    - [luw_request_end_handler](#luw_request_end_handler)
    - [luw_response_end_handler](#luw_response_end_handler)
  * [Required](#required)
    - [luw_request_handler](#luw_request_handler)
    - [luw_free_handler](#luw_free_handler)
    - [luw_malloc_handler](#luw_malloc_handler)
4. [Functions](#functions)
  * [UWR_CTX_INITIALIZER](#uwr_ctx_initializer)
  * [uwr_init_ctx](#uwr_init_ctx)
  * [uwr_set_req_buf](#uwr_set_req_buf)
  * [uwr_get_http_path](#uwr_get_http_path)
  * [uwr_get_http_method](#uwr_get_http_method)
  * [uwr_get_http_version](#uwr_get_http_version)
  * [uwr_get_http_query](#uwr_get_http_query)
  * [uwr_get_http_remote](#uwr_get_http_remote)
  * [uwr_get_http_local_addr](#uwr_get_http_local_addr)
  * [uwr_get_http_local_port](#uwr_get_http_local_port)
  * [uwr_get_http_server_name](#uwr_get_http_server_name)
  * [uwr_get_http_content](#uwr_get_http_content)
  * [uwr_get_http_content_str](#uwr_get_http_content_str)
  * [uwr_get_http_content_len](#uwr_get_http_content_len)
  * [uwr_get_http_content_sent](#uwr_get_http_content_sent)
  * [uwr_get_http_total_content_sent](#uwr_get_http_total_content_sent)
  * [uwr_http_is_tls](#uwr_http_is_tls)
  * [uwr_http_hdr_iter](#uwr_http_hdr_iter)
  * [uwr_http_hdr_get_value](#uwr_http_hdr_get_value)
  * [uwr_get_response_data_size](#uwr_get_response_data_size)
  * [uwr_mem_write_buf](#uwr_mem_write_buf)
  * [uwr_req_buf_append](#uwr_req_buf_append)
  * [uwr_mem_fill_buf_from_req](#uwr_mem_fill_buf_from_req)
  * [uwr_mem_reset](#uwr_mem_reset)
  * [uwr_http_send_response](#uwr_http_send_response)
  * [uwr_http_init_headers](#uwr_http_init_headers)
  * [uwr_http_add_header](#uwr_http_add_header)
  * [uwr_http_add_header_content_type](#uwr_http_add_header_content_type)
  * [uwr_http_add_header_content_len](#uwr_http_add_header_content_len)
  * [uwr_http_send_headers](#uwr_http_send_headers)
  * [uwr_http_response_end](#uwr_http_response_end)
  * [uwr_mem_get_init_size](#uwr_mem_get_init_size)
5. [Misc. Functions](#misc-functions)
  * [uwr_malloc](#uwr_malloc)
  * [uwr_free](#uwr_free)

## Macros

### Version

For the underlying libunit-wasm version.

```Rust
pub const LUW_VERSION_MAJOR: i32;
pub const LUW_VERSION_MINOR: i32;
pub const LUW_VERSION_PATCH: i32;
```

```Rust
/* Version number in hex 0xMMmmpp00 */
pub const LUW_VERSION_NUMBER: i32 =
    (LUW_VERSION_MAJOR << 24) | \
    (LUW_VERSION_MINOR << 16) | \
    (LUW_VERSION_PATCH << 8);
```

### String Conversion

```Rust
C2S!(string);
```

Converts a C string into a Rust String

Main use is internally and in the *uwr_http_hdr_iter()* callback function,
e.g

```Rust
pub extern "C" fn hdr_iter_func(
    ctx: *mut luw_ctx_t,
    name: *const c_char,
    value: *const c_char,
    _data: *mut c_void,
) -> bool {
    uwr_write_str!(ctx, "{} = {}\n", C2S!(name), C2S!(value));

    return true;
}
```

Example taken from the
[echo-request](https://github.com/nginx/unit-wasm/blob/main/examples/rust/echo-request/src/lib.rs)
Wasm demo module

```Rust
S2C!(formatted string);
```

Converts a Rust String, with optional formatting, to a C string.

Used internally.

### uwr_write_str!

```Rust
uwr_write_str!*ctx, fmt, ...);
```

This is essentially a wrapper around
[luw_mem_writep_data()](https://github.com/nginx/unit-wasm/blob/main/API-C.md#luw_mem_writep_data)

It is the main way to write responses back to the client.

It takes the luw_ctx_t context pointer, a string that will be run through the
[format!()](https://doc.rust-lang.org/std/macro.format.html) macro and any
optional arguments.

## Function Handlers

These functions are exported from the WebAssembly module and are called from
the WebAssembly runtime (the Unit WebAssembly language module in this case).

There are two types of handlers; required & optional.

luw_request_handler(), luw_malloc_handler() & luw_free_handler() are required
with the rest being optional.

libunit-wasm includes exports for these handlers and some default
implementations.

These functions are defined as _weak_ symbols and so if a developer writes
their own function of the same name, that will take precedence.

However, developers are under no obligation to use these and can create their
own with any (valid) names they like.

Whatever names developers choose, they are specified in the Unit config.

## Required

#### luw_request_handler

```Rust
#[no_mangle]
pub extern "C" fn luw_request_handler(addr: *mut u8) -> i32;
```

This is called by Unit during a request. It may be called multiple times for
a single HTTP request if there is more request data than the available memory
for host <--> module communications.

You will need to provide your own implementation of this function.

It receives the base address of the shared memory. Essentially what is
returned by luw_malloc_handler().

It returns an int, this is currently ignored but will likely be used to
indicate a HTTP status code.

#### luw_malloc_handler

```Rust
#[no_mangle]
pub extern "C" fn luw_malloc_handler(size: usize) -> u32;
```

This is called by Unit when it loads the WebAssembly language module. This
provides the shared memory used for host <--> module communications.

It receives the desired size of the memory, which is currently
NXT_WASM_MEM_SIZE + NXT_WASM_PAGE_SIZE.

However calls to luw_mem_get_init_size() will return just NXT_WASM_MEM_SIZE
(which is currently 32MiB). The extra NXT_WASM_PAGE_SIZE is to cater for
structure sizes in the response so developers can generally assume they have
the full NXT_WASM_MEM_SIZE for their data.

A default implementation of this function is provided ready for use that
calls malloc(3).

#### luw_free_handler

```Rust
#[no_mangle]
pub extern "C" fn luw_free_handler(addr: u32);
```

This is called by Unit when it shuts down the WebAssembly language module and
free's the memory previously allocated by luw_malloc_handler().

It receives the address of the memory to free.

An implementation of this function is provided ready for use that calls
free(3), in which case it receives the address that was previously returned
by luw_malloc_handler().

### Optional

#### luw_module_init_handler

```Rust
#[no_mangle]
pub extern "C" fn luw_module_init_handler();
```

This is called by Unit when it loads the WebAssembly language module.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

#### luw_module_end_handler

```Rust
#[no_mangle]
pub extern "C" fn luw_module_end_handler();
```

This is called by Unit when it shuts down the WebAssembly language module.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

#### luw_request_init_handler

```Rust
#[no_mangle]
pub extern "C" fn luw_request_init_handler();
```

This is called by Unit at the start of nxt_wasm_request_handler(), i.e at the
start of a new request.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

#### luw_request_end_handler

```Rust
#[no_mangle]
pub extern "C" fn luw_request_end_handler();
```

This is called by Unit at the end of nxt_wasm_request_handler(), i.e at the
end of a request.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

#### luw_response_end_handler

```Rust
#[no_mangle]
pub extern "C" fn luw_response_end_handler();
```

This is called by Unit after luw_http_response_end() has been called.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

## Functions

### UWR_CTX_INITIALIZER

```Rust
pub const fn UWR_CTX_INITIALIZER() -> luw_ctx_t;
```

Used to initialise a luw_ctx_t context structure. E.g

```Rust
let ctx = &mut UWR_CTX_INITIALIZER();
```

### uwr_init_ctx

```Rust
pub fn uwr_init_ctx(ctx: *mut luw_ctx_t, addr: *mut u8, offset: usize);
```

This function sets up a *luw_ctx_t* context structure, this contains stuff
required all throughout the API.

**addr** is a pointer to the shared memory as passed into luw_request_handler().

**offset** is where in the shared memory it should start writing the response.

#### A quick word about memory

The way the Unit WebAssembly language module (the host/runtime) and the
WebAssembly module you want to write (the guest) communicate is via a chunk
of shared memory.

This shared memory is simply the modules (guest) address space from which we
can allocate a chunk. How this memory is laid out varies on how the module
is built.

With clang/linker flags of -Wl,--stack-first -Wl,-z,stack-size=$((8*1024*1024))
we get a memory layout something like

```
  |----------------------------------------------------------------------|
  |                     |             |                                  |
  |       <-- Stack     | Global Data |      Heap -->                    |
  |                     |             |                                  |
  |----------------------------------------------------------------------|
  0                     0x800000                               0x100000000

            WebAssembly Module Linear Memory / Process Memory Layout
```

(The above is assuming _--target=wasm32-wasi_, i.e 32bit)

A chunk of memory from the heap is allocated at Unit WebAssembly language
module startup.

We currently use this same chunk of memory for both requests and responses.
This means that depending on what you're doing, you'll want to take a copy
of the request (and remember luw_request_handler() may be called multiple
times for a single http request).

That will be covered in more detail by the next function, uwr_set_req_buf().

Now back to _offset_, it may be convenient to put the response headers at the
beginning of this memory and then put the response after it, rather than
doing the headers and then doing the response as separate steps, if the
headers depends on some aspect of the response, its size for example and
Content-Length.

Example

```Rust
#[no_mangle]
pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
    let ctx = &mut UWR_CTX_INITIALIZER();
    /* ... */
    uwr_init_ctx(ctx, addr, 4096 /* Response offset */);
```

### uwr_set_req_buf

```Rust
pub fn uwr_set_req_buf(
    ctx: *mut luw_ctx_t,
    buf: *mut *mut u8,
    flags: u32,
) -> i32;
```

This function is used to take a copy of the request buffer (as discussed
above).

This takes a previously initialised (with uwr_init_ctx()) luw_ctx_t.

**buf** is a buffer where the request data will written.

**flags** can be some combination (OR'd) of the following

**LUW_SRB_NONE**

No specific action to be performed. It will simply copy the request data
into the specified buffer.

**LUW_SRB_APPEND**

Sets up append mode whereby multiple successive requests will be appended
to the specified buffer.

The first request will have all its metadata copied. Subsequent requests
will _only_ have the actual body data appended.

**LUW_SRB_ALLOC**

Allocate memory for the specified buffer.

**LUW_SRB_FULL_SIZE**

Used in conjunction with *LUW_SRB_ALLOC*. By default only
*ctx->req->request_size* is allocated. If this flag is present it says to
allocate memory for the _entire_ request that will eventually be sent.

Example

```Rust
static mut CTX: luw_ctx_t = UWR_CTX_INITIALIZER();

static mut REQUEST_BUF: *mut u8 = null_mut();
*/ ... */
#[no_mangle]
pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
    let ctx: *mut luw_ctx_t = unsafe { &mut CTX };

    if unsafe { REQUEST_BUF.is_null() } {
        uwr_init_ctx(ctx, addr, 0 /* Response offset */);
        /*
         * Take a copy of the request and use that, we do this
         * in APPEND mode so we can build up request_buf from
         * multiple requests.
         *
         * Just allocate memory for the total amount of data we
         * expect to get, this includes the request structure
         * itself as well as any body content.
         */
        uwr_set_req_buf(
            ctx,
            unsafe { &mut REQUEST_BUF },
            LUW_SRB_APPEND | LUW_SRB_ALLOC | LUW_SRB_FULL_SIZE,
        );
    } else {
        uwr_req_buf_append(ctx, addr);
    }

    upload_reflector(ctx);

    return 0;
}
```

That example is taken from the
[upload-reflector demo](https://github.com/nginx/unit-wasm/blob/main/examples/rust/upload-reflector/src/lib.rs)
demo module. For a simpler example see the
[echo-request demo](https://github.com/nginx/unit-wasm/blob/main/examples/rust/echo-request/src/lib.rs)

### uwr_get_http_path

```Rust
pub fn uwr_get_http_path(ctx: *const luw_ctx_t) -> &'static str;
```

This function returns a pointer to the HTTP request path.

E.g

Given a request of
```
http://localhost:8080/echo/?q=a
```
this function will return
```
/echo/?q=a
```

### uwr_get_http_method

```Rust
pub fn uwr_get_http_method(ctx: *const luw_ctx_t) -> &'static str;
```

This function returns a pointer to the HTTP method.

E.g

```
GET
```

### uwr_get_http_version

```Rust
pub fn uwr_get_http_version(ctx: *const luw_ctx_t) -> &'static str;
```

This function returns a pointer to the HTTP version.

E.g

```
1.1
```

### uwr_get_http_query

```Rust
pub fn uwr_get_http_query(ctx: *const luw_ctx_t) -> &'static str;
```

This function returns a pointer to the query string (empty string for no query
string).

E.g

Given a request of
```
http://localhost:8080/echo/?q=a
```
this function will return
```
q=a
```

### uwr_get_http_remote

```Rust
pub fn uwr_get_http_remote(ctx: *const luw_ctx_t) -> &'static str;
```

This function returns a pointer to the remote/client/peer address.

E.g

```
2001:db8::f00
```

### uwr_get_http_local_addr

```Rust
pub fn uwr_get_http_local_addr(ctx: *const luw_ctx_t) -> &'static str;
```

This function returns a pointer to the local/server address.

E.g

```
2001:db8::1
```

### uwr_get_http_local_port

```Rust
pub fn uwr_get_http_local_port(ctx: *const luw_ctx_t) -> &'static str;
```

This function returns a pointer to the local/server port.

E.g

```
443
```

### uwr_get_http_server_name

```Rust
pub fn uwr_get_http_server_name(ctx: *const luw_ctx_t) -> &'static str;
```

This function returns a pointer to the local/server name.

E.g

```
www.example.com
```

### uwr_get_http_content

```Rust
pub fn uwr_get_http_content(ctx: *const luw_ctx_t) -> *const u8;
```

This function returns a pointer to the start of the request body.

### uwr_get_http_content_str

```Rsut
pub fn uwr_get_http_content_str(ctx: *const luw_ctx_t) -> &'static str;
```

Same as above but returns a Rust str.

_Version: 0.2.0_

### uwr_get_http_content_len

```Rust
pub fn uwr_get_http_content_len(ctx: *const luw_ctx_t) -> usize;
```

This function returns the size of the overall content. I.e Content-Length.


### uwr_get_http_content_sent

```Rust
pub fn uwr_get_http_content_sent(ctx: *const luw_ctx_t) -> usize;
```

This function returns the length of the content that was sent to the
WebAssembly module in _this_ request. Remember, a single HTTP request may be
split over several calls to luw_request_handler().

### uwr_get_http_total_content_sent

```Rust
pub fn uwr_get_http_total_content_sent(ctx: *const luw_ctx_t) -> usize;
```

This function returns the total length of the content that was sent to the
WebAssembly module so far. Remember, a single HTTP request may be split over
several calls to luw_request_handler().

_Version: 0.2.0_

### uwr_http_is_tls

```Rust
pub fn uwr_http_is_tls(ctx: *const luw_ctx_t) -> bool;
```

This function returns _true_ if the connection to Unit was made over TLS.

### uwr_http_hdr_iter

```Rust
pub fn uwr_http_hdr_iter(
    ctx: *mut luw_ctx_t,
    luw_http_hdr_iter_func: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut luw_ctx_t,
            name: *const c_char,
            value: *const c_char,
            data: *mut c_void,
        ) -> bool,
    >,
    user_data: *mut c_void,
);
```

This function allows to iterate over the HTTP headers. For each header it
will call the given luw_http_hdr_iter_func() function whose prototype is

```Rust
pub extern "C" fn hdr_iter_func(
    ctx: *mut luw_ctx_t,
    name: *const c_char,
    value: *const c_char,
    data: *mut c_void,
) -> bool;
```

You may call this function whatever you like. For each header it will be
passed the *luw_ctx_t*, the header name, its value and a user specified
pointer if any, can be NULL.

Returning _true_ from this function will cause the iteration process to
continue, returning _false_ will terminate it.

Example

```Rust
pub extern "C" fn hdr_iter_func(
    ctx: *mut luw_ctx_t,
    name: *const c_char,
    value: *const c_char,
    _data: *mut c_void,
) -> bool {
    /* Do something with name & value, ignoring data */

    return true;
}

/* ... *

uwr_http_hdr_iter(ctx, Some(hdr_iter_func), null_mut());
```

### uwr_http_hdr_get_value

```Rust
pub fn uwr_http_hdr_get_value(ctx: *const luw_ctx_t, hdr: &str) -> &'static str;
```

Given a HTTP header _hdr_ this function will look it up in the request and
return its value if found, otherwise _NULL_.

The lookup is done case insensitively.

### uwr_get_response_data_size

```Rust
pub fn uwr_get_response_data_size(ctx: *const luw_ctx_t) -> usize;
```

This function returns the size of the response data written to memory.

### uwr_mem_write_buf

```Rust
pub fn uwr_mem_write_buf(
    ctx: *mut luw_ctx_t,
    src: *const u8,
    size: usize,
) -> usize;
```

This function just appends _size_ bytes from _src_ to the response.

It returns the new size of the response.

### uwr_req_buf_append

```Rust
pub fn uwr_req_buf_append(ctx: *mut luw_ctx_t, src: *const u8);
```

This function appends the request data contained in _src_ to the previously
setup *request_buffer* with uwr_set_req_buf().

This function would be used after an initial request to append the data from
subsequent requests to the request_buffer.

Example

```Rust
#[no_mangle]
pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
    let ctx: *mut luw_ctx_t = unsafe { &mut CTX };

    if unsafe { REQUEST_BUF.is_null() } {
        uwr_init_ctx(ctx, addr, 0 /* Response offset */);
        /*
         * Take a copy of the request and use that, we do this
         * in APPEND mode so we can build up request_buf from
         * multiple requests.
         *
         * Just allocate memory for the total amount of data we
         * expect to get, this includes the request structure
         * itself as well as any body content.
         */
        uwr_set_req_buf(
            ctx,
            unsafe { &mut REQUEST_BUF },
            LUW_SRB_APPEND | LUW_SRB_ALLOC | LUW_SRB_FULL_SIZE,
        );
    } else {
        uwr_req_buf_append(ctx, addr);
    }

    upload_reflector(ctx);

    return 0;
}
```

### uwr_mem_fill_buf_from_req

```Rust
pub fn uwr_req_buf_append(ctx: *mut luw_ctx_t, src: *const u8);
```

This is a convenience function to fill the response buffer with data from
the request buffer.

_from_ is basically the offset in the request_buffer where to start copying
data from.

Example

```Rust
/* ... */
write_bytes = uwr_mem_fill_buf_from_req(ctx, TOTAL_RESPONSE_SENT);
TOTAL_RESPONSE_SENT += write_bytes;
/* ... */
```

This is taken from the
[upload-reflector demo](https://github.com/nginx/unit-wasm/blob/main/examples/c/upload-reflector/src/lib.rs)
demo module.

In this case we build up a request_buffer on each call of
luw_request_handler(), so TOTAL_RESPONSE_SENT grows each time by how much
data was sent in _that_ request.

Here are are sending data back to the client after each time we receive it to
demonstrate the interleaving of requests and responses from the WebAssembly
module during a single http request.

This function returns the number of bytes written to the response buffer.

### uwr_mem_reset

```Rust
pub fn uwr_luw_mem_reset(ctx: *mut luw_ctx_t);
```

This function resets the response buffer size and the number of response
headers back to 0.

### uwr_http_send_response

```Rust
pub fn uwr_http_send_response(ctx: *const luw_ctx_t);
```

This function calls into Unit to send the response buffer back.

### uwr_http_init_headers

```Rust
pub fn uwr_http_init_headers(ctx: *mut luw_ctx_t, nr: usize, offset: usize);
```

This function is used in the preparation of sending back response headers.

_nr_ is the number of headers we are sending.

_offset_ is the offset into the response buffer where we are placing these
headers. This will usually be 0.

Example

```Rust
uwr_http_init_headers(ctx, 2, 0);
```

### uwr_http_add_header

```Rust
pub fn uwr_http_add_header(
    ctx: *mut luw_ctx_t,
    name: &str,
    value: &str,
);
```

This function is used to add a header to the response.

_name_ is the name of the header.

_value_ is the value of the header.

Example

```Rust
uwr_http_add_header(&ctx, "Content-Type", "text/plain");
uwr_http_add_header(
    ctx,
    "Content-Length",
    &format!("{}", uwr_get_response_data_size(ctx)),
);
```

### uwr_http_add_header_content_type

```Rust
pub fn uwr_http_add_header_content_type(ctx: *mut luw_ctx_t, ctype: &str);
```

A convenience function for setting the 'Content-Type' response header.
E.g the above example that adds the _Content-Type_ header could be
written as

```Rust
uwr_http_add_header_content_type(ctx, "text/plain");
```

_Version: 0.2.0_

### uwr_http_add_header_content_len

```Rust
pub fn uwr_http_add_header_content_len(ctx: *mut luw_ctx_t);
```

A convenience function for setting the 'Content-Length' response header.
E.g the above example that adds the _Content-Length_ header could be
written as

```Rust
uwr_http_add_header_content_len(ctx);
```

This function uses [uwr_get_response_data_size](#uwr_get_response_data_size)
internally to get the size of the response data.

_Version: 0.2.0_

### uwr_http_send_headers

```Rust
pub fn uwr_http_send_headers(ctx: *const luw_ctx_t);
```

This function calls into Unit and triggers the sending of the response
headers.

### uwr_http_response_end

```Rust
pub fn uwr_http_response_end();
```

This function calls into Unit and tells it this is the end of the response
which will trigger Unit to send it to the client.

### uwr_mem_get_init_size

```Rust
pub fn uwr_mem_get_init_size() -> u32;
```

This function calls into Unit to get the size of the shared memory. This is
the amount of memory you should assume you have for creating responses.
Remember you can create multiple responses before calling
luw_http_response_end().

## Misc. Functions

The following functions are convenience wrappers for the Rust bindings and
should **not** be used directly.

### uwr_malloc

```Rust
pub fn uwr_malloc(size: u32) -> *mut u8;
```

Essentially a straight wrapper for malloc(3).

### uwr_free

```Rust
pub fn uwr_free(ptr: *mut u8);
```

Essentially a straight wrapper for free(3).
