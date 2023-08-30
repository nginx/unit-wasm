# libunit-wasm C API

C Library for creating WebAssembly modules for use with NGINX Unit.

```C
#include <unit/unit-wasm.h>
```

1. [libunit-wasm C API](#libunit-wasm-c-api)
2. [Macros](#macros)
  * [Version](#version)
  * [Misc](#misc)
3. [Types](#types)
4. [Enums](#enums)
5. [Structs](#structs)
6. [Function Handlers](#function-handlers)
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
7. [Functions](#functions)
  * [luw_init_ctx](#luw_init_ctx)
  * [luw_set_req_buf](#luw_set_req_buf)
  * [luw_get_http_path](#luw_get_http_path)
  * [luw_get_http_method](#luw_get_http_method)
  * [luw_get_http_version](#luw_get_http_version)
  * [luw_get_http_query](#luw_get_http_query)
  * [luw_get_http_remote](#luw_get_http_remote)
  * [luw_get_http_local_addr](#luw_get_http_local_addr)
  * [luw_get_http_local_port](#luw_get_http_local_port)
  * [luw_get_http_server_name](#luw_get_http_server_name)
  * [luw_get_http_content](#luw_get_http_content)
  * [luw_get_http_content_len](#luw_get_http_content_len)
  * [luw_get_http_content_sent](#luw_get_http_content_sent)
  * [luw_get_http_total_content_sent](#luw_get_http_total_content_sent)
  * [luw_http_is_tls](#luw_http_is_tls)
  * [luw_http_hdr_iter](#luw_http_hdr_iter)
  * [luw_http_hdr_get_value](#luw_http_hdr_get_value)
  * [luw_get_response_data_size](#luw_get_response_data_size)
  * [luw_mem_writep](#luw_mem_writep)
  * [luw_mem_writep_data](#luw_mem_writep_data)
  * [luw_req_buf_append](#luw_req_buf_append)
  * [luw_mem_fill_buf_from_req](#luw_mem_fill_buf_from_req)
  * [luw_mem_reset](#luw_mem_reset)
  * [luw_http_send_response](#luw_http_send_response)
  * [luw_http_init_headers](#luw_http_init_headers)
  * [luw_http_add_header](#luw_http_add_header)
  * [luw_http_send_headers](#luw_http_send_headers)
  * [luw_http_response_end](#luw_http_response_end)
  * [luw_mem_get_init_size](#luw_mem_get_init_size)
  * [luw_foreach_http_hdr](#luw_foreach_http_hdr)
8. [Misc. Functions](#misc-functions)
  * [luw_malloc](#luw_malloc)
  * [luw_free](#luw_free)

## Macros

### Version

```C
#define LUW_VERSION_MAJOR   M
#define LUW_VERSION_MINOR   m
#define LUW_VERSION_PATCH   p
```

```C
/* Version number in hex 0xMMmmpp00 */
#define LUW_VERSION_NUMBER \
        ( (LUW_VERSION_MAJOR << 24) | \
          (LUW_VERSION_MINOR << 16) | \
          (LUW_VERSION_PATCH << 8) )
```

### Misc

```C
#define __luw_export_name(name) __attribute__((export_name(name)))
```

```C
#define __luw_unused            __attribute__((unused))
#define __luw_maybe_unused      __luw_unused
```

```C
#define luw_foreach_http_hdr(ctx, iter, name, value) \
        for (iter = ctx.req->fields, \
             name = (const char *)ctx.req + iter->name_off; \
             (iter < (ctx.req->fields + ctx.req->nr_fields)) && \
             (value = (const char *)ctx.req + iter->value_off); \
             iter++, name = (const char *)ctx.req + iter->name_off)
```

## Types

```C
typedef uint64_t u64;
typedef int64_t  s64;
typedef uint32_t u32;
typedef int32_t  s32;
typedef uint16_t u16;
typedef int16_t  s16;
typedef uint8_t   u8;
typedef int8_t    s8;
```

## Enums

```C
typedef enum {
        LUW_SRB_NONE = 0x00,
        LUW_SRB_APPEND = 0x01,
        LUW_SRB_ALLOC = 0x02,
        LUW_SRB_FULL_SIZE = 0x04,

        LUW_SRB_FLAGS_ALL = (LUW_SRB_NONE|LUW_SRB_APPEND|LUW_SRB_ALLOC|
                             LUW_SRB_FULL_SIZE)
} luw_srb_flags_t;
```

## Structs

```C
struct luw_hdr_field {
        u32 name_off;
        u32 name_len;
        u32 value_off;
        u32 value_len;
};
```

```C
struct luw_req {
        u32 method_off;
        u32 method_len;
        u32 version_off;
        u32 version_len;
        u32 path_off;
        u32 path_len;
        u32 query_off;
        u32 query_len;
        u32 remote_off;
        u32 remote_len;
        u32 local_addr_off;
        u32 local_addr_len;
        u32 local_port_off;
        u32 local_port_len;
        u32 server_name_off;
        u32 server_name_len;

        u32 content_off;
        u32 content_len;
        u32 content_sent;
        u32 total_content_sent;

        u32 request_size;

        u32 nr_fields;

        u32 tls;

        struct luw_hdr_field fields[];
};
```

```C
struct luw_resp {
        u32 size;

        u8 data[];
};
```

```C
struct luw_resp_hdr {
        u32 nr_fields;

        struct luw_hdr_field fields[];
};
```

```C
typedef struct {
        /* pointer to the shared memory */
        u8 *addr;

        /* points to the end of ctx->resp->data */
        u8 *mem;

        /* struct luw_req representation of the shared memory */
        struct luw_req *req;

        /* struct luw_resp representation of the shared memory */
        struct luw_resp *resp;

        /* struct luw_resp_hdr representation of the shared memory */
        struct luw_resp_hdr *resp_hdr;

        /* offset to where the struct resp starts in the shared memory */
        size_t resp_offset;

        /* points to the external buffer used for a copy of the request */
        u8 *req_buf;

        /* points to the end of the fields array in struct luw_resp_hdr */
        u8 *hdrp;

        /* points to the end of ctx->req_buf */
        u8 *reqp;

        /* tracks the response header index number */
        s32 resp_hdr_idx;
} luw_ctx_t;
```

```C
typedef struct luw_hdr_field luw_http_hdr_iter_t;
```

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

```C
__attribute__((export_name("luw_request_handler"), __weak__))
int luw_request_handler(u8 *addr);
```

This is called by Unit during a request. It may be called multiple times for
a single HTTP request if there is more request data than the available memory
for host <--> module communications.

You will need to provide your own implementation of this function.

It receives the base address of the shared memory. Essentially what is
returned by luw_malloc_handler().

This memory will contain a *struct luw_req*.

It returns an int, this is currently ignored but will likely be used to
indicate a HTTP status code.

#### luw_malloc_handler

```C
__attribute__((export_name("luw_malloc_handler"), __weak__))
u32 luw_malloc_handler(size_t size);
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

```C
__attribute__((export_name("luw_free_handler"), __weak__))
void luw_free_handler(u32 addr);
```

This is called by Unit when it shuts down the WebAssembly language module and
free's the memory previously allocated by luw_malloc_handler().

It receives the address of the memory to free.

An implementation of this function is provided ready for use that calls
free(3), in which case it receives the address that was previously returned
by luw_malloc_handler().

### Optional

#### luw_module_init_handler

```C
__attribute__((export_name("luw_module_init_handler"), __weak__))
void luw_module_init_handler(void);
```

This is called by Unit when it loads the WebAssembly language module.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

#### luw_module_end_handler

```C
__attribute__((export_name("luw_module_end_handler"), __weak__))
void luw_module_end_handler(void);
```

This is called by Unit when it shuts down the WebAssembly language module.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

#### luw_request_init_handler

```C
__attribute__((export_name("luw_request_init_handler"), __weak__))
void luw_request_init_handler(void);
```

This is called by Unit at the start of nxt_wasm_request_handler(), i.e at the
start of a new request.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

#### luw_request_end_handler

```C
__attribute__((export_name("luw_request_end_handler"), __weak__))
void luw_request_end_handler(void);
```

This is called by Unit at the end of nxt_wasm_request_handler(), i.e at the
end of a request.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

#### luw_response_end_handler

```C
__attribute__((export_name("luw_response_end_handler"), __weak__))
void luw_response_end_handler(void);
```

This is called by Unit after luw_http_response_end() has been called.

A default dummy function is provided. If this handler is not required, there
is no need to specify it in the Unit config.

## Functions

### luw_init_ctx

```C
void luw_init_ctx(luw_ctx_t *ctx, u8 *addr, size_t offset);
```

This function sets up a *luw_ctx_t* context structure, this contains stuff
required all throughout the API. It's a typedef for opaqueness and you should
not in general be concerned with its contents.

It take a pointer to a stack allocated luw_ctx_t, this will be zeroed and
have various members initialised.

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

That will be covered in more detail by the next function, luw_set_req_buf().

Now back to _offset_, it may be convenient to put the response headers at the
beginning of this memory and then put the response after it, rather than
doing the headers and then doing the response as separate steps, if the
headers depends on some aspect of the response, its size for example and
Content-Length.

Example

```C
luw_ctx_t ctx;
/* ... */
luw_init_ctx(&ctx, addr, 4096 /* Response offset */);
```

### luw_set_req_buf

```C
int luw_set_req_buf(luw_ctx_t *ctx, u8 **buf, unsigned int flags);
```

This function is used to take a copy of the request buffer (as discussed
above).

This takes a previously initialised (with luw_init_ctx()) luw_ctx_t.

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

```C
static u8 *request_buf;
*/ ... */
int luw_request_handler(u8 *addr)
{
        if (!request_buf) {
                luw_init_ctx(&ctx, addr, 0);
                /*
                 * Take a copy of the request and use that, we do this
                 * in APPEND mode so we can build up request_buf from
                 * multiple requests.
                 *
                 * Just allocate memory for the total amount of data we
                 * expect to get, this includes the request structure
                 * itself as well as any body content.
                 */
                luw_set_req_buf(&ctx, &request_buf,
                                LUW_SRB_APPEND|LUW_SRB_ALLOC|LUW_SRB_FULL_SIZE);
        } else {
                luw_req_buf_append(&ctx, addr);
        }

        /* operate on the request (ctx) */

        return 0;
}
```

That example is taken from the [luw-upload-reflector.c](https://github.com/nginx/unit-wasm/blob/main/examples/c/luw-upload-reflector.c) demo module. For a
simpler example see [luw-echo-request.c](https://github.com/nginx/unit-wasm/blob/main/examples/c/luw-echo-request.c)

### luw_get_http_path

```C
const char *luw_get_http_path(const luw_ctx_t *ctx);
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

### luw_get_http_method

```C
const char *luw_get_http_method(const luw_ctx_t *ctx);
```

This function returns a pointer to the HTTP method.

E.g

```
GET
```

### luw_get_http_version

```C
const char *luw_get_http_version(const luw_ctx_t *ctx);
```

This function returns a pointer to the HTTP version.

E.g

```
1.1
```

### luw_get_http_query

```C
const char *luw_get_http_query(const luw_ctx_t *ctx);
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

### luw_get_http_remote

```C
const char *luw_get_http_remote(const luw_ctx_t *ctx);
```

This function returns a pointer to the remote/client/peer address.

E.g

```
2001:db8::f00
```

### luw_get_http_local_addr

```C
const char *luw_get_http_local_addr(const luw_ctx_t *ctx);
```

This function returns a pointer to the local/server address.

E.g

```
2001:db8::1
```

### luw_get_http_local_port

```C
const char *luw_get_http_local_port(const luw_ctx_t *ctx);
```

This function returns a pointer to the local/server port.

E.g

```
443
```

### luw_get_http_server_name

```C
const char *luw_get_http_server_name(const luw_ctx_t *ctx);
```

This function returns a pointer to the local/server name.

E.g

```
www.example.com
```

### luw_get_http_content

```C
const u8 *luw_get_http_content(const luw_ctx_t *ctx);
```

This function returns a pointer to the start of the request body.

### luw_get_http_content_len

```C
size_t luw_get_http_content_len(const luw_ctx_t *ctx);
```

This function returns the size of the overall content. I.e Content-Length.


### luw_get_http_content_sent

```C
size_t luw_get_http_content_sent(const luw_ctx_t *ctx);
```

This function returns the length of the content that was sent to the
WebAssembly module in _this_ request. Remember, a single HTTP request may be
split over several calls to luw_request_handler().

### luw_get_http_total_content_sent

```C
size_t luw_get_http_total_content_sent(const luw_ctx_t *ctx);
```

This function returns the total length of the content that was sent to the
WebAssembly module so far. Remember, a single HTTP request may be split over
several calls to luw_request_handler().

_Version: 0.2.0_

### luw_http_is_tls

```C
bool luw_http_is_tls(const luw_ctx_t *ctx);
```

This function returns _true_ if the connection to Unit was made over TLS.

### luw_http_hdr_iter

```C
void luw_http_hdr_iter(luw_ctx_t *ctx,
                       bool (*luw_http_hdr_iter_func)(luw_ctx_t *ctx,
                                                      const char *name,
                                                      const char *value,
                                                      void *data),
                       void *user_data)
```

This function allows to iterate over the HTTP headers. For each header it
will call the given luw_http_hdr_iter_func() function whose prototype is

```C
bool luw_http_hdr_iter_func(luw_ctx_t *ctx,
                            const char *name, const char *value, void *data);
```

You may call this function whatever you like. For each header it will be
passed the *luw_ctx_t*, the header name, its value and a user specified
pointer if any, can be NULL.

Returning _true_ from this function will cause the iteration process to
continue, returning _false_ will terminate it.

Example

```C
static bool hdr_iter_func(luw_ctx_t *ctx, const char *name, const char *value,
                          void *user_data __luw_unused)
{
        /* Do something with name & value */

        /* Continue iteration or return false to stop */
        return true;
}

/* ... *

luw_http_hdr_iter(&ctx, hdr_iter_func, NULL);
```

### luw_http_hdr_get_value

```C
const char *luw_http_hdr_get_value(const luw_ctx_t *ctx, const char *hdr);
```

Given a HTTP header _hdr_ this function will look it up in the request and
return its value if found, otherwise _NULL_.

The lookup is done case insensitively.

### luw_get_response_data_size

```C
size_t luw_get_response_data_size(const luw_ctx_t *ctx);
```

This function returns the size of the response data written to memory.

### luw_mem_writep

```C
__attribute__((__format__(printf, 2, 3)))
int luw_mem_writep(luw_ctx_t *ctx, const char *fmt, ...);
```

This function is a cross between vasprintf(3) and mempcpy(3).

It takes a format argument and zero or more arguments that will be
substituted into the format string.

It then appends this formatted string to the memory. Note this string will
_not_ be nul terminated. Unit does not expect this response data to be nul
terminated and we track the size of the response and return that to Unit.

This function returns -1 on error or the length of the string written.

### luw_mem_writep_data

```C
size_t luw_mem_writep_data(luw_ctx_t *ctx, const u8 *src, size_t size);
```

This function just appends _size_ bytes from _src_ to the response.

It returns the new size of the response.

### luw_req_buf_append

```C
void luw_req_buf_append(luw_ctx_t *ctx, const u8 *src);
```

This function appends the request data contained in _src_ to the previously
setup *request_buffer* with luw_set_req_buf().

This function would be used after an initial request to append the data from
subsequent requests to the request_buffer.

Example

```C
int luw_request_handler(u8 *addr)
{
        if (!request_buf) {
                luw_init_ctx(&ctx, addr, 0);
                /*
                 * Take a copy of the request and use that, we do this
                 * in APPEND mode so we can build up request_buf from
                 * multiple requests.
                 *
                 * Just allocate memory for the total amount of data we
                 * expect to get, this includes the request structure
                 * itself as well as any body content.
                 */
                luw_set_req_buf(&ctx, &request_buf,
                                LUW_SRB_APPEND|LUW_SRB_ALLOC|LUW_SRB_FULL_SIZE);
        } else {
                luw_req_buf_append(&ctx, addr);
        }

        /* Do something with the request (ctx) */

        return 0;
}
```

### luw_mem_fill_buf_from_req

```C
size_t luw_mem_fill_buf_from_req(luw_ctx_t *ctx, size_t from);
```

This is a convenience function to fill the response buffer with data from
the request buffer.

_from_ is basically the offset in the request_buffer where to start copying
data from.

Example

```C
/* ... */
write_bytes = luw_mem_fill_buf_from_req(ctx, total_response_sent);
total_response_sent += write_bytes;
/* ... */
```

This is taken from the [luw-upload-reflector.c](https://github.com/nginx/unit-wasm/blob/main/examples/c/luw-upload-reflector.c) demo module.

In this case we build up a request_buffer on each call of
luw_request_handler(), so total_response_sent grows each time by how much data
was sent in _that_ request.

Here are are sending data back to the client after each time we receive it to
demonstrate the interleaving of requests and responses from the WebAssembly
module during a single http request.

This function returns the number of bytes written to the response buffer.

### luw_mem_reset

```C
void luw_mem_reset(luw_ctx_t *ctx);
```

This function resets the response buffer size and the number of response
headers back to 0.

### luw_http_send_response

```C
void luw_http_send_response(const luw_ctx_t *ctx);
```

This function calls into Unit to send the response buffer back.

### luw_http_init_headers

```C
void luw_http_init_headers(luw_ctx_t *ctx, size_t nr, size_t offset);
```

This function is used in the preparation of sending back response headers.

_nr_ is the number of headers we are sending.

_offset_ is the offset into the response buffer where we are placing these
headers. This will usually be 0.

Example

```C
luw_http_init_headers(ctx, 2, 0);
```

### luw_http_add_header

```C
void luw_http_add_header(luw_ctx_t *ctx, const char *name, const char *value);
```

This function is used to add a header to the response.

_name_ is the name of the header.

_value_ is the value of the header.

Example

```C
char clen[32];
/* ... */
snprintf(clen, sizeof(clen), "%lu", luw_get_response_data_size(&ctx));
luw_http_add_header(&ctx, "Content-Type", "text/plain");
luw_http_add_header(&ctx, "Content-Length", clen);
```

### luw_http_send_headers

```C
void luw_http_send_headers(const luw_ctx_t *ctx);
```

This function calls into Unit and triggers the sending of the response
headers.

### luw_http_response_end

```C
void luw_http_response_end(void);
```

This function calls into Unit and tells it this is the end of the response
which will trigger Unit to send it to the client.

### luw_mem_get_init_size

```C
u32 luw_mem_get_init_size(void);
```

This function calls into Unit to get the size of the shared memory. This is
the amount of memory you should assume you have for creating responses.
Remember you can create multiple responses before calling
luw_http_response_end().

### luw_foreach_http_hdr

```C
void luw_foreach_http_hdr(luw_ctx_t ctx, luw_http_hdr_iter_t *iter,
                          const char *name, const char *value)
```

Defined as a macro, this is used to iterate over the HTTP header fields.

It takes a _luw_ctx_t *_ and a _luw_http_hdr_iter_t *_ and returns pointers
to the field name and value.

Example

```C
luw_ctx_t ctx;
luw_http_hdr_iter_t *iter;
const char *name;
const char *value;
/* ... */
luw_foreach_http_hdr(ctx, iter, name, value) {
        printf("Field name : %s, field value : %s\n", name, value);
        /* do something else with name & value */
}
```

## Misc. Functions

The following functions are convenience wrappers for the Rust bindings and
should **not** be used directly.

### luw_malloc

```C
void *luw_malloc(size_t size);
```

Straight wrapper for malloc(3).

### luw_free

```C
void luw_free(void *ptr);
```

Straight wrapper for free(3).
