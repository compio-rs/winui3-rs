#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]
#![cfg_attr(all(not(feature = "std")), no_std)]

#[allow(unused_extern_crates)]
extern crate self as windows;

include!("Windows/mod.rs");
