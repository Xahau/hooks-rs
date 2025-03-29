#![feature(maybe_uninit_uninit_array)]
#![no_std]
#![no_main]

use hooks_rs::*;

const ACCOUNT_SLOT_ID: u32 = 1;
const ACCOUNT_BALANCE_SLOT_ID: u32 = 3;

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

    // Does not exist on Xahau testnet
    // Check existence on https://explorer.xahau-test.net/rDAFKjBukJ6r197ZPH7W5QkJDqVQLZhxud
    let nonexistent_account_id: [u8; ACC_ID_LEN] = [
        0x8d, 0xff, 0x79, 0x78, 0xae, 0xb4, 0x94, 0x45, 0xc7, 0xcf, 0x8c, 0x93, 0x3a, 0x4a, 0xdf,
        0xa5, 0x23, 0x14, 0x06, 0xfa,
    ];
    let nonexistent_account_keylet =
        util_keylet(KeyletAccount::new(&nonexistent_account_id).build()).unwrap_line_number();

    match slot_set(&nonexistent_account_keylet, 2) {
        Ok(_) => {
            // This should not happen
            rollback(
                b"Should not be able to set slot for nonexistent account",
                -2,
            );
        }
        Err(e) => {
            // This is expected
            if e != Error::DoesntExist {
                // This should not happen
                rollback(
                    b"Should not be able to set slot for nonexistent account",
                    -3,
                );
            }
        }
    }

    slot_subfield(ACCOUNT_SLOT_ID, FieldId::Balance, ACCOUNT_BALANCE_SLOT_ID).unwrap_line_number();
    let account_balance_slot = slot::<64>(ACCOUNT_BALANCE_SLOT_ID).unwrap_line_number();

    accept(&account_balance_slot, 0);
}
