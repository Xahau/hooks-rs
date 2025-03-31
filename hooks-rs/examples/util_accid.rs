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
    max_iter(1);

    let acc_id = match util_accid(b"rLqUFYGLMBS9jF63iRkadvu3cTixadRTd3") {
        Ok(acc_id) => acc_id,
        Err(err) => {
            rollback(b"util_accid failed.", err.into());
        }
    };

    // decodeAccountID("rLqUFYGLMBS9jF63iRkadvu3cTixadRTd3")
    // .toString(`hex`)
    // .toUpperCase(),
    let expected_acc_id: [u8; 20] = [
        0xD9, 0x88, 0x38, 0xFB, 0xFE, 0x5F, 0x5E, 0xC2, 0x65, 0xB8, 0xB6, 0xCF, 0xF7, 0xF0, 0x2D,
        0xDF, 0x59, 0x05, 0xEA, 0xCB,
    ];

    if !is_buffer_equal(&acc_id, &expected_acc_id) {
        // 14
        rollback(b"!is_buffer_equal(acc_id, expected_acc_id)", -1)
    }

    accept(&acc_id, 0);
}
