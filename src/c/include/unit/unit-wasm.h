/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) F5, Inc.
 */

#ifndef _UNIT_WASM_H_
#define _UNIT_WASM_H_

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

#define LUW_VERSION_MAJOR	0
#define LUW_VERSION_MINOR	2
#define LUW_VERSION_PATCH	0

/* Version number in hex 0xMMmmpp00 */
#define LUW_VERSION_NUMBER \
	( (LUW_VERSION_MAJOR << 24) | \
	  (LUW_VERSION_MINOR << 16) | \
	  (LUW_VERSION_PATCH << 8) )

#define __luw_export_name(name)	__attribute__((export_name(name)))

#define __luw_unused		__attribute__((unused))
#define __luw_maybe_unused	__luw_unused

typedef uint64_t u64;
typedef int64_t  s64;
typedef uint32_t u32;
typedef int32_t  s32;
typedef uint16_t u16;
typedef int16_t  s16;
typedef uint8_t   u8;
typedef int8_t    s8;

typedef enum {
	LUW_HTTP_CONTINUE				= 100,
	LUW_HTTP_SWITCHING_PROTOCOLS			= 101,

	LUW_HTTP_OK					= 200,
	LUW_HTTP_CREATED				= 201,
	LUW_HTTP_ACCEPTED				= 202,
	LUW_HTTP_NO_CONTENT				= 204,

	LUW_HTTP_MULTIPLE_CHOICES			= 300,
	LUW_HTTP_MOVED_PERMANENTLY			= 301,
	LUW_HTTP_FOUND					= 302,
	LUW_HTTP_SEE_OTHER				= 303,
	LUW_HTTP_NOT_MODIFIED				= 304,
	LUW_HTTP_TEMPORARY_REDIRECT			= 307,
	LUW_HTTP_PERMANENT_REDIRECT			= 308,

	LUW_HTTP_BAD_REQUEST				= 400,
	LUW_HTTP_UNAUTHORIZED				= 401,
	LUW_HTTP_FORBIDDEN				= 403,
	LUW_HTTP_NOT_FOUND				= 404,
	LUW_HTTP_METHOD_NOT_ALLOWED			= 405,
	LUW_HTTP_NOT_ACCEPTABLE				= 406,
	LUW_HTTP_REQUEST_TIMEOUT			= 408,
	LUW_HTTP_CONFLICT				= 409,
	LUW_HTTP_GONE					= 410,
	LUW_HTTP_LENGTH_REQUIRED			= 411,
	LUW_HTTP_PAYLOAD_TOO_LARGE			= 413,
	LUW_HTTP_URI_TOO_LONG				= 414,
	LUW_HTTP_UNSUPPORTED_MEDIA_TYPE			= 415,
	LUW_HTTP_UPGRADE_REQUIRED			= 426,
	LUW_HTTP_TOO_MANY_REQUESTS			= 429,
	LUW_HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE	= 431,

	/* Proposed by RFC 7725 */
	LUW_HTTP_UNAVAILABLE_FOR_LEGAL_REASONS		= 451,

	LUW_HTTP_INTERNAL_SERVER_ERROR			= 500,
	LUW_HTTP_NOT_IMPLEMENTED			= 501,
	LUW_HTTP_BAD_GATEWAY				= 502,
	LUW_HTTP_SERVICE_UNAVAILABLE			= 503,
	LUW_HTTP_GATEWAY_TIMEOUT			= 504,
} luw_http_status_t;

#if !defined(__DEFINED_ssize_t)
/*
 * Match the typedef from wasm32-wasi/include/bits/alltypes.h
 * without requiring the wasi-sysroot for building the rust
 * stuff.
 */
typedef long	  ssize_t;
#endif

struct luw_hdr_field {
	u32 name_off;
	u32 name_len;
	u32 value_off;
	u32 value_len;
};

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

struct luw_resp {
	u32 size;

	u8 data[];
};

struct luw_resp_hdr {
	u32 nr_fields;

	struct luw_hdr_field fields[];
};

