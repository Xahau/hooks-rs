//! This crate allows you to write XRPL hooks in Rust.
//!
//! Warning: this is a pre-alpha version of the library. It is not recommended to use it in production yet
//! and many things are subject to change or simply not implemented yet.
//!
//! hooks-rs provides a few things for the hook builders:
//!
//! 1. Abstraction over the XRPL Hooks C API
//! 2. A set of pre-built transaction builders, like `XrpPaymentBuilder`
//! 3. Utility methods to make working with hooks easier, such as `max_iter` or `ComparableArray`.

#![no_std]
#![deny(
    warnings,
    clippy::all,
    missing_copy_implementations,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    non_ascii_idents,
    unreachable_pub
)]
#![doc(test(attr(deny(warnings))))]
#![feature(maybe_uninit_uninit_array, maybe_uninit_array_assume_init)]
/// Internal C bindings. Unless if you are creating something very low-level,
/// you should not need to use this module directly.
#[allow(missing_docs)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// XRPL Hooks API that abstracts the usage of external C API
pub mod api;

/// Utility methods to make working with hooks easier
pub mod utils;

/// Transaction builders. It is a lot of manual work to build an XRPL transaction.
/// This module provides a few pre-built transaction builders as well as a generic
/// buffer and builder that can be used to build any transaction.
pub mod transaction;

// Prelude
pub use {api::*, transaction::*, utils::*};

#[cfg(not(test))]
use core::panic::PanicInfo;
/// You should use rollback() instead of native panic!() macro
#[cfg(not(test))]
#[inline(always)]
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}

#[cfg(doc)]
#[doc(hidden)]
#[path = "../examples"]
mod examples {}
