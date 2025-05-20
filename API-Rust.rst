‘Rusty’ Rust API
================

Rusty is a more native Rust wrapper around the auto-generated bindings
to
`libunit-wasm <https://github.com/nginx/unit-wasm/blob/main/API-C.md>`__.

.. code:: rust

   use unit_wasm::rusty::*;

If using

.. code:: rust

   uwr_http_hdr_iter();

.. code:: rust

   use std::ffi::CStr;
   use std::os::raw::c_char;
   use std::os::raw::c_void;

Naming
------

You will see references to functions etc starting with *luw\_* or
*LUW\_* and \*uwr\_“.

**luw/LUW** (libunit-wasm) come from the underlying C library and in the
Rust case are the auto-generated bindings with a few manual additions.

**uwr** (Unit Wasm Rust aka ‘*rusty*’) is a more Rust native wrapper
ontop of the bindings.

In *rusty* the luw/LUW API is generally the low level stuff like the
library version macros and the various function handlers where they can
be used as is and there isn’t a real need to create wrappers
specifically for them.

1. `‘Rusty’ Rust API <#rusty-rust-api>`__

-  `Naming <#naming>`__

2. `Macros <#macros>`__

-  `Version <#version>`__
-  `String Conversion <#string-conversion>`__
-  `uwr_write_str! <#uwr_write_str>`__

3. `Enums <#enums>`__ `luw_http_status_t <#luw_http_status_t>`__
4. `Function Handlers <#function-handlers>`__

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

4. `Functions <#functions>`__

-  `UWR_CTX_INITIALIZER <#uwr_ctx_initializer>`__
-  `uwr_init_ctx <#uwr_init_ctx>`__
-  `uwr_set_req_buf <#uwr_set_req_buf>`__
-  `uwr_get_http_path <#uwr_get_http_path>`__
-  `uwr_get_http_method <#uwr_get_http_method>`__
-  `uwr_get_http_version <#uwr_get_http_version>`__
-  `uwr_get_http_query <#uwr_get_http_query>`__
-  `uwr_get_http_remote <#uwr_get_http_remote>`__
-  `uwr_get_http_local_addr <#uwr_get_http_local_addr>`__
-  `uwr_get_http_local_port <#uwr_get_http_local_port>`__
-  `uwr_get_http_server_name <#uwr_get_http_server_name>`__
-  `uwr_get_http_content <#uwr_get_http_content>`__
-  `uwr_get_http_content_str <#uwr_get_http_content_str>`__
-  `uwr_get_http_content_len <#uwr_get_http_content_len>`__
-  `uwr_get_http_content_sent <#uwr_get_http_content_sent>`__
-  `uwr_get_http_total_content_sent <#uwr_get_http_total_content_sent>`__
-  `uwr_http_is_tls <#uwr_http_is_tls>`__
-  `uwr_http_hdr_iter <#uwr_http_hdr_iter>`__
-  `uwr_http_hdr_get_value <#uwr_http_hdr_get_value>`__
-  `uwr_get_response_data_size <#uwr_get_response_data_size>`__
-  `uwr_mem_write_buf <#uwr_mem_write_buf>`__
-  `uwr_req_buf_append <#uwr_req_buf_append>`__
-  `uwr_req_buf_copy <#uwr_req_buf_copy>`__
-  `uwr_mem_splice_file <#uwr_mem_splice_file>`__
-  `uwr_mem_fill_buf_from_req <#uwr_mem_fill_buf_from_req>`__
-  `uwr_mem_reset <#uwr_mem_reset>`__
-  `uwr_http_set_response_status <#uwr_http_set_response_status>`__
-  `uwr_http_send_response <#uwr_http_send_response>`__
-  `uwr_http_init_headers <#uwr_http_init_headers>`__
-  `uwr_http_add_header <#uwr_http_add_header>`__
-  `uwr_http_add_header_content_type <#uwr_http_add_header_content_type>`__
-  `uwr_http_add_header_content_len <#uwr_http_add_header_content_len>`__
-  `uwr_http_send_headers <#uwr_http_send_headers>`__
-  `uwr_http_response_end <#uwr_http_response_end>`__
-  `uwr_mem_get_init_size <#uwr_mem_get_init_size>`__

5. `Misc. Functions <#misc-functions>`__

-  `uwr_malloc <#uwr_malloc>`__
-  `uwr_free <#uwr_free>`__

Macros
------

Version
~~~~~~~

For the underlying libunit-wasm version.

.. code:: rust

   pub const LUW_VERSION_MAJOR: i32;
   pub const LUW_VERSION_MINOR: i32;
   pub const LUW_VERSION_PATCH: i32;

