/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Andrew Clayton
 * Copyright (C) F5, Inc.
 */

pub const LUW_VERSION_MAJOR: i32 = 0;
pub const LUW_VERSION_MINOR: i32 = 2;
pub const LUW_VERSION_PATCH: i32 = 0;

pub const LUW_VERSION_NUMBER: i32 =
    (LUW_VERSION_MAJOR << 24) |
    (LUW_VERSION_MINOR << 16) |
    (LUW_VERSION_PATCH << 8);

pub const LUW_SRB_NONE:      u32 = luw_srb_flags_t::LUW_SRB_NONE as u32;
pub const LUW_SRB_APPEND:    u32 = luw_srb_flags_t::LUW_SRB_APPEND as u32;
pub const LUW_SRB_ALLOC:     u32 = luw_srb_flags_t::LUW_SRB_ALLOC as u32;
pub const LUW_SRB_FULL_SIZE: u32 = luw_srb_flags_t::LUW_SRB_FLAGS_ALL as u32;
