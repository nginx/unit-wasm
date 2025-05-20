libunit-wasm C API
==================

C Library for creating WebAssembly modules for use with NGINX Unit.

.. code:: c

   #include <unit/unit-wasm.h>

1. `libunit-wasm C API <#libunit-wasm-c-api>`__
2. `Macros <#macros>`__

-  `Version <#version>`__
-  `Misc <#misc>`__

3. `Types <#types>`__
4. `Enums <#enums>`__

-  `luw_srb_flags_t <#luw_srb_flags_t>`__
-  `luw_http_status_t <#luw_http_status_t>`__

5. `Structs <#structs>`__
6. `Function Handlers <#function-handlers>`__

-  `Optional <#optional>`__

   -  `luw_module_init_handler <#luw_module_init_handler>`__
   -  `luw_module_end_handler <#luw_module_end_handler>`__
   -  `luw_request_init_handler <#luw_request_init_handler>`__
   -  `luw_request_end_handler <#luw_request_end_handler>`__
   -  `luw_response_end_handler <#luw_response_end_handler>`__

-  `Required <#required>`__

   -  `luw_request_handler <#luw_request_handler>`__
   -  `luw_free_handler <#luw_free_handler>`__
   -  `luw_malloc_handler <#luw_malloc_handler>`__

7. `Functions <#functions>`__

-  `luw_init_ctx <#luw_init_ctx>`__
-  `luw_set_req_buf <#luw_set_req_buf>`__
-  `luw_get_http_path <#luw_get_http_path>`__
-  `luw_get_http_method <#luw_get_http_method>`__
-  `luw_get_http_version <#luw_get_http_version>`__
-  `luw_get_http_query <#luw_get_http_query>`__
-  `luw_get_http_remote <#luw_get_http_remote>`__
-  `luw_get_http_local_addr <#luw_get_http_local_addr>`__
-  `luw_get_http_local_port <#luw_get_http_local_port>`__
-  `luw_get_http_server_name <#luw_get_http_server_name>`__
-  `luw_get_http_content <#luw_get_http_content>`__
-  `luw_get_http_content_len <#luw_get_http_content_len>`__
-  `luw_get_http_content_sent <#luw_get_http_content_sent>`__
-  `luw_get_http_total_content_sent <#luw_get_http_total_content_sent>`__
-  `luw_http_is_tls <#luw_http_is_tls>`__
-  `luw_http_hdr_iter <#luw_http_hdr_iter>`__
-  `luw_http_hdr_get_value <#luw_http_hdr_get_value>`__
-  `luw_get_response_data_size <#luw_get_response_data_size>`__
-  `luw_mem_writep <#luw_mem_writep>`__
-  `luw_mem_writep_data <#luw_mem_writep_data>`__
-  `luw_req_buf_append <#luw_req_buf_append>`__
-  `luw_req_buf_copy <#luw_req_buf_copy>`__
-  `luw_mem_splice_file <#luw_mem_splice_file>`__
-  `luw_mem_fill_buf_from_req <#luw_mem_fill_buf_from_req>`__
-  `luw_mem_reset <#luw_mem_reset>`__
-  `luw_http_set_response_status <#luw_http_set_response_status>`__
-  `luw_http_send_response <#luw_http_send_response>`__
-  `luw_http_init_headers <#luw_http_init_headers>`__
-  `luw_http_add_header <#luw_http_add_header>`__
-  `luw_http_send_headers <#luw_http_send_headers>`__
-  `luw_http_response_end <#luw_http_response_end>`__
-  `luw_mem_get_init_size <#luw_mem_get_init_size>`__
-  `luw_foreach_http_hdr <#luw_foreach_http_hdr>`__

8. `Misc. Functions <#misc-functions>`__

-  `luw_malloc <#luw_malloc>`__
-  `luw_free <#luw_free>`__

Macros
------

Version
~~~~~~~

.. code:: c

   #define LUW_VERSION_MAJOR   M
   #define LUW_VERSION_MINOR   m
   #define LUW_VERSION_PATCH   p

.. code:: c

   /* Version number in hex 0xMMmmpp00 */
   #define LUW_VERSION_NUMBER \
           ( (LUW_VERSION_MAJOR << 24) | \
             (LUW_VERSION_MINOR << 16) | \
             (LUW_VERSION_PATCH << 8) )

Misc
~~~~

.. code:: c

   #define __luw_export_name(name) __attribute__((export_name(name)))

