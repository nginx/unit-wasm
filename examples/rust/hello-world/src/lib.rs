/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) Timo Stark
 * Copyright (C) F5, Inc.
 */

use unit_wasm::rusty::*;

#[no_mangle]
pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
    let ctx = &mut UWR_CTX_INITIALIZER();
    let mut request_buf: *mut u8 = std::ptr::null_mut();

    uwr_init_ctx(ctx, addr, 4096);
    uwr_set_req_buf(ctx, &mut request_buf, LUW_SRB_ALLOC);

    uwr_write_str!(
        ctx,
        " * Welcome to WebAssembly in Rust on Unit! \
            [libunit-wasm ({}.{}.{}/{:#010x})] *\n\n",
        LUW_VERSION_MAJOR,
        LUW_VERSION_MINOR,
        LUW_VERSION_PATCH,
        LUW_VERSION_NUMBER,
    );

    uwr_http_init_headers(ctx, 2, 0);
    uwr_http_add_header_content_type(ctx, "text/plain");
    uwr_http_add_header_content_len(ctx);
    uwr_http_send_headers(ctx);

    uwr_http_send_response(ctx);
    uwr_http_response_end();

    uwr_free(request_buf);

    return 0;
}
