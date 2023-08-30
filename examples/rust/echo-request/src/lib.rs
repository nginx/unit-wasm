/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) Timo Stark
 * Copyright (C) F5, Inc.
 */

use unit_wasm::rusty::*;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr::null_mut;

// Buffer of some size to store the copy of the request
static mut REQUEST_BUF: *mut u8 = null_mut();

#[no_mangle]
pub extern "C" fn uwr_module_end_handler() {
    unsafe {
        uwr_free(REQUEST_BUF);
    }
}

#[no_mangle]
pub extern "C" fn uwr_module_init_handler() {
    unsafe {
        REQUEST_BUF = uwr_malloc(uwr_mem_get_init_size());
    }
}

pub extern "C" fn hdr_iter_func(
    ctx: *mut luw_ctx_t,
    name: *const c_char,
    value: *const c_char,
    _data: *mut c_void,
) -> bool {
    uwr_write_str!(ctx, "{} = {}\n", C2S!(name), C2S!(value));

    return true;
}

#[no_mangle]
pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
    // Declare a 0-initialised context structure
    let ctx = &mut UWR_CTX_INITIALIZER();
    // Initialise the context structure.
    //
    // addr is the address of the previously allocated memory shared
    // between the module and unit.
    //
    // The response data will be stored @ addr + offset (of 4096 bytes).
    // This will leave some space for the response headers.
    uwr_init_ctx(ctx, addr, 4096);

    // Set where we will copy the request into
    uwr_set_req_buf(ctx, unsafe { &mut REQUEST_BUF }, LUW_SRB_NONE);

    // Define the Response Body Text.

    uwr_write_str!(
        ctx,
        " * Welcome to WebAssembly in Rust on Unit! \
            [libunit-wasm ({}.{}.{}/{:#010x})] *\n\n",
        LUW_VERSION_MAJOR,
        LUW_VERSION_MINOR,
        LUW_VERSION_PATCH,
        LUW_VERSION_NUMBER,
    );

    uwr_write_str!(ctx, "[Request Info]\n");

    uwr_write_str!(ctx, "REQUEST_PATH = {}\n", uwr_get_http_path(ctx));
    uwr_write_str!(ctx, "METHOD       = {}\n", uwr_get_http_method(ctx));
    uwr_write_str!(ctx, "VERSION      = {}\n", uwr_get_http_version(ctx));
    uwr_write_str!(ctx, "QUERY        = {}\n", uwr_get_http_query(ctx));
    uwr_write_str!(ctx, "REMOTE       = {}\n", uwr_get_http_remote(ctx));
    uwr_write_str!(ctx, "LOCAL_ADDR   = {}\n", uwr_get_http_local_addr(ctx));
    uwr_write_str!(ctx, "LOCAL_PORT   = {}\n", uwr_get_http_local_port(ctx));
    uwr_write_str!(ctx, "SERVER_NAME  = {}\n", uwr_get_http_server_name(ctx));

    uwr_write_str!(ctx, "\n[Request Headers]\n");

    uwr_http_hdr_iter(ctx, Some(hdr_iter_func), null_mut());

    let method = uwr_get_http_method(ctx);
    if method == "POST" || method == "PUT" {
        uwr_write_str!(ctx, "\n[{} data]\n", method);
        uwr_mem_write_buf(
            ctx,
            uwr_get_http_content(ctx),
            uwr_get_http_content_len(ctx),
        );
        uwr_write_str!(ctx, "\n");
    }

    // Init Response Headers
    //
    // Needs the context, number of headers about to add as well as
    // the offset where to store the headers. In this case we are
    // storing the response headers at the beginning of our shared
    // memory at offset 0.
    uwr_http_init_headers(ctx, 2, 0);
    uwr_http_add_header_content_type(ctx, "text/plain");
    uwr_http_add_header_content_len(ctx);

    // This calls nxt_wasm_send_headers() in Unit
    uwr_http_send_headers(ctx);

    // This calls nxt_wasm_send_response() in Unit
    uwr_http_send_response(ctx);

    // This calls nxt_wasm_response_end() in Unit
    uwr_http_response_end();

    return 0;
}
