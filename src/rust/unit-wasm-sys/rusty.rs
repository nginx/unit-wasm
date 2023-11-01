/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) F5, Inc.
 */

use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::fs::File;
use std::os::fd::{AsRawFd, RawFd};
use std::ptr::null_mut;
use std::slice;
use std::str;

#[macro_export]
macro_rules! C2S {
    ($a:expr) => {{
        unsafe { CStr::from_ptr($a).to_str().unwrap() }
    }};
}

#[macro_export]
macro_rules! S2C {
    ($a:expr) => {{
        format!("{}\0", $a)
    }};
}

#[macro_export]
macro_rules! uwr_write_str{
    ($a:expr, $($arg:tt)*) => {
    {
        uwr_mem_write_str($a, &format!($($arg)*))
    }
}}

pub const fn UWR_CTX_INITIALIZER() -> luw_ctx_t {
    luw_ctx_t {
        addr: null_mut(),
        mem: null_mut(),
        req: null_mut(),
        resp: null_mut(),
        resp_hdr: null_mut(),
        resp_offset: 0,
        req_buf: null_mut(),
        hdrp: null_mut(),
        reqp: null_mut(),
        resp_hdr_idx: -1,
    }
}

pub unsafe fn uwr_init_ctx(ctx: *mut luw_ctx_t, addr: *mut u8, offset: usize) {
    luw_init_ctx(ctx, addr, offset);
}

pub unsafe fn uwr_set_req_buf(
    ctx: *mut luw_ctx_t,
    buf: *mut *mut u8,
    flags: u32,
) -> i32 {
    luw_set_req_buf(ctx, buf, flags)
}

pub unsafe fn uwr_get_http_path(ctx: *const luw_ctx_t) -> &'static str {
    C2S!(luw_get_http_path(ctx))
}

pub unsafe fn uwr_get_http_method(ctx: *const luw_ctx_t) -> &'static str {
    C2S!(luw_get_http_method(ctx))
}

pub unsafe fn uwr_get_http_version(ctx: *const luw_ctx_t) -> &'static str {
    C2S!(luw_get_http_version(ctx))
}

pub unsafe fn uwr_get_http_query(ctx: *const luw_ctx_t) -> &'static str {
    C2S!(luw_get_http_query(ctx))
}

pub unsafe fn uwr_get_http_remote(ctx: *const luw_ctx_t) -> &'static str {
    C2S!(luw_get_http_remote(ctx))
}

pub unsafe fn uwr_get_http_local_addr(ctx: *const luw_ctx_t) -> &'static str {
    C2S!(luw_get_http_local_addr(ctx))
}

pub unsafe fn uwr_get_http_local_port(ctx: *const luw_ctx_t) -> &'static str {
    C2S!(luw_get_http_local_port(ctx))
}

pub unsafe fn uwr_get_http_server_name(ctx: *const luw_ctx_t) -> &'static str {
    C2S!(luw_get_http_server_name(ctx))
}

pub unsafe fn uwr_get_http_content_len(ctx: *const luw_ctx_t) -> u64 {
    luw_get_http_content_len(ctx)
}

pub unsafe fn uwr_get_http_content_sent(ctx: *const luw_ctx_t) -> usize {
    luw_get_http_content_sent(ctx)
}

pub unsafe fn uwr_get_http_total_content_sent(ctx: *const luw_ctx_t) -> u64 {
    luw_get_http_total_content_sent(ctx)
}

pub unsafe fn uwr_get_http_content(ctx: *const luw_ctx_t) -> *const u8 {
    luw_get_http_content(ctx)
}

pub unsafe fn uwr_get_http_content_str(ctx: *const luw_ctx_t) -> &'static str {
    let slice = slice::from_raw_parts(
        uwr_get_http_content(ctx),
        uwr_get_http_total_content_sent(ctx).try_into().unwrap(),
    );
    str::from_utf8(slice).unwrap()
}

