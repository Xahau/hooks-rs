use core::mem::MaybeUninit;

use crate::c;

use super::*;

/// Get the burden of a hypothetically emitted transaction
#[inline(always)]
pub fn etxn_burden() -> i64 {
    unsafe { c::etxn_burden() }
}

/// Produce emit details for for a soon-to-be emitted transaction.
/// Normally, it is appended at the end of the transaction buffer.
///
/// It is generally recommended to use [insert_etxn_details_from_ptr](insert_etxn_details_from_ptr)
/// instead of this function to avoid unnecessary memory allocations.
#[inline(always)]
pub fn etxn_details<const EMIT_DETAILS_LEN: usize>() -> Result<[u8; EMIT_DETAILS_LEN]> {
    init_buffer_mut(|buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> =
            unsafe { c::etxn_details(buffer_mut_ptr as u32, EMIT_DETAILS_LEN as u32).into() };

        result
    })
}

/// Produce emit details for for a soon-to-be emitted transaction.
/// Normally, it is appended at the end of the transaction buffer.
///
/// # Example
/// ```
/// let buf_mut_ptr: *mut MaybeUninit<u8> = buf.as_mut_ptr();
/// let pos = 45;
/// let insert_etxn_details_from_ptr_result: Result<u64> = insert_etxn_details_from_ptr(unsafe { buf_mut_ptr.add(pos) as u32 }, 138);
/// match insert_etxn_details_from_ptr_result {
///     Err(e) => return Err(e),
///     Ok(_) => {}
/// }
/// ```
#[inline(always)]
pub fn insert_etxn_details_from_ptr(txn_buffer_mut_ptr: u32, emit_details_len: u32) -> Result<u64> {
    unsafe { c::etxn_details(txn_buffer_mut_ptr, emit_details_len).into() }
}

/// Estimate the required fee for a txn to be emitted successfully
///
/// Note that this function can only be called after the transaction buffer is filled except the fee part.
/// The fee part should only include its 'header' part and the rest of it filled with zero. Zero is not
/// equivalent to being uninitialized.
///
/// # Example
/// ```
/// let xrp_payment_txn_buffer = XrpPaymentBuilder::uninit_buffer();
///
/// // fill the tranasction buffer
/// // ...
/// // ...
///
/// // finally, once the transaction buffer is filled except the fee part, estimate the fee.
/// let fee = match etxn_fee_base(&xrp_payment_txn_buffer);
/// ```
#[inline(always)]
pub fn etxn_fee_base<T>(tx_blob: &[T]) -> Result<u64> {
    etxn_fee_base_from_ptr(tx_blob.as_ptr(), tx_blob.len())
}

/// Estimate the required fee for a txn to be emitted successfully from a pointer to the transaction buffer.
///
/// Does the same thing as [etxn_fee_base](etxn_fee_base) but takes a pointer to the transaction buffer instead of a slice.
///
/// # Example
/// ```
/// let xrp_payment_txn_buffer = XrpPaymentBuilder::uninit_buffer();
///
/// // fill the tranasction buffer
/// // ...
/// // ...
///
/// // finally, once the transaction buffer is filled except the fee part, estimate the fee.
/// let fee = match etxn_fee_base_from_ptr(xrp_payment_txn_buffer.as_ptr(), xrp_payment_txn_buffer.len());
/// ```
#[inline(always)]
pub fn etxn_fee_base_from_ptr<T>(tx_blob_ptr: *const T, tx_blob_len: usize) -> Result<u64> {
    unsafe { c::etxn_fee_base(tx_blob_ptr as u32, tx_blob_len as u32).into() }
}

/// Generate a 32 byte nonce for use in an emitted transaction
#[inline(always)]
pub fn etxn_nonce() -> Result<[u8; NONCE_LEN]> {
    init_buffer_mut(|buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> =
            unsafe { c::etxn_nonce(buffer_mut_ptr as u32, NONCE_LEN as u32).into() };

        result
    })
}

/// Estimate the required fee for a txn to be emitted successfully
#[inline(always)]
pub fn etxn_reserve(count: u32) -> Result<u64> {
    unsafe { c::etxn_reserve(count).into() }
}

/// Get the generation of a hypothetically emitted transaction
#[inline(always)]
pub fn etxn_generation() -> i64 {
    unsafe { c::etxn_generation() }
}

/// Emit a new transaction from the hook and return the 32-bytes long txn hash
/// T should almost always be `MaybeUninit<u8>` or `u8` type depending on your use case.
///
/// # Example
/// ```
/// let xrp_payment_txn_builder = XrpPaymentBuilder::new(1000, &otxn_account, 0, 0);
/// let mut xrp_payment_txn_buffer = XrpPaymentBuilder::uninit_buffer();
/// match xrp_payment_txn_builder.build(&mut xrp_payment_txn_buffer) {
///     Ok(ptr) => ptr,
///     Err(err) => {
///         rollback(b"could not build xrp payment txn", err.into());
///     }
/// };
/// let txn_hash = match emit(
///     &xrp_payment_txn_buffer,
/// ) {
///     Ok(hash) => hash,
///     Err(err) => {
///         rollback(b"could not emit xrp payment txn", err.into());
///     }
/// };
/// ```
#[inline(always)]
pub fn emit<T>(tx: &[T]) -> Result<[u8; HASH_LEN]> {
    emit_from_ptr(tx.as_ptr() as *const u8, tx.len() as u32)
}

/// Emit a new transaction from the hook and return the 32-bytes long txn hash.
///
/// Same as [emit](emit) but takes a pointer to the transaction buffer instead of a slice.
/// This might be useful for dealing with raw pointers.
#[inline(always)]
pub fn emit_from_ptr(tx_ptr: *const u8, tx_len: u32) -> Result<[u8; HASH_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> = unsafe {
            c::emit(
                buffer_mut_ptr as u32,
                HASH_LEN as u32,
                tx_ptr as u32,
                tx_len,
            )
            .into()
        };

        result
    };

    init_buffer_mut(func)
}
