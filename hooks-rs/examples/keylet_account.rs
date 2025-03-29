//! A hook that accepts any transaction coming through it

#![no_std]
#![no_main]

use hooks_rs::*;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    // Every hook needs to import guard function
    // and use it at least once
    max_iter(1);
    let _ = trace(b"accept.rs: Called.", b"", DataRepr::AsUTF8);

    // Accept all
    accept(b"accept.rs: Finished.", line!().into());
}