.. code:: c

   #define __luw_unused            __attribute__((unused))
   #define __luw_maybe_unused      __luw_unused

.. code:: c

   #define luw_foreach_http_hdr(ctx, iter, name, value) \
           for (iter = ctx.req->fields, \
                name = (const char *)ctx.req + iter->name_off; \
                (iter < (ctx.req->fields + ctx.req->nr_fields)) && \
                (value = (const char *)ctx.req + iter->value_off); \
                iter++, name = (const char *)ctx.req + iter->name_off)

Types
-----

.. code:: c

   typedef uint64_t u64;
   typedef int64_t  s64;
   typedef uint32_t u32;
   typedef int32_t  s32;
   typedef uint16_t u16;
   typedef int16_t  s16;
   typedef uint8_t   u8;
   typedef int8_t    s8;

Enums
-----

luw_srb_flags_t
~~~~~~~~~~~~~~~

.. code:: c

   typedef enum {
           LUW_SRB_NONE = 0x00,
           LUW_SRB_APPEND = 0x01,
           LUW_SRB_ALLOC = 0x02,
           LUW_SRB_FULL_SIZE = 0x04,

           LUW_SRB_FLAGS_ALL = (LUW_SRB_NONE|LUW_SRB_APPEND|LUW_SRB_ALLOC|
                                LUW_SRB_FULL_SIZE)
   } luw_srb_flags_t;

luw_http_status_t
~~~~~~~~~~~~~~~~~

.. code:: c

   typedef enum {
           LUW_HTTP_CONTINUE                               = 100,
           LUW_HTTP_SWITCHING_PROTOCOLS                    = 101,

           LUW_HTTP_OK                                     = 200,
           LUW_HTTP_CREATED                                = 201,
           LUW_HTTP_ACCEPTED                               = 202,
           LUW_HTTP_NO_CONTENT                             = 204,

           LUW_HTTP_MULTIPLE_CHOICES                       = 300,
           LUW_HTTP_MOVED_PERMANENTLY                      = 301,
           LUW_HTTP_FOUND                                  = 302,
           LUW_HTTP_SEE_OTHER                              = 303,
           LUW_HTTP_NOT_MODIFIED                           = 304,
           LUW_HTTP_TEMPORARY_REDIRECT                     = 307,
           LUW_HTTP_PERMANENT_REDIRECT                     = 308,

           LUW_HTTP_BAD_REQUEST                            = 400,
           LUW_HTTP_UNAUTHORIZED                           = 401,
           LUW_HTTP_FORBIDDEN                              = 403,
           LUW_HTTP_NOT_FOUND                              = 404,
           LUW_HTTP_METHOD_NOT_ALLOWED                     = 405,
           LUW_HTTP_NOT_ACCEPTABLE                         = 406,
           LUW_HTTP_REQUEST_TIMEOUT                        = 408,
           LUW_HTTP_CONFLICT                               = 409,
           LUW_HTTP_GONE                                   = 410,
           LUW_HTTP_LENGTH_REQUIRED                        = 411,
           LUW_HTTP_PAYLOAD_TOO_LARGE                      = 413,
           LUW_HTTP_URI_TOO_LONG                           = 414,
           LUW_HTTP_UNSUPPORTED_MEDIA_TYPE                 = 415,
           LUW_HTTP_UPGRADE_REQUIRED                       = 426,
           LUW_HTTP_TOO_MANY_REQUESTS                      = 429,
           LUW_HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE        = 431,

           /* Proposed by RFC 7725 */
           LUW_HTTP_UNAVAILABLE_FOR_LEGAL_REASONS          = 451,

           LUW_HTTP_INTERNAL_SERVER_ERROR                  = 500,
           LUW_HTTP_NOT_IMPLEMENTED                        = 501,
           LUW_HTTP_BAD_GATEWAY                            = 502,
           LUW_HTTP_SERVICE_UNAVAILABLE                    = 503,
           LUW_HTTP_GATEWAY_TIMEOUT                        = 504,
   } luw_http_status_t;

Structs
-------

.. code:: c

   struct luw_hdr_field {
           u32 name_off;
           u32 name_len;
           u32 value_off;
           u32 value_len;
   };

.. code:: c

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

           u64 content_len;
           u64 total_content_sent;
           u32 content_sent;
           u32 content_off;

           u32 request_size;

           u32 nr_fields;

           u32 tls;

           char __pad[4];

           struct luw_hdr_field fields[];
   };