.. code:: rust

   /* Version number in hex 0xMMmmpp00 */
   pub const LUW_VERSION_NUMBER: i32 =
       (LUW_VERSION_MAJOR << 24) | \
       (LUW_VERSION_MINOR << 16) | \
       (LUW_VERSION_PATCH << 8);

String Conversion
~~~~~~~~~~~~~~~~~

.. code:: rust

   C2S!(string);

Converts a C string into a Rust String

Main use is internally and in the *uwr_http_hdr_iter()* callback
function, e.g

.. code:: rust

   pub extern "C" fn hdr_iter_func(
       ctx: *mut luw_ctx_t,
       name: *const c_char,
       value: *const c_char,
       _data: *mut c_void,
   ) -> bool {
       uwr_write_str!(ctx, "{} = {}\n", C2S!(name), C2S!(value));

       return true;
   }

Example taken from the
`echo-request <https://github.com/nginx/unit-wasm/blob/main/examples/rust/echo-request/src/lib.rs>`__
Wasm demo module

.. code:: rust

   S2C!(formatted string);

Converts a Rust String, with optional formatting, to a C string.

Used internally.

uwr_write_str!
~~~~~~~~~~~~~~

.. code:: rust

   uwr_write_str!*ctx, fmt, ...);

This is essentially a wrapper around
`luw_mem_writep_data() <https://github.com/nginx/unit-wasm/blob/main/API-C.md#luw_mem_writep_data>`__

It is the main way to write responses back to the client.

It takes the luw_ctx_t context pointer, a string that will be run
through the
`format!() <https://doc.rust-lang.org/std/macro.format.html>`__ macro
and any optional arguments.

Enums
-----

luw_http_status_t
~~~~~~~~~~~~~~~~~

.. code:: rust

   pub enum luw_http_status_t {
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
   }

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

.. code:: rust

   #[no_mangle]
   pub extern "C" fn luw_request_handler(addr: *mut u8) -> i32;

This is called by Unit during a request. It may be called multiple times
for a single HTTP request if there is more request data than the
available memory for host <–> module communications.

You will need to provide your own implementation of this function.

It receives the base address of the shared memory. Essentially what is
returned by luw_malloc_handler().

It returns an int. This should nearly always be *0*.

If you wish to indicate a ‘500 Internal Server Error’, for example if
some internal API has failed or an OS level error occurred, then you can
simply return *-1*, *if* you have haven’t already *sent* any response or
headers.

You can still return 0 *and* set the HTTP response status to 500 using
`uwr_http_set_response_status <#uwr_http_set_response_status>`__.

luw_malloc_handler
^^^^^^^^^^^^^^^^^^

.. code:: rust

   #[no_mangle]
   pub extern "C" fn luw_malloc_handler(size: usize) -> u32;

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

.. code:: rust

   #[no_mangle]
   pub extern "C" fn luw_free_handler(addr: u32);

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

.. code:: rust

   #[no_mangle]
   pub extern "C" fn luw_module_init_handler();

This is called by Unit when it loads the WebAssembly language module.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

luw_module_end_handler
^^^^^^^^^^^^^^^^^^^^^^

.. code:: rust

   #[no_mangle]
   pub extern "C" fn luw_module_end_handler();

This is called by Unit when it shuts down the WebAssembly language
module.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

luw_request_init_handler
^^^^^^^^^^^^^^^^^^^^^^^^

.. code:: rust

   #[no_mangle]
   pub extern "C" fn luw_request_init_handler();

This is called by Unit at the start of nxt_wasm_request_handler(), i.e
at the start of a new request.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

luw_request_end_handler
^^^^^^^^^^^^^^^^^^^^^^^

.. code:: rust

   #[no_mangle]
   pub extern "C" fn luw_request_end_handler();

This is called by Unit at the end of nxt_wasm_request_handler(), i.e at
the end of a request.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

luw_response_end_handler
^^^^^^^^^^^^^^^^^^^^^^^^

.. code:: rust

   #[no_mangle]
   pub extern "C" fn luw_response_end_handler();

This is called by Unit after luw_http_response_end() has been called.

A default dummy function is provided. If this handler is not required,
there is no need to specify it in the Unit config.

Functions
---------

UWR_CTX_INITIALIZER
~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub const fn UWR_CTX_INITIALIZER() -> luw_ctx_t;

Used to initialise a luw_ctx_t context structure. E.g

.. code:: rust

   let ctx = &mut UWR_CTX_INITIALIZER();

uwr_init_ctx
~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_init_ctx(ctx: *mut luw_ctx_t, addr: *mut u8, offset: usize);

