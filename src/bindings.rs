//! Raw bindings to the libuiohook C library.
//!
//! This module contains the auto-generated bindings to the libuiohook C library.
//! These bindings are created using the `bindgen` crate and should not be used directly.
//! Instead, use the safe Rust wrappers provided in other modules of this crate.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