.. code:: c

   struct luw_resp {
           u32 size;

           u8 data[];
   };

.. code:: c

   struct luw_resp_hdr {
           u32 nr_fields;

           struct luw_hdr_field fields[];
   };

.. code:: c

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

.. code:: c

   typedef struct luw_hdr_field luw_http_hdr_iter_t;

Function Handlers
-----------------

These functions are exported from the WebAssembly module and are called
from the WebAssembly runtime (the Unit WebAssembly language module in
this case).

There are two types of handlers; required & optional.

luw_request_handler(), luw_malloc_handler() & luw_free_handler() are
required with the rest being optional.

libunit-wasm includes exports for these handlers and some default
implementations.

These functions are defined as *weak* symbols and so if a developer
writes their own function of the same name, that will take precedence.

However, developers are under no obligation to use these and can create
their own with any (valid) names they like.

Whatever names developers choose, they are specified in the Unit config.

Required
~~~~~~~~

luw_request_handler
^^^^^^^^^^^^^^^^^^^

.. code:: c

   __attribute__((export_name("luw_request_handler"), __weak__))
   int luw_request_handler(u8 *addr);

This is called by Unit during a request. It may be called multiple times
for a single HTTP request if there is more request data than the
available memory for host <–> module communications.

You will need to provide your own implementation of this function.

It receives the base address of the shared memory. Essentially what is
returned by luw_malloc_handler().

This memory will contain a *struct luw_req*.

It returns an int. This should nearly always be *0*.

If you wish to indicate a ‘500 Internal Server Error’, for example if
some internal API has failed or an OS level error occurred, then you can
simply return *-1*, *if* you have haven’t already *sent* any response or
headers.

You can still return 0 *and* set the HTTP response status to 500 using
`luw_http_set_response_status <#luw_http_set_response_status>`__.

luw_malloc_handler
^^^^^^^^^^^^^^^^^^

.. code:: c

   __attribute__((export_name("luw_malloc_handler"), __weak__))
   u32 luw_malloc_handler(size_t size);

This is called by Unit when it loads the WebAssembly language module.
This provides the shared memory used for host <–> module communications.

It receives the desired size of the memory, which is currently
NXT_WASM_MEM_SIZE + NXT_WASM_PAGE_SIZE.

However calls to luw_mem_get_init_size() will return just
NXT_WASM_MEM_SIZE (which is currently 32MiB). The extra
NXT_WASM_PAGE_SIZE is to cater for structure sizes in the response so
developers can generally assume they have the full NXT_WASM_MEM_SIZE for
their data.

A default implementation of this function is provided ready for use that
calls malloc(3).

luw_free_handler
^^^^^^^^^^^^^^^^

.. code:: c

   __attribute__((export_name("luw_free_handler"), __weak__))
   void luw_free_handler(u32 addr);

This is called by Unit when it shuts down the WebAssembly language
module and free’s the memory previously allocated by
luw_malloc_handler().

It receives the address of the memory to free.

An implementation of this function is provided ready for use that calls
free(3), in which case it receives the address that was previously
returned by luw_malloc_handler().

Optional
~~~~~~~~

luw_module_init_handler
^^^^^^^^^^^^^^^^^^^^^^^

.. code:: c

   __attribute__((export_name("luw_module_init_handler"), __weak__))
   void luw_module_init_handler(void);

This is called by Unit when it loads the WebAssembly language module.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

luw_module_end_handler
^^^^^^^^^^^^^^^^^^^^^^

.. code:: c

   __attribute__((export_name("luw_module_end_handler"), __weak__))
   void luw_module_end_handler(void);

This is called by Unit when it shuts down the WebAssembly language
module.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

luw_request_init_handler
^^^^^^^^^^^^^^^^^^^^^^^^

.. code:: c

   __attribute__((export_name("luw_request_init_handler"), __weak__))
   void luw_request_init_handler(void);

This is called by Unit at the start of nxt_wasm_request_handler(), i.e
at the start of a new request.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

luw_request_end_handler
^^^^^^^^^^^^^^^^^^^^^^^

.. code:: c

   __attribute__((export_name("luw_request_end_handler"), __weak__))
   void luw_request_end_handler(void);

This is called by Unit at the end of nxt_wasm_request_handler(), i.e at
the end of a request.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

luw_response_end_handler
^^^^^^^^^^^^^^^^^^^^^^^^

