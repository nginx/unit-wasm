/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) Timo Stark
 * Copyright (C) F5, Inc.
 */

use unit_wasm::rusty::*;

use std::ptr::null_mut;

static mut CTX: luw_ctx_t = UWR_CTX_INITIALIZER();

static mut TOTAL_RESPONSE_SENT: usize = 0;

// Buffer of some size to store the copy of the request
static mut REQUEST_BUF: *mut u8 = null_mut();

#[no_mangle]
pub extern "C" fn uwr_response_end_handler() {
    unsafe {
        TOTAL_RESPONSE_SENT = 0;
    }
}

#[no_mangle]
pub extern "C" fn uwr_request_end_handler() {
    unsafe {
        if REQUEST_BUF.is_null() {
            return;
        }

        uwr_free(REQUEST_BUF);
        REQUEST_BUF = null_mut();
    }
}

pub fn upload_reflector(ctx: *mut luw_ctx_t) -> i32 {
    let write_bytes: usize;

    // Send headers
    if unsafe { TOTAL_RESPONSE_SENT == 0 } {
        let defct = "application/octet-stream";
        let mut ct = uwr_http_hdr_get_value(ctx, "Content-Type");

        if ct.is_empty() {
            ct = defct;
        }

        uwr_http_init_headers(ctx, 2, 0);
        uwr_http_add_header_content_type(ctx, ct);
        uwr_http_add_header(
            ctx,
            "Content-Length",
            &format!("{}", uwr_get_http_content_len(ctx)),
        );
        uwr_http_send_headers(ctx);
    }

    unsafe {
        write_bytes = uwr_mem_fill_buf_from_req(ctx, TOTAL_RESPONSE_SENT);
        TOTAL_RESPONSE_SENT += write_bytes;
    }

    uwr_http_send_response(ctx);

    if unsafe { TOTAL_RESPONSE_SENT == uwr_get_http_content_len(ctx) } {
        // Tell Unit no more data to send
        uwr_http_response_end();
    }

    return 0;
}

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
