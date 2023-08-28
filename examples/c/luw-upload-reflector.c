/* SPDX-License-Identifier: Apache-2.0 */

/*
 * examples/c/luw-upload-reflector.c - Example of writing a WASM module for
 *				       use with Unit using libunit-wasm
 *
 * Copyright (C) Andrew Clayton
 * Copyright (C) F5, Inc.
 */

#define _XOPEN_SOURCE	500

#include <stdio.h>
#include <stdlib.h>

#include "unit/unit-wasm.h"

static luw_ctx_t ctx;

static size_t total_response_sent;

static u8 *request_buf;

/*
 * While these first two _handlers_ aren't technically required, they
 * could be combined or the code could just go in upload_reflector(),
 * they demonstrate their use in ensuring the module is in the right
 * state for a new request.
 */
__luw_export_name("luw_response_end_handler")
void luw_response_end_handler(void)
{
	total_response_sent = 0;
}

__luw_export_name("luw_request_end_handler")
void luw_request_end_handler(void)
{
	if (!request_buf)
		return;

	free(request_buf);
	request_buf = NULL;
}

static int upload_reflector(luw_ctx_t *ctx)
{
	size_t write_bytes;

	/* Send headers */
	if (total_response_sent == 0) {
		static const char *defct = "application/octet-stream";
		const char *ct = luw_http_hdr_get_value(ctx, "Content-Type");
		char clen[32];

		snprintf(clen, sizeof(clen), "%lu",
			 luw_get_http_content_len(ctx));

		luw_http_init_headers(ctx, 2, 0);
		luw_http_add_header(ctx, "Content-Type", ct ? ct : defct);
		luw_http_add_header(ctx, "Content-Length", clen);
		luw_http_send_headers(ctx);
	}

	write_bytes = luw_mem_fill_buf_from_req(ctx, total_response_sent);
	total_response_sent += write_bytes;

	luw_http_send_response(ctx);

	if (total_response_sent == luw_get_http_content_len(ctx)) {
		/* Tell Unit no more data to send */
		luw_http_response_end();
	}

	return 0;
}

__luw_export_name("luw_request_handler")
int luw_request_handler(u8 *addr)
{
	if (!request_buf) {
		luw_init_ctx(&ctx, addr, 0 /* Response offset */);
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

	upload_reflector(&ctx);

	return 0;
}