.. code:: c

   __attribute__((export_name("luw_response_end_handler"), __weak__))
   void luw_response_end_handler(void);

This is called by Unit after luw_http_response_end() has been called.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

Functions
---------

luw_init_ctx
~~~~~~~~~~~~

.. code:: c

   void luw_init_ctx(luw_ctx_t *ctx, u8 *addr, size_t offset);

This function sets up a *luw_ctx_t* context structure, this contains
stuff required all throughout the API. It’s a typedef for opaqueness and
you should not in general be concerned with its contents.

It take a pointer to a stack allocated luw_ctx_t, this will be zeroed
and have various members initialised.

**addr** is a pointer to the shared memory as passed into
luw_request_handler().

**offset** is where in the shared memory it should start writing the
response.

A quick word about memory
^^^^^^^^^^^^^^^^^^^^^^^^^

The way the Unit WebAssembly language module (the host/runtime) and the
WebAssembly module you want to write (the guest) communicate is via a
chunk of shared memory.

This shared memory is simply the modules (guest) address space from
which we can allocate a chunk. How this memory is laid out varies on how
the module is built.

With clang/linker flags of -Wl,–stack-first
-Wl,-z,stack-size=$((8\ *1024*\ 1024)) we get a memory layout something
like

::

     |----------------------------------------------------------------------|
     |                     |             |                                  |
     |       <-- Stack     | Global Data |      Heap -->                    |
     |                     |             |                                  |
     |----------------------------------------------------------------------|
     0                     0x800000                               0x100000000

               WebAssembly Module Linear Memory / Process Memory Layout

(The above is assuming *–target=wasm32-wasi*, i.e 32bit)

A chunk of memory from the heap is allocated at Unit WebAssembly
language module startup.

We currently use this same chunk of memory for both requests and
responses. This means that depending on what you’re doing, you’ll want
to take a copy of the request (and remember luw_request_handler() may be
called multiple times for a single http request).

That will be covered in more detail by the next function,
luw_set_req_buf().

Now back to *offset*, it may be convenient to put the response headers
at the beginning of this memory and then put the response after it,
rather than doing the headers and then doing the response as separate
steps, if the headers depends on some aspect of the response, its size
for example and Content-Length.

Example

.. code:: c

   luw_ctx_t ctx;
   /* ... */
   luw_init_ctx(&ctx, addr, 4096 /* Response offset */);

luw_set_req_buf
~~~~~~~~~~~~~~~

.. code:: c

   int luw_set_req_buf(luw_ctx_t *ctx, u8 **buf, unsigned int flags);

This function is used to take a copy of the request buffer (as discussed
above).

This takes a previously initialised (with luw_init_ctx()) luw_ctx_t.

**buf** is a buffer where the request data will written.

**flags** can be some combination (OR’d) of the following

**LUW_SRB_NONE**

No specific action to be performed. It will simply copy the request data
into the specified buffer.

**LUW_SRB_APPEND**

Sets up append mode whereby multiple successive requests will be
appended to the specified buffer.

The first request will have all its metadata copied. Subsequent requests
will *only* have the actual body data appended.

**LUW_SRB_ALLOC**

Allocate memory for the specified buffer.

**LUW_SRB_FULL_SIZE**

Used in conjunction with *LUW_SRB_ALLOC*. By default only
*ctx->req->request_size* is allocated. If this flag is present it says
to allocate memory for the *entire* request that will eventually be
sent.

Example

.. code:: c

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

That example is taken from the
`luw-upload-reflector.c <https://github.com/nginx/unit-wasm/blob/main/examples/c/luw-upload-reflector.c>`__
demo module. For a simpler example see
`luw-echo-request.c <https://github.com/nginx/unit-wasm/blob/main/examples/c/luw-echo-request.c>`__

luw_get_http_path
~~~~~~~~~~~~~~~~~

.. code:: c

   const char *luw_get_http_path(const luw_ctx_t *ctx);

This function returns a pointer to the HTTP request path.

E.g

Given a request of

::

   http://localhost:8080/echo/?q=a

this function will return

::

   /echo/?q=a

luw_get_http_method
~~~~~~~~~~~~~~~~~~~

.. code:: c

   const char *luw_get_http_method(const luw_ctx_t *ctx);

This function returns a pointer to the HTTP method.

E.g

::

   GET

luw_get_http_version
~~~~~~~~~~~~~~~~~~~~