This function sets up a *luw_ctx_t* context structure, this contains
stuff required all throughout the API.

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
uwr_set_req_buf().

Now back to *offset*, it may be convenient to put the response headers
at the beginning of this memory and then put the response after it,
rather than doing the headers and then doing the response as separate
steps, if the headers depends on some aspect of the response, its size
for example and Content-Length.

Example

.. code:: rust

   #[no_mangle]
   pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
       let ctx = &mut UWR_CTX_INITIALIZER();
       /* ... */
       uwr_init_ctx(ctx, addr, 4096 /* Response offset */);

uwr_set_req_buf
~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_set_req_buf(
       ctx: *mut luw_ctx_t,
       buf: *mut *mut u8,
       flags: u32,
   ) -> i32;

This function is used to take a copy of the request buffer (as discussed
above).

This takes a previously initialised (with uwr_init_ctx()) luw_ctx_t.

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

.. code:: rust

   static mut CTX: luw_ctx_t = UWR_CTX_INITIALIZER();

   static mut REQUEST_BUF: *mut u8 = null_mut();
   */ ... */
   #[no_mangle]
   pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
       let ctx: *mut luw_ctx_t = addr_of_mut!(CTX);

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
               addr_of_mut!(REQUEST_BUF),
               LUW_SRB_APPEND | LUW_SRB_ALLOC | LUW_SRB_FULL_SIZE,
           );
       } else {
           uwr_req_buf_append(ctx, addr);
       }

       upload_reflector(ctx);

       return 0;
   }

That example is taken from the `upload-reflector
demo <https://github.com/nginx/unit-wasm/blob/main/examples/rust/upload-reflector/src/lib.rs>`__
demo module. For a simpler example see the `echo-request
demo <https://github.com/nginx/unit-wasm/blob/main/examples/rust/echo-request/src/lib.rs>`__

uwr_get_http_path
~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_path(ctx: *const luw_ctx_t) -> &'static str;

This function returns a pointer to the HTTP request path.

E.g

Given a request of

::

   http://localhost:8080/echo/?q=a

this function will return

::

   /echo/?q=a

uwr_get_http_method
~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_method(ctx: *const luw_ctx_t) -> &'static str;

This function returns a pointer to the HTTP method.

E.g

::

   GET

uwr_get_http_version
~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_version(ctx: *const luw_ctx_t) -> &'static str;

This function returns a pointer to the HTTP version.

E.g

::

   1.1

uwr_get_http_query
~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_query(ctx: *const luw_ctx_t) -> &'static str;

This function returns a pointer to the query string (empty string for no
query string).

E.g

Given a request of

::

   http://localhost:8080/echo/?q=a

this function will return

::

   q=a

uwr_get_http_remote
~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_remote(ctx: *const luw_ctx_t) -> &'static str;

This function returns a pointer to the remote/client/peer address.

E.g

::

   2001:db8::f00

uwr_get_http_local_addr
~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_local_addr(ctx: *const luw_ctx_t) -> &'static str;

This function returns a pointer to the local/server address.

E.g

::

   2001:db8::1

uwr_get_http_local_port
~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_local_port(ctx: *const luw_ctx_t) -> &'static str;

This function returns a pointer to the local/server port.

E.g

::

   443

uwr_get_http_server_name
~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_server_name(ctx: *const luw_ctx_t) -> &'static str;

This function returns a pointer to the local/server name.

E.g

::

   www.example.com

uwr_get_http_content
~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_content(ctx: *const luw_ctx_t) -> *const u8;

This function returns a pointer to the start of the request body.

uwr_get_http_content_str
~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rsut

   pub fn uwr_get_http_content_str(ctx: *const luw_ctx_t) -> &'static str;

Same as above but returns a Rust str.

*Version: 0.2.0*

uwr_get_http_content_len
~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_content_len(ctx: *const luw_ctx_t) -> u64;

This function returns the size of the overall content. I.e
Content-Length.

Prior to version 0.3.0 it returned a usize

uwr_get_http_content_sent
~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_content_sent(ctx: *const luw_ctx_t) -> usize;

This function returns the length of the content that was sent to the
WebAssembly module in *this* request. Remember, a single HTTP request
may be split over several calls to luw_request_handler().

uwr_get_http_total_content_sent
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_http_total_content_sent(ctx: *const luw_ctx_t) -> u64;

This function returns the total length of the content that was sent to
the WebAssembly module so far. Remember, a single HTTP request may be
split over several calls to luw_request_handler().

*Version: 0.2.0* Prior to 0.3.0 it returned a usize

uwr_http_is_tls
~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_is_tls(ctx: *const luw_ctx_t) -> bool;