pub unsafe fn uwr_http_is_tls(ctx: *const luw_ctx_t) -> bool {
    luw_http_is_tls(ctx)
}

pub unsafe fn uwr_http_hdr_iter(
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
) {
    luw_http_hdr_iter(ctx, luw_http_hdr_iter_func, user_data)
}

pub unsafe fn uwr_http_hdr_get_value(
    ctx: *const luw_ctx_t,
    hdr: &str,
) -> &'static str {
    C2S!(luw_http_hdr_get_value(ctx, S2C!(hdr).as_ptr() as *const i8))
}

pub unsafe fn uwr_get_response_data_size(ctx: *const luw_ctx_t) -> usize {
    luw_get_response_data_size(ctx)
}

pub unsafe fn uwr_mem_write_str(ctx: *mut luw_ctx_t, src: &str) -> usize {
    luw_mem_writep_data(ctx, src.as_ptr(), src.len())
}

pub unsafe fn uwr_mem_write_buf(
    ctx: *mut luw_ctx_t,
    src: *const u8,
    size: u64,
) -> usize {
    /*
     * We're dealing with a 32bit address space, but we allow
     * size to come from the output of uwr_get_http_content_len()
     * which returns a u64 to allow for larger than memory uploads.
     */
    let sz = size as usize;

    luw_mem_writep_data(ctx, src, sz)
}

pub unsafe fn uwr_req_buf_append(ctx: *mut luw_ctx_t, src: *const u8) {
    luw_req_buf_append(ctx, src);
}

pub unsafe fn uwr_req_buf_copy(ctx: *mut luw_ctx_t, src: *const u8) {
    luw_req_buf_copy(ctx, src);
}

pub unsafe fn uwr_mem_splice_file(src: *const u8, f: &mut File) -> isize {
    let fd: RawFd = f.as_raw_fd();

    luw_mem_splice_file(src, fd)
}

pub unsafe fn uwr_mem_fill_buf_from_req(
    ctx: *mut luw_ctx_t,
    from: usize,
) -> usize {
    luw_mem_fill_buf_from_req(ctx, from)
}

pub unsafe fn uwr_luw_mem_reset(ctx: *mut luw_ctx_t) {
    luw_mem_reset(ctx);
}

pub unsafe fn uwr_http_set_response_status(status: luw_http_status_t) {
    luw_http_set_response_status(status);
}

pub unsafe fn uwr_http_send_response(ctx: *const luw_ctx_t) {
    luw_http_send_response(ctx);
}

pub unsafe fn uwr_http_init_headers(
    ctx: *mut luw_ctx_t,
    nr: usize,
    offset: usize,
) {
    luw_http_init_headers(ctx, nr, offset);
}

pub unsafe fn uwr_http_add_header(
    ctx: *mut luw_ctx_t,
    name: &str,
    value: &str,
) {
    luw_http_add_header(
        ctx,
        S2C!(name).as_ptr() as *const i8,
        S2C!(value).as_ptr() as *const i8,
    );
}

pub unsafe fn uwr_http_add_header_content_type(
    ctx: *mut luw_ctx_t,
    ctype: &str,
) {
    uwr_http_add_header(ctx, "Content-Type", ctype);
}

pub unsafe fn uwr_http_add_header_content_len(ctx: *mut luw_ctx_t) {
    uwr_http_add_header(
        ctx,
        "Content-Length",
        &format!("{}", uwr_get_response_data_size(ctx)),
    );
}

pub unsafe fn uwr_http_send_headers(ctx: *const luw_ctx_t) {
    luw_http_send_headers(ctx);
}

pub unsafe fn uwr_http_response_end() {
    luw_http_response_end();
}

pub unsafe fn uwr_mem_get_init_size() -> u32 {
    luw_mem_get_init_size()
}

pub unsafe fn uwr_malloc(size: u32) -> *mut u8 {
    luw_malloc(size as usize) as *mut u8
}

pub unsafe fn uwr_free(ptr: *mut u8) {
    luw_free(ptr as *mut c_void);
}
