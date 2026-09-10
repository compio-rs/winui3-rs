//! WebView2 Win32 bindings for Rust
//!
//! This crate provides type bindings to the WebView2 API,
//! as well as a reimplementation of the WebView2Loader in pure Rust.

#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]
mod bindings;
pub use bindings::*;

mod native;
pub use native::*;