typedef struct {
	/* pointer to the shared memory */
	u8 *addr;

	/* points to the end of ctx->resp->data */
	u8 *mem;

	/* struct luw_req representation of the shared memory */
	struct luw_req *req;

	/* struct luw_resp representation of the shared memory */
	struct luw_resp *resp;

	/* struct luw_resp_hdr represnetation of the shared memory */
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

typedef enum {
	LUW_SRB_NONE = 0x00,
	LUW_SRB_APPEND = 0x01,
	LUW_SRB_ALLOC = 0x02,
	LUW_SRB_FULL_SIZE = 0x04,

	LUW_SRB_FLAGS_ALL = (LUW_SRB_NONE|LUW_SRB_APPEND|LUW_SRB_ALLOC|
			     LUW_SRB_FULL_SIZE)
} luw_srb_flags_t;

typedef struct luw_hdr_field luw_http_hdr_iter_t;

#define luw_foreach_http_hdr(ctx, iter, name, value) \
	for (iter = ctx.req->fields, \
	     name = (const char *)ctx.req + iter->name_off; \
	     (iter < (ctx.req->fields + ctx.req->nr_fields)) && \
	     (value = (const char *)ctx.req + iter->value_off); \
	     iter++, name = (const char *)ctx.req + iter->name_off)

/* Imported functions from the host/runtime */
__attribute__((import_module("env"), import_name("nxt_wasm_get_init_mem_size")))
u32 nxt_wasm_get_init_mem_size(void);
__attribute__((import_module("env"), import_name("nxt_wasm_response_end")))
void nxt_wasm_response_end(void);
__attribute__((import_module("env"), import_name("nxt_wasm_send_headers")))
void nxt_wasm_send_headers(u32 offset);
__attribute__((import_module("env"), import_name("nxt_wasm_send_response")))
void nxt_wasm_send_response(u32 offset);
__attribute__((import_module("env"), import_name("nxt_wasm_set_resp_status")))
void nxt_wasm_set_resp_status(u32 status);

extern void luw_module_init_handler(void);
extern void luw_module_end_handler(void);
extern void luw_request_init_handler(void);
extern void luw_request_end_handler(void);
extern void luw_response_end_handler(void);
extern int luw_request_handler(u8 *addr);
extern void luw_free_handler(u32 addr);
extern u32 luw_malloc_handler(size_t size);

#pragma GCC visibility push(default)

extern void luw_init_ctx(luw_ctx_t *ctx, u8 *addr, size_t offset);
extern int luw_set_req_buf(luw_ctx_t *ctx, u8 **buf, unsigned int flags);
extern const char *luw_get_http_path(const luw_ctx_t *ctx);
extern const char *luw_get_http_method(const luw_ctx_t *ctx);
extern const char *luw_get_http_version(const luw_ctx_t *ctx);
extern const char *luw_get_http_query(const luw_ctx_t *ctx);
extern const char *luw_get_http_remote(const luw_ctx_t *ctx);
extern const char *luw_get_http_local_addr(const luw_ctx_t *ctx);
extern const char *luw_get_http_local_port(const luw_ctx_t *ctx);
extern const char *luw_get_http_server_name(const luw_ctx_t *ctx);
extern const u8 *luw_get_http_content(const luw_ctx_t *ctx);
extern u64 luw_get_http_content_len(const luw_ctx_t *ctx);
extern size_t luw_get_http_content_sent(const luw_ctx_t *ctx);
extern u64 luw_get_http_total_content_sent(const luw_ctx_t *ctx);
extern bool luw_http_is_tls(const luw_ctx_t *ctx);
extern void luw_http_hdr_iter(luw_ctx_t *ctx,
			      bool (*luw_http_hdr_iter_func)(luw_ctx_t *ctx,
							     const char *name,
							     const char *value,
							     void *data),
			      void *user_data);
extern const char *luw_http_hdr_get_value(const luw_ctx_t *ctx,
					  const char *hdr);
extern size_t luw_get_response_data_size(const luw_ctx_t *ctx);
extern int luw_mem_writep(luw_ctx_t *ctx, const char *fmt, ...);
extern size_t luw_mem_writep_data(luw_ctx_t *ctx, const u8 *src, size_t size);
extern void luw_req_buf_append(luw_ctx_t *ctx, const u8 *src);
extern void luw_req_buf_copy(luw_ctx_t *ctx, const u8 *src);
extern ssize_t luw_mem_splice_file(const u8 *src, int fd);
extern size_t luw_mem_fill_buf_from_req(luw_ctx_t *ctx, size_t from);
extern void luw_mem_reset(luw_ctx_t *ctx);
extern void luw_http_set_response_status(luw_http_status_t status);
extern void luw_http_send_response(const luw_ctx_t *ctx);
extern void luw_http_init_headers(luw_ctx_t *ctx, size_t nr, size_t offset);
extern void luw_http_add_header(luw_ctx_t *ctx, const char *name,
				const char *value);
extern void luw_http_send_headers(const luw_ctx_t *ctx);
extern void luw_http_response_end(void);
extern u32 luw_mem_get_init_size(void);

/*
 * Convenience wrappers for the Rust bindings, not for general consumption.
 */
extern void *luw_malloc(size_t size);
extern void luw_free(void *ptr);

#pragma GCC visibility pop

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* _UNIT_WASM_H_ */
