#![feature(maybe_uninit_uninit_array)]
#![no_std]
#![no_main]

use hooks_rs::*;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    max_iter(33);

    let hash = util_sha512h(b"hihihi").unwrap_line_number();
    // https://emn178.github.io/online-tools/sha512.html
    let expected_hash: [u8; 32] = [
        0x5b, 0xaf, 0x5c, 0x46, 0xf2, 0xcb, 0xc6, 0x77, 0x2c, 0xc2, 0x47, 0xba, 0x4a, 0xe1, 0xfc,
        0xd5, 0x2b, 0xee, 0xa8, 0x44, 0xaf, 0x1e, 0x8d, 0x3d, 0x8c, 0xe3, 0x37, 0x1a, 0x29, 0x79,
        0x49, 0x9b,
    ];

    if hash.len() != expected_hash.len() {
        rollback(b"hash.len() != expected_hash.len()", -2);
    }

    let mut i = 0;
    while i < hash.len() {
        if hash[i] != expected_hash[i] {
            rollback(b"_g", -1)
        }
        i += 1;
    }

    accept(b"passing", 0);
}
