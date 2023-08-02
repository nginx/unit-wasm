/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) Timo Stark
 * Copyright (C) F5, Inc.
 */

// Include RAW FFI Bindings.
// @todo: Replace this with the new native Rust API
use unit_wasm::ffi::*;

use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;

static mut CTX: luw_ctx_t = luw_ctx_t {
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

static mut TOTAL_RESPONSE_SENT: usize = 0;

// Buffer of some size to store the copy of the request
static mut REQUEST_BUF: *mut u8 = ptr::null_mut();

#[no_mangle]
pub extern "C" fn luw_response_end_handler() {
    unsafe {
        TOTAL_RESPONSE_SENT = 0;
    }
}

#[no_mangle]
pub extern "C" fn luw_request_end_handler() {
    unsafe {
        if REQUEST_BUF.is_null() {
            return;
        }

        luw_free(REQUEST_BUF as *mut c_void);
        REQUEST_BUF = ptr::null_mut();
    }
}

pub fn upload_reflector(ctx: *mut luw_ctx_t) -> i32 {
    let write_bytes: usize;

    unsafe {
        // Send headers
        if TOTAL_RESPONSE_SENT == 0 {
            let content_len = format!("{}\0", luw_get_http_content_len(ctx));
            let defct = "application/octet-stream\0".as_ptr() as *const c_char;
            let mut ct = luw_http_hdr_get_value(
                ctx,
                "Content-Type\0".as_ptr() as *const c_char,
            );

            if ct == ptr::null_mut() {
                ct = defct;
            }

            luw_http_init_headers(ctx, 2, 0);
            luw_http_add_header(
                ctx,
                0,
                "Content-Type\0".as_ptr() as *const c_char,
                ct,
            );
            luw_http_add_header(
                ctx,
                1,
                "Content-Length\0".as_ptr() as *const c_char,
                content_len.as_ptr() as *const c_char,
            );
            luw_http_send_headers(ctx);
        }

        write_bytes = luw_mem_fill_buf_from_req(ctx, TOTAL_RESPONSE_SENT);
        TOTAL_RESPONSE_SENT += write_bytes;

        luw_http_send_response(ctx);

        if TOTAL_RESPONSE_SENT == luw_get_http_content_len(ctx) {
            // Tell Unit no more data to send
            luw_http_response_end();
        }
    }

    return 0;
}

#[no_mangle]
pub extern "C" fn luw_request_handler(addr: *mut u8) -> i32 {
    unsafe {
        let ctx: *mut luw_ctx_t = &mut CTX;

        if REQUEST_BUF.is_null() {
            luw_init_ctx(ctx, addr, 0 /* Response offset */);
            /*
             * Take a copy of the request and use that, we do this
             * in APPEND mode so we can build up request_buf from
             * multiple requests.
             *
             * Just allocate memory for the total amount of data we
             * expect to get, this includes the request structure
             * itself as well as any body content.
             */
            luw_set_req_buf(
                ctx,
                &mut REQUEST_BUF,
                luw_srb_flags_t_LUW_SRB_APPEND
                    | luw_srb_flags_t_LUW_SRB_ALLOC
                    | luw_srb_flags_t_LUW_SRB_FULL_SIZE,
            );
        } else {
            luw_req_buf_append(ctx, addr);
        }

        upload_reflector(ctx);
    }

    return 0;
}
