use core::mem::MaybeUninit;

use crate::c;

use super::*;

/// One must use `KeyletPayloadBuilder` to create a `KeyletPayload`.
/// The `KeyletPayload` is then used to create a keylet.
pub trait KeyletPayloadBuilder {
    /// Build the keylet payload.
    fn build(&self) -> KeyletPayload;
}

/// Parameters for `util_keylet`.
#[derive(Clone, Copy)]
pub struct KeyletPayload {
    keylet_type: KeyletType,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
}

/// Keylet for a hook
pub struct KeyletHook<'a> {
    account_id: &'a [u8; ACC_ID_LEN],
}

/// Keylet for an account
pub struct KeyletAccount<'a> {
    account_id: &'a [u8; ACC_ID_LEN],
}

/// Convert a 20 byte Account ID to an r-address of 25 and 35 characters in length.
///
/// The resulting buffer might not be fully populated because size of an r-address varies.
/// The rest of the buffer will be filled with zeroes.
#[inline(always)]
pub fn util_raddr(accid: &[u8; 20]) -> Result<[u8; 35]> {
    let mut uninit_r_address_buffer: [MaybeUninit<u8>; 35] = MaybeUninit::uninit_array();

    let u64_uninit_r_address_buffer_ptr = uninit_r_address_buffer.as_mut_ptr() as *mut u64;
    unsafe {
        u64_uninit_r_address_buffer_ptr.write(0); // 8 bytes
        u64_uninit_r_address_buffer_ptr.add(1).write(0); // 16 bytes
        u64_uninit_r_address_buffer_ptr.add(2).write(0); // 24 bytes
        u64_uninit_r_address_buffer_ptr.add(3).write(0); // 32 bytes
    }
    unsafe {
        let r_address_last_3_bytes_ptr = uninit_r_address_buffer.get_unchecked_mut(32).as_mut_ptr();
        r_address_last_3_bytes_ptr.write(0);
        r_address_last_3_bytes_ptr.add(1).write(0);
        r_address_last_3_bytes_ptr.add(2).write(0);
    };
    let mut r_address_buffer = unsafe {
        uninit_r_address_buffer
            .as_ptr()
            .cast::<[u8; 35]>()
            .read_volatile()
    };

    let result: Result<u64> = unsafe {
        c::util_raddr(
            r_address_buffer.as_mut_ptr() as u32,
            35,
            accid.as_ptr() as u32,
            ACC_ID_LEN as u32,
        )
        .into()
    };

    match result {
        Ok(_) => Ok(r_address_buffer),
        Err(e) => Err(e),
    }
}

/// Convert an r-address into a 20 byte Account ID
///
/// R_ADDRESS_LEN must be >= 25 && <= 35.
#[inline(always)]
pub fn util_accid<const R_ADDRESS_LEN: usize>(raddr_in: &[u8; R_ADDRESS_LEN]) -> Result<[u8; 20]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> = unsafe {
            c::util_raddr(
                buffer_mut_ptr as u32,
                ACC_ID_LEN as u32,
                raddr_in.as_ptr() as u32,
                R_ADDRESS_LEN as u32,
            )
            .into()
        };

        result
    };

    init_buffer_mut::<ACC_ID_LEN, _>(func)
}

/// Verify a cryptographic signature
///
/// If the public key is prefixed with 0xED then use ED25519. Otherwise assume SECP256k1.
#[inline(always)]
pub fn util_verify(payload: &[u8], signature: &[u8], publickey: &[u8]) -> Result<bool> {
    let result: Result<u64> = unsafe {
        c::util_verify(
            payload.as_ptr() as _,
            payload.len() as _,
            signature.as_ptr() as _,
            signature.len() as _,
            publickey.as_ptr() as _,
            publickey.len() as _,
        )
        .into()
    };

    match result {
        Ok(0) => Ok(false),
        Ok(1) => Ok(true),
        Err(e) => Err(e),
        _ => unreachable!(),
    }
}

/// Compute an sha512-half over some data
#[inline(always)]
pub fn util_sha512h(data_in: &[u8]) -> Result<[u8; HASH_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> = unsafe {
            c::util_sha512h(
                buffer_mut_ptr as u32,
                HASH_LEN as u32,
                data_in.as_ptr() as u32,
                data_in.len() as u32,
            )
            .into()
        };

        result
    };

    init_buffer_mut(func)
}

/// Compute a serialized keylet of a given type
#[inline(always)]
pub fn util_keylet(keylet_payload: KeyletPayload) -> Result<[u8; KEYLET_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        unsafe {
            c::util_keylet(
                buffer_mut_ptr as u32,
                KEYLET_LEN as u32,
                keylet_payload.keylet_type.into(),
                keylet_payload.a,
                keylet_payload.b,
                keylet_payload.c,
                keylet_payload.d,
                keylet_payload.e,
                keylet_payload.f,
            )
        }
        .into()
    };

    init_buffer_mut(func)
}

impl<'a> KeyletHook<'_> {
    /// Create a new `KeyletHook` instance.
    #[inline(always)]
    pub fn new(account_id: &'a [u8; ACC_ID_LEN]) -> KeyletHook<'a> {
        KeyletHook { account_id }
    }
}

impl KeyletPayloadBuilder for KeyletHook<'_> {
    #[inline(always)]
    fn build(&self) -> KeyletPayload {
        KeyletPayload {
            keylet_type: KeyletType::Hook,
            a: self.account_id.as_ptr() as u32,
            b: ACC_ID_LEN as u32,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
        }
    }
}

impl<'a> KeyletAccount<'_> {
    /// Create a new `KeyletAccount` instance.
    #[inline(always)]
    pub fn new(account_id: &'a [u8; ACC_ID_LEN]) -> KeyletAccount<'a> {
        KeyletAccount { account_id }
    }
}

impl KeyletPayloadBuilder for KeyletAccount<'_> {
    #[inline(always)]
    fn build(&self) -> KeyletPayload {
        KeyletPayload {
            keylet_type: KeyletType::Account,
            a: self.account_id.as_ptr() as u32,
            b: ACC_ID_LEN as u32,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
        }
    }
}
