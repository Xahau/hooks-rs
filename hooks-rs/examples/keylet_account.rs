#![feature(maybe_uninit_uninit_array)]
#![no_std]
#![no_main]

use hooks_rs::*;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

// Example: https://github.com/Xahau/TreasuryHook/blob/ac8e2f7db4b687450d9ca1cba412bfac3b1a87bc/treasuryInvoke.c#L150-L153
#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    // Every hook needs to import guard function
    // and use it at least once
    max_iter(1);
    // https://explorer.xahau-test.net/r3zEReqN3Ge3g1GXUThuVSrn4dM8xpoA7i
    // 579CA1B8B51227C2E40AB3217D9421EFB13B1364
    let account_id: [u8; ACC_ID_LEN] = [
        0x57, 0x9C, 0xA1, 0xB8, 0xB5, 0x12, 0x27, 0xC2, 0xE4, 0x0A, 0xB3, 0x21, 0x7D, 0x94, 0x21,
        0xEF, 0xB1, 0x3B, 0x13, 0x64,
    ];
    let keylet_payload = KeyletAccount::new(&account_id).build();
    let account_keylet = util_keylet(keylet_payload).unwrap_line_number();

    let _ = trace(b"keylet", &account_keylet, DataRepr::AsHex);

    // Check at https://richardah.github.io/xrpl-keylet-tools/
    // by inserting r3zEReqN3Ge3g1GXUThuVSrn4dM8xpoA7i into the account field
    // to get 0061355E27663861169420D62A5955FECE7FB519121F8CDC15963FD1561653531F9C
    //
    // Cannot do this because Rust compiled down to wasm produces an unguarded loop
    // TODO: if we want to make this possible, we might need to modify LLVM
    // but we don't do it now because the scope becomes too large
    // const EXPECTED_KEYLET: [u8; 34] = [
    //     0x00, 0x61, 0x35, 0x5E, 0x27, 0x66, 0x38, 0x61, 0x16, 0x94, 0x20, 0xD6, 0x2A, 0x59, 0x55,
    //     0xFE, 0xCE, 0x7F, 0xB5, 0x19, 0x12, 0x1F, 0x8C, 0xDC, 0x15, 0x96, 0x3F, 0xD1, 0x56, 0x16,
    //     0x53, 0x31, 0xF9, 0xC,
    // ];
    //
    // Check in TS test file
    accept(&account_keylet, 0);
}
