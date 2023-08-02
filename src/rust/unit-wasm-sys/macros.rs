/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) F5, Inc.
 */

pub const LUW_VERSION_MAJOR: i32 = 0;
pub const LUW_VERSION_MINOR: i32 = 1;
pub const LUW_VERSION_PATCH: i32 = 0;

pub const LUW_VERSION_NUMBER: i32 =
    (LUW_VERSION_MAJOR << 24) |
    (LUW_VERSION_MINOR << 16) |
    (LUW_VERSION_PATCH << 8);
