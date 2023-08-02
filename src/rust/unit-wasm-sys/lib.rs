/* SPDX-License-Identifier: Apache-2.0 */

/*
 * Copyright (C) Timo Stark
 * Copyright (C) F5, Inc.
 */

#[doc(hidden)]
mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    include!("macros.rs");
}

#[doc(no_inline)]
pub use bindings::*;