This function returns *true* if the connection to Unit was made over
TLS.

uwr_http_hdr_iter
~~~~~~~~~~~~~~~~~

.. code:: rust

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

This function allows to iterate over the HTTP headers. For each header
it will call the given luw_http_hdr_iter_func() function whose prototype
is

.. code:: rust

   pub extern "C" fn hdr_iter_func(
       ctx: *mut luw_ctx_t,
       name: *const c_char,
       value: *const c_char,
       data: *mut c_void,
   ) -> bool;

You may call this function whatever you like. For each header it will be
passed the *luw_ctx_t*, the header name, its value and a user specified
pointer if any, can be NULL.

Returning *true* from this function will cause the iteration process to
continue, returning *false* will terminate it.

Example

.. code:: rust

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

uwr_http_hdr_get_value
~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_hdr_get_value(ctx: *const luw_ctx_t, hdr: &str) -> &'static str;

Given a HTTP header *hdr* this function will look it up in the request
and return its value if found, otherwise *NULL*.

The lookup is done case insensitively.

uwr_get_response_data_size
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_get_response_data_size(ctx: *const luw_ctx_t) -> usize;

This function returns the size of the response data written to memory.

uwr_mem_write_buf
~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_mem_write_buf(
       ctx: *mut luw_ctx_t,
       src: *const u8,
       size: usize,
   ) -> usize;

This function just appends *size* bytes from *src* to the response.

It returns the new size of the response.

uwr_req_buf_append
~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_req_buf_append(ctx: *mut luw_ctx_t, src: *const u8);

This function appends the request data contained in *src* to the
previously setup *request_buffer* with uwr_set_req_buf().

This function would be used after an initial request to append the data
from subsequent requests to the request_buffer.

Example

.. code:: rust

   #[no_mangle]
   pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
       let ctx: *mut luw_ctx_t = addr_of_mut!(CTX);

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
               addr_of_mut!(REQUEST_BUF),
               LUW_SRB_APPEND | LUW_SRB_ALLOC | LUW_SRB_FULL_SIZE,
           );
       } else {
           uwr_req_buf_append(ctx, addr);
       }

       upload_reflector(ctx);

       return 0;
   }

uwr_req_buf_copy
~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_req_buf_copy(ctx: *mut luw_ctx_t, src: *const u8);

This function is analogous to
`uwr_req_buf_append <#uwr_req_buf_append>`__ but rather than appending
the request data contained in *src* to the previously setup
*request_buffer* with uwr_set_req_buf(), it simply overwrites what’s
currently there.

This function could be used to handle large requests/uploads that you
want to save out to disk or some such and can’t buffer it all in memory.

uwr_mem_splice_file
~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_mem_splice_file(src: *const u8, f: &mut File) -> isize;

This function write(2)’s the request data directly from the shared
memory (*src*) to the file represented by the given *File* object (*f*).

This can be used as an alternative to
`uwr_req_buf_copy <#uwr_req_buf_copy>`__ and avoids an extra copying of
the request data.

Example

.. code:: rust

   pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
       let ctx: *mut luw_ctx_t = addr_of_mut!(CTX);
       let mut f;
       let bytes_wrote: isize;
       let mut total = unsafe { TOTAL_BYTES_WROTE };

       if total == 0 {
           uwr_init_ctx(ctx, addr, 0);
           uwr_set_req_buf(
               ctx,
               addr_of_mut!(REQUEST_BUF),
               LUW_SRB_NONE
           );

           f = File::create("/var/tmp/large-file.dat").unwrap();
       } else {
           f = File::options()
               .append(true)
               .open("/var/tmp/large-file.dat")
               .unwrap();
       }

       bytes_wrote = uwr_mem_splice_file(addr, &mut f);
       if bytes_wrote == -1 {
           return -1;
       }

       total += bytes_wrote as u64;
       if total == uwr_get_http_content_len(ctx) {
           total = 0;

           uwr_http_response_end();
       }

       unsafe { TOTAL_BYTES_WROTE = total };

       return 0;
   }

uwr_mem_fill_buf_from_req
~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_req_buf_append(ctx: *mut luw_ctx_t, src: *const u8);

This is a convenience function to fill the response buffer with data
from the request buffer.

*from* is basically the offset in the request_buffer where to start
copying data from.

Example

.. code:: rust

   /* ... */
   write_bytes = uwr_mem_fill_buf_from_req(ctx, TOTAL_RESPONSE_SENT);
   TOTAL_RESPONSE_SENT += write_bytes;
   /* ... */

This is taken from the `upload-reflector
demo <https://github.com/nginx/unit-wasm/blob/main/examples/c/upload-reflector/src/lib.rs>`__
demo module.