.. code:: c

   const char *luw_get_http_version(const luw_ctx_t *ctx);

This function returns a pointer to the HTTP version.

E.g

::

   1.1

luw_get_http_query
~~~~~~~~~~~~~~~~~~

.. code:: c

   const char *luw_get_http_query(const luw_ctx_t *ctx);

This function returns a pointer to the query string (empty string for no
query string).

E.g

Given a request of

::

   http://localhost:8080/echo/?q=a

this function will return

::

   q=a

luw_get_http_remote
~~~~~~~~~~~~~~~~~~~

.. code:: c

   const char *luw_get_http_remote(const luw_ctx_t *ctx);

This function returns a pointer to the remote/client/peer address.

E.g

::

   2001:db8::f00

luw_get_http_local_addr
~~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   const char *luw_get_http_local_addr(const luw_ctx_t *ctx);

This function returns a pointer to the local/server address.

E.g

::

   2001:db8::1

luw_get_http_local_port
~~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   const char *luw_get_http_local_port(const luw_ctx_t *ctx);

This function returns a pointer to the local/server port.

E.g

::

   443

luw_get_http_server_name
~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   const char *luw_get_http_server_name(const luw_ctx_t *ctx);

This function returns a pointer to the local/server name.

E.g

::

   www.example.com

luw_get_http_content
~~~~~~~~~~~~~~~~~~~~

.. code:: c

   const u8 *luw_get_http_content(const luw_ctx_t *ctx);

This function returns a pointer to the start of the request body.

luw_get_http_content_len
~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   u64 luw_get_http_content_len(const luw_ctx_t *ctx);

This function returns the size of the overall content. I.e
Content-Length.

Prior to version 0.3.0 it returned a size_t

luw_get_http_content_sent
~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   size_t luw_get_http_content_sent(const luw_ctx_t *ctx);

This function returns the length of the content that was sent to the
WebAssembly module in *this* request. Remember, a single HTTP request
may be split over several calls to luw_request_handler().

luw_get_http_total_content_sent
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   u64 luw_get_http_total_content_sent(const luw_ctx_t *ctx);

This function returns the total length of the content that was sent to
the WebAssembly module so far. Remember, a single HTTP request may be
split over several calls to luw_request_handler().

*Version: 0.2.0* Prior to 0.3.0 it returned a size_t

luw_http_is_tls
~~~~~~~~~~~~~~~

.. code:: c

   bool luw_http_is_tls(const luw_ctx_t *ctx);

This function returns *true* if the connection to Unit was made over
TLS.

luw_http_hdr_iter
~~~~~~~~~~~~~~~~~

.. code:: c

   void luw_http_hdr_iter(luw_ctx_t *ctx,
                          bool (*luw_http_hdr_iter_func)(luw_ctx_t *ctx,
                                                         const char *name,
                                                         const char *value,
                                                         void *data),
                          void *user_data)

This function allows to iterate over the HTTP headers. For each header
it will call the given luw_http_hdr_iter_func() function whose prototype
is

.. code:: c

   bool luw_http_hdr_iter_func(luw_ctx_t *ctx,
                               const char *name, const char *value, void *data);

You may call this function whatever you like. For each header it will be
passed the *luw_ctx_t*, the header name, its value and a user specified
pointer if any, can be NULL.

Returning *true* from this function will cause the iteration process to
continue, returning *false* will terminate it.

Example

