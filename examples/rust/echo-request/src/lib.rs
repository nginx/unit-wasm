/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) Timo Stark
 * Copyright (C) F5, Inc.
 */

// Include RAW FFI Bindings.
// @todo: Replace this with the new native Rust API
use unit_wasm::ffi::*;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;

// Buffer of some size to store the copy of the request
static mut REQUEST_BUF: *mut u8 = ptr::null_mut();

#[no_mangle]
pub extern "C" fn luw_module_end_handler() {
    unsafe {
        luw_free(REQUEST_BUF as *mut c_void);
    }
}

#[no_mangle]
pub extern "C" fn luw_module_init_handler() {
    unsafe {
        REQUEST_BUF = luw_malloc(luw_mem_get_init_size().try_into().unwrap())
            as *mut u8;
    }
}

pub extern "C" fn hdr_iter_func(
    ctx: *mut luw_ctx_t,
    name: *const c_char,
    value: *const c_char,
    _data: *mut c_void,
) -> bool {
    unsafe {
        luw_mem_writep(
            ctx,
            "%s = %s\n\0".as_ptr() as *const c_char,
            name,
            value,
        );
    }

    return true;
}

#[no_mangle]
pub extern "C" fn luw_request_handler(addr: *mut u8) -> i32 {
    // Need a initalization
    //
    // It sucks that rust needs this, this is supposed to be
    // an opaque structure and the structure is 0-initialised
    // in luw_init_ctx();
    let mut ctx_: luw_ctx_t = luw_ctx_t {
        addr: ptr::null_mut(),
        mem: ptr::null_mut(),
        req: ptr::null_mut(),
        resp: ptr::null_mut(),
        resp_hdr: ptr::null_mut(),
        resp_offset: 0,
        req_buf: ptr::null_mut(),
        hdrp: ptr::null_mut(),
        reqp: ptr::null_mut(),
    };
    let ctx: *mut luw_ctx_t = &mut ctx_;

    unsafe {
        // Initialise the context structure.
        //
        // addr is the address of the previously allocated memory shared
        // between the module and unit.
        //
        // The response data will be stored @ addr + offset (of 4096 bytes).
        // This will leave some space for the response headers.
        luw_init_ctx(ctx, addr, 4096);

        // Allocate memory to store the request and copy the request data.
        luw_set_req_buf(ctx, &mut REQUEST_BUF, luw_srb_flags_t_LUW_SRB_NONE);

        // Define the Response Body Text.

        luw_mem_writep(
            ctx,
            " * Welcome to WebAssembly in Rust on Unit! \
            [libunit-wasm (%d.%d.%d/%#0.8x)] \
            *\n\n\0"
                .as_ptr() as *const c_char,
            LUW_VERSION_MAJOR,
            LUW_VERSION_MINOR,
            LUW_VERSION_PATCH,
            LUW_VERSION_NUMBER,
        );

        luw_mem_writep(ctx, "[Request Info]\n\0".as_ptr() as *const c_char);

        luw_mem_writep(
            ctx,
            "REQUEST_PATH = %s\n\0".as_ptr() as *const c_char,
            luw_get_http_path(ctx) as *const c_char,
        );
        luw_mem_writep(
            ctx,
            "METHOD       = %s\n\0".as_ptr() as *const c_char,
            luw_get_http_method(ctx) as *const c_char,
        );
        luw_mem_writep(
            ctx,
            "VERSION      = %s\n\0".as_ptr() as *const c_char,
            luw_get_http_version(ctx) as *const c_char,
        );
        luw_mem_writep(
            ctx,
            "QUERY        = %s\n\0".as_ptr() as *const c_char,
            luw_get_http_query(ctx) as *const c_char,
        );
        luw_mem_writep(
            ctx,
            "REMOTE       = %s\n\0".as_ptr() as *const c_char,
            luw_get_http_remote(ctx) as *const c_char,
        );
        luw_mem_writep(
            ctx,
            "LOCAL_ADDR   = %s\n\0".as_ptr() as *const c_char,
            luw_get_http_local_addr(ctx) as *const c_char,
        );
        luw_mem_writep(
            ctx,
            "LOCAL_PORT   = %s\n\0".as_ptr() as *const c_char,
            luw_get_http_local_port(ctx) as *const c_char,
        );
        luw_mem_writep(
            ctx,
            "SERVER_NAME  = %s\n\0".as_ptr() as *const c_char,
            luw_get_http_server_name(ctx) as *const c_char,
        );

        luw_mem_writep(
            ctx,
            "\n[Request Headers]\n\0".as_ptr() as *const c_char,
        );

        luw_http_hdr_iter(ctx, Some(hdr_iter_func), ptr::null_mut());

        let method = CStr::from_ptr(luw_get_http_method(ctx)).to_str().unwrap();
        if method == "POST" || method == "PUT" {
            luw_mem_writep(
                ctx,
                "\n[%s data]\n\0".as_ptr() as *const c_char,
                luw_get_http_method(ctx) as *const c_char,
            );
            luw_mem_writep_data(
                ctx,
                luw_get_http_content(ctx),
                luw_get_http_content_len(ctx),
            );
            luw_mem_writep(ctx, "\n\0".as_ptr() as *const c_char);
        }

        let content_len = format!("{}\0", luw_get_response_data_size(ctx));

        // Init Response Headers
        //
        // Needs the context, number of headers about to add as well as
        // the offset where to store the headers. In this case we are
        // storing the response headers at the beginning of our shared
        // memory at offset 0.

        luw_http_init_headers(ctx, 2, 0);
        luw_http_add_header(
            ctx,
            0,
            "Content-Type\0".as_ptr() as *const c_char,
            "text/plain\0".as_ptr() as *const c_char,
        );
        luw_http_add_header(
            ctx,
            1,
            "Content-Length\0".as_ptr() as *const c_char,
            content_len.as_ptr() as *const c_char,
        );

        // This calls nxt_wasm_send_headers() in Unit
        luw_http_send_headers(ctx);

        // This calls nxt_wasm_send_response() in Unit
        luw_http_send_response(ctx);

        // This calls nxt_wasm_response_end() in Unit
        luw_http_response_end();
    }

    return 0;
}