In this case we build up a request_buffer on each call of
luw_request_handler(), so TOTAL_RESPONSE_SENT grows each time by how
much data was sent in *that* request.

Here are are sending data back to the client after each time we receive
it to demonstrate the interleaving of requests and responses from the
WebAssembly module during a single http request.

This function returns the number of bytes written to the response
buffer.

uwr_mem_reset
~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_luw_mem_reset(ctx: *mut luw_ctx_t);

This function resets the response buffer size and the number of response
headers back to 0.

uwr_http_set_response_status
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_set_response_status(status: luw_http_status_t);

This function is used to set the HTTP response status. It takes one of
the `luw_http_status_t <#luw_http_status_t>`__ enum values.

It should be called before any calls to *uwr_http_send_response()* or
*uwr_http_send_headers()*.

If you don’t call this function the response status defaults to ‘200
OK’.

If you wish to error out with a ‘500 Internal Server Error’, you don’t
need to call this function. Simply returning *-1* from the
request_handler function will indicate this error.

E.g

Send a ‘403 Forbidden’

.. code:: rust

   /* ... */
   uwr_http_set_response_status(LUW_HTTP_FORBIDDEN);
   uwr_http_send_response(ctx);   /* Doesn't require any body */
   uwr_http_response_end();
   /* ... */
   return 0;

Send a ‘307 Temporary Re-direct’

.. code:: rust

   /* ... */
   uwr_http_set_response_status(LUW_HTTP_TEMPORARY_REDIRECT);

   uwr_http_init_headers(ctx, 1, 0);
   uwr_http_add_header(ctx, "Location", "https://example.com/");
   uwr_http_send_headers(ctx);
   uwr_http_response_end();
   /* ... */
   return 0;

*Version: 0.3.0*

uwr_http_send_response
~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_send_response(ctx: *const luw_ctx_t);

This function calls into Unit to send the response buffer back.

uwr_http_init_headers
~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_init_headers(ctx: *mut luw_ctx_t, nr: usize, offset: usize);

This function is used in the preparation of sending back response
headers.

*nr* is the number of headers we are sending.

*offset* is the offset into the response buffer where we are placing
these headers. This will usually be 0.

Example

.. code:: rust

   uwr_http_init_headers(ctx, 2, 0);

uwr_http_add_header
~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_add_header(
       ctx: *mut luw_ctx_t,
       name: &str,
       value: &str,
   );

This function is used to add a header to the response.

*name* is the name of the header.

*value* is the value of the header.

Example

.. code:: rust

   uwr_http_add_header(&ctx, "Content-Type", "text/plain");
   uwr_http_add_header(
       ctx,
       "Content-Length",
       &format!("{}", uwr_get_response_data_size(ctx)),
   );

uwr_http_add_header_content_type
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_add_header_content_type(ctx: *mut luw_ctx_t, ctype: &str);

A convenience function for setting the ‘Content-Type’ response header.
E.g the above example that adds the *Content-Type* header could be
written as

.. code:: rust

   uwr_http_add_header_content_type(ctx, "text/plain");

*Version: 0.2.0*

uwr_http_add_header_content_len
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_add_header_content_len(ctx: *mut luw_ctx_t);

A convenience function for setting the ‘Content-Length’ response header.
E.g the above example that adds the *Content-Length* header could be
written as

.. code:: rust

   uwr_http_add_header_content_len(ctx);

This function uses
`uwr_get_response_data_size <#uwr_get_response_data_size>`__ internally
to get the size of the response data.

*Version: 0.2.0*

uwr_http_send_headers
~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_send_headers(ctx: *const luw_ctx_t);

This function calls into Unit and triggers the sending of the response
headers.

uwr_http_response_end
~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_http_response_end();

This function calls into Unit and tells it this is the end of the
response which will trigger Unit to send it to the client.

uwr_mem_get_init_size
~~~~~~~~~~~~~~~~~~~~~

.. code:: rust

   pub fn uwr_mem_get_init_size() -> u32;

This function calls into Unit to get the size of the shared memory. This
is the amount of memory you should assume you have for creating
responses. Remember you can create multiple responses before calling
luw_http_response_end().

Misc. Functions
---------------

The following functions are convenience wrappers for the Rust bindings
and should **not** be used directly.

uwr_malloc
~~~~~~~~~~

.. code:: rust

   pub fn uwr_malloc(size: u32) -> *mut u8;

Essentially a straight wrapper for malloc(3).

uwr_free
~~~~~~~~

.. code:: rust

   pub fn uwr_free(ptr: *mut u8);

Essentially a straight wrapper for free(3).
