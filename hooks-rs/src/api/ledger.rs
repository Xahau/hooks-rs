use core::mem::MaybeUninit;

use super::*;
use crate::c;

/// Fetch the fee base of the current ledger
#[inline(always)]
pub fn fee_base() -> i64 {
    unsafe { c::fee_base() }
}

/// Fetch the current ledger sequence number
#[inline(always)]
pub fn ledger_seq() -> i64 {
    unsafe { c::ledger_seq() }
}

/// Fetch the last time the ledger was closed
#[inline(always)]
pub fn ledger_last_time() -> i64 {
    unsafe { c::ledger_last_time() }
}

/// Retreive the 32 byte namespace biased SHA512H of the last closed ledger
#[inline(always)]
pub fn ledger_last_hash() -> Result<[u8; 32]> {
    init_buffer_mut(|buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> =
            unsafe { c::ledger_last_hash(buffer_mut_ptr as u32, 32_u32).into() };

        result
    })
}

/// Generate a 32 byte nonce for use in an emitted transaction
#[inline(always)]
pub fn ledger_nonce() -> Result<[u8; 32]> {
    init_buffer_mut(|buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> = unsafe { c::ledger_nonce(buffer_mut_ptr as u32, 32_u32).into() };

        result
    })
}

/// Search for a keylet within a specified range on the current ledger
#[inline(always)]
pub fn ledger_keylet(
    low_boundary_keylet: [u8; KEYLET_LEN],
    high_boundary_keylet: [u8; KEYLET_LEN],
) -> Result<[u8; KEYLET_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        unsafe {
            c::ledger_keylet(
                buffer_mut_ptr as u32,
                KEYLET_LEN as u32,
                low_boundary_keylet.as_ptr() as u32,
                KEYLET_LEN as u32,
                high_boundary_keylet.as_ptr() as u32,
                KEYLET_LEN as u32,
            )
        }
        .into()
    };

    init_buffer_mut(func)
}
