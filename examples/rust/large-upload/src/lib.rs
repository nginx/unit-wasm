/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) Timo Stark
 * Copyright (C) F5, Inc.
 */

use unit_wasm::rusty::*;

use std::fs::File;
use std::ptr::null_mut;

static mut CTX: luw_ctx_t = UWR_CTX_INITIALIZER();
static mut REQUEST_BUF: *mut u8 = null_mut();
static mut TOTAL_BYTES_WROTE: u64 = 0;

#[no_mangle]
pub unsafe extern "C" fn uwr_module_end_handler() {
    uwr_free(REQUEST_BUF);
}

#[no_mangle]
pub unsafe extern "C" fn uwr_module_init_handler() {
    REQUEST_BUF = uwr_malloc(uwr_mem_get_init_size());
}

#[no_mangle]
pub unsafe extern "C" fn uwr_response_end_handler() {
    TOTAL_BYTES_WROTE = 0;
}

#[no_mangle]
pub extern "C" fn uwr_request_handler(addr: *mut u8) -> i32 {
    let ctx: *mut luw_ctx_t = unsafe { &mut CTX };
    let mut f;
    let bytes_wrote: isize;
    let mut total = unsafe { TOTAL_BYTES_WROTE };

    if total == 0 {
        uwr_init_ctx(ctx, addr, 0);
        uwr_set_req_buf(ctx, unsafe { &mut REQUEST_BUF }, LUW_SRB_NONE);

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
        uwr_http_response_end();
    } else {
        unsafe { TOTAL_BYTES_WROTE = total };
    }

    return 0;
}