.. code:: c

   static bool hdr_iter_func(luw_ctx_t *ctx, const char *name, const char *value,
                             void *user_data __luw_unused)
   {
           /* Do something with name & value */

           /* Continue iteration or return false to stop */
           return true;
   }

   /* ... *

   luw_http_hdr_iter(&ctx, hdr_iter_func, NULL);

luw_http_hdr_get_value
~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   const char *luw_http_hdr_get_value(const luw_ctx_t *ctx, const char *hdr);

Given a HTTP header *hdr* this function will look it up in the request
and return its value if found, otherwise *NULL*.

The lookup is done case insensitively.

luw_get_response_data_size
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   size_t luw_get_response_data_size(const luw_ctx_t *ctx);

This function returns the size of the response data written to memory.

luw_mem_writep
~~~~~~~~~~~~~~

.. code:: c

   __attribute__((__format__(printf, 2, 3)))
   int luw_mem_writep(luw_ctx_t *ctx, const char *fmt, ...);

This function is a cross between vasprintf(3) and mempcpy(3).

It takes a format argument and zero or more arguments that will be
substituted into the format string.

It then appends this formatted string to the memory. Note this string
will *not* be nul terminated. Unit does not expect this response data to
be nul terminated and we track the size of the response and return that
to Unit.

This function returns -1 on error or the length of the string written.

luw_mem_writep_data
~~~~~~~~~~~~~~~~~~~

.. code:: c

   size_t luw_mem_writep_data(luw_ctx_t *ctx, const u8 *src, size_t size);

This function just appends *size* bytes from *src* to the response.

It returns the new size of the response.

luw_req_buf_append
~~~~~~~~~~~~~~~~~~

.. code:: c

   void luw_req_buf_append(luw_ctx_t *ctx, const u8 *src);

This function appends the request data contained in *src* to the
previously setup *request_buffer* with luw_set_req_buf().

This function would be used after an initial request to append the data
from subsequent requests to the request_buffer.

Example

.. code:: c

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

luw_req_buf_copy
~~~~~~~~~~~~~~~~

.. code:: c

   void luw_req_buf_copy(luw_ctx_t *ctx, const u8 *src);

This function is analogous to
`luw_req_buf_append <#luw_req_buf_append>`__ but rather than appending
the request data contained in *src* to the previously setup
*request_buffer* with luw_set_req_buf(), it simply overwrites what’s
currently there.

This function could be used to handle large requests/uploads that you
want to save out to disk or some such and can’t buffer it all in memory.

Example

.. code:: c

   int luw_request_handler(u8 *addr)
   {
           const u8 *buf;
           ssize_t bytes_wrote;

           if (total_bytes_wrote == 0) {
                   luw_init_ctx(&ctx, addr, 0);
                   luw_set_req_buf(&ctx, &request_buf, LUW_SRB_NONE);

                   fd = open("/var/tmp/large-file.dat", O_CREAT|O_TRUNC|O_WRONLY,
                             0666);
           } else {
                   luw_req_buf_copy(&ctx, addr);
           }

           buf = luw_get_http_content(&ctx);
           bytes_wrote = write(fd, buf, luw_get_http_content_sent(&ctx));
           if (bytes_wrote == -1)
                   return -1;

           total_bytes_wrote += bytes_wrote;
           if (total_bytes_wrote == luw_get_http_content_len(&ctx))
                   luw_http_response_end();

           return 0;
   }

*Version: 0.3.0*

luw_mem_splice_file
~~~~~~~~~~~~~~~~~~~

.. code:: c

   ssize_t luw_mem_splice_file(const u8 *src, int fd);

This function write(2)’s the request data directly from the shared
memory (*src*) to the file represented by the given file-descriptor
(*fd*).

This can be used as an alternative to
`luw_req_buf_copy <#luw_req_buf_copy>`__ and avoids an extra copying of
the request data.

Example

.. code:: c

   int luw_request_handler(u8 *addr) {
           ssize_t bytes_wrote;

           if (total_bytes_wrote == 0) {
                   luw_init_ctx(&ctx, addr, 0);
                   luw_set_req_buf(&ctx, &request_buf, LUW_SRB_NONE);

                   fd = open("/var/tmp/large-file.dat", O_CREAT|O_TRUNC|O_WRONLY,
                             0666);
           }

           bytes_wrote = luw_mem_splice_file(addr, fd);
           if (bytes_wrote == -1)
                   return -1;

           total_bytes_wrote += bytes_wrote;
           if (total_bytes_wrote == luw_get_http_content_len(&ctx))
                   luw_http_response_end();

           return 0;
   }

*Version: 0.3.0*

luw_mem_fill_buf_from_req
~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   size_t luw_mem_fill_buf_from_req(luw_ctx_t *ctx, size_t from);

This is a convenience function to fill the response buffer with data
from the request buffer.

*from* is basically the offset in the request_buffer where to start
copying data from.

Example

.. code:: c

   /* ... */
   write_bytes = luw_mem_fill_buf_from_req(ctx, total_response_sent);
   total_response_sent += write_bytes;
   /* ... */

This is taken from the
`luw-upload-reflector.c <https://github.com/nginx/unit-wasm/blob/main/examples/c/luw-upload-reflector.c>`__
demo module.

In this case we build up a request_buffer on each call of
luw_request_handler(), so total_response_sent grows each time by how
much data was sent in *that* request.

Here are are sending data back to the client after each time we receive
it to demonstrate the interleaving of requests and responses from the
WebAssembly module during a single http request.

