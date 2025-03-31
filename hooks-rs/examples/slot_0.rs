#![feature(maybe_uninit_uninit_array)]
#![no_std]
#![no_main]

use hooks_rs::*;

const ACCOUNT_SLOT_ID: u32 = 1;

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

    // Will throw if account does not exist, but we know the account exists, so it shouldn't throw
    slot_set(&account_keylet, ACCOUNT_SLOT_ID).unwrap_line_number();
    let account_slot_size = slot_size(ACCOUNT_SLOT_ID).unwrap_line_number();

    let _ = trace(
        b"account_slot_size",
        &account_slot_size.to_be_bytes(),
        DataRepr::AsHex,
    );

    if account_slot_size == 0 {
        rollback(b"account_slot_size == 0", -1);
    }

    let result_code = slot_clear(ACCOUNT_SLOT_ID).unwrap_line_number();

    if result_code != 1 {
        rollback(b"slot_clear failed", result_code as i64)
    }

    // Because slot is cleared, slot_size should throw
    match slot_size(ACCOUNT_SLOT_ID) {
        Ok(_) => {
            rollback(b"cleared slot exists", -2);
        }
        Err(err) => {
            if err != Error::DoesntExist {
                rollback(b"err != Error::DoesntExist", err.into());
            }
        }
    }

    // Another way to get the account balance
    accept(b"passing", 0);
}
