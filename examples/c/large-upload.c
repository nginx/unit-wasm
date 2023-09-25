/* SPDX-License-Identifier: Apache-2.0 */

/* examples/c/large-upload.c - Example of handling request payload larger
 *			       larger than the shared memory
 *
 * Copyright (C) Andrew Clayton
 * Copyright (C) F5, Inc.
 */

#define _XOPEN_SOURCE	500

#define _FILE_OFFSET_BITS 64

#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>

#include "unit/unit-wasm.h"

static luw_ctx_t ctx;
static u8 *request_buf;
static unsigned long long total_bytes_wrote;
static int fd;

__luw_export_name("luw_module_end_handler")
void luw_module_end_handler(void)
{
        free(request_buf);
}

__luw_export_name("luw_module_init_handler")
void luw_module_init_handler(void)
{
        request_buf = malloc(luw_mem_get_init_size());
}

__luw_export_name("luw_response_end_handler")
void luw_response_end_handler(void)
{
	close(fd);
	total_bytes_wrote = 0;
}

__luw_export_name("luw_request_handler")
int luw_request_handler(u8 *addr)
{
	ssize_t bytes_wrote;

	if (total_bytes_wrote == 0) {
		luw_init_ctx(&ctx, addr, 0);
		luw_set_req_buf(&ctx, &request_buf, LUW_SRB_NONE);

		fd = open("/var/tmp/large-file.dat", O_CREAT|O_TRUNC|O_WRONLY,
			  0666);
	}

	bytes_wrote = luw_mem_splice_file(addr, fd);
	if (bytes_wrote == -1)
		return -1;

	total_bytes_wrote += bytes_wrote;
	if (total_bytes_wrote == luw_get_http_content_len(&ctx))
		luw_http_response_end();

	return 0;
}