This function returns the number of bytes written to the response
buffer.

luw_mem_reset
~~~~~~~~~~~~~

.. code:: c

   void luw_mem_reset(luw_ctx_t *ctx);

This function resets the response buffer size and the number of response
headers back to 0.

luw_http_set_response_status
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   void luw_http_set_response_status(luw_http_status_t status);

This function is used to set the HTTP response status. It takes one of
the `luw_http_status_t <#luw_http_status_t>`__ enum values.

It should be called before any calls to *luw_http_send_response()* or
*luw_http_send_headers()*.

If you don’t call this function the response status defaults to ‘200
OK’.

If you wish to error out with a ‘500 Internal Server Error’, you don’t
need to call this function. Simply returning *-1* from the
request_handler function will indicate this error.

E.g

Send a ‘403 Forbidden’

.. code:: c

   /* ... */
   luw_http_set_response_status(LUW_HTTP_FORBIDDEN);
   luw_http_send_response(ctx);   /* Doesn't require any body */
   luw_http_response_end();
   /* ... */
   return 0;

Send a ‘307 Temporary Re-direct’

.. code:: c

   /* ... */
   luw_http_set_response_status(LUW_HTTP_TEMPORARY_REDIRECT);

   luw_http_init_headers(ctx, 1, 0);
   luw_http_add_header(ctx, "Location", "https://example.com/");
   luw_http_send_headers(ctx);
   luw_http_response_end();
   /* ... */
   return 0;

*Version: 0.3.0*

luw_http_send_response
~~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   void luw_http_send_response(const luw_ctx_t *ctx);

This function calls into Unit to send the response buffer back.

luw_http_init_headers
~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   void luw_http_init_headers(luw_ctx_t *ctx, size_t nr, size_t offset);

This function is used in the preparation of sending back response
headers.

*nr* is the number of headers we are sending.

*offset* is the offset into the response buffer where we are placing
these headers. This will usually be 0.

Example

.. code:: c

   luw_http_init_headers(ctx, 2, 0);

luw_http_add_header
~~~~~~~~~~~~~~~~~~~

.. code:: c

   void luw_http_add_header(luw_ctx_t *ctx, const char *name, const char *value);

This function is used to add a header to the response.

*name* is the name of the header.

*value* is the value of the header.

Example

.. code:: c

   char clen[32];
   /* ... */
   snprintf(clen, sizeof(clen), "%lu", luw_get_response_data_size(&ctx));
   luw_http_add_header(&ctx, "Content-Type", "text/plain");
   luw_http_add_header(&ctx, "Content-Length", clen);

luw_http_send_headers
~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   void luw_http_send_headers(const luw_ctx_t *ctx);

This function calls into Unit and triggers the sending of the response
headers.

luw_http_response_end
~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   void luw_http_response_end(void);

This function calls into Unit and tells it this is the end of the
response which will trigger Unit to send it to the client.

luw_mem_get_init_size
~~~~~~~~~~~~~~~~~~~~~

.. code:: c

   u32 luw_mem_get_init_size(void);

This function calls into Unit to get the size of the shared memory. This
is the amount of memory you should assume you have for creating
responses. Remember you can create multiple responses before calling
luw_http_response_end().

luw_foreach_http_hdr
~~~~~~~~~~~~~~~~~~~~

.. code:: c

   void luw_foreach_http_hdr(luw_ctx_t ctx, luw_http_hdr_iter_t *iter,
                             const char *name, const char *value)

Defined as a macro, this is used to iterate over the HTTP header fields.

It takes a \_luw_ctx_t \*\_ and a \_luw_http_hdr_iter_t \*\_ and returns
pointers to the field name and value.

Example

.. code:: c

   luw_ctx_t ctx;
   luw_http_hdr_iter_t *iter;
   const char *name;
   const char *value;
   /* ... */
   luw_foreach_http_hdr(ctx, iter, name, value) {
           printf("Field name : %s, field value : %s\n", name, value);
           /* do something else with name & value */
   }

Misc. Functions
---------------

The following functions are convenience wrappers for the Rust bindings
and should **not** be used directly.

luw_malloc
~~~~~~~~~~

.. code:: c

   void *luw_malloc(size_t size);

Straight wrapper for malloc(3).

luw_free
~~~~~~~~

.. code:: c

   void luw_free(void *ptr);

Straight wrapper for free(3).
