use core::mem::MaybeUninit;

use crate::c;

use super::*;

/// Serialize and output a slotted object
#[inline(always)]
pub fn slot<const SLOT_LEN: usize>(slot_no: u32) -> Result<[u8; SLOT_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> =
            unsafe { c::slot(buffer_mut_ptr as u32, SLOT_LEN as u32, slot_no).into() };

        result
    };

    init_buffer_mut(func)
}

/// Free up a currently occupied slot. Returns 1 on success.
#[inline(always)]
pub fn slot_clear(slot_no: u32) -> Result<u64> {
    unsafe { c::slot_clear(slot_no) }.into()
}

/// Count the elements of an array object in a slot
#[inline(always)]
pub fn slot_count(slot_no: u32) -> Result<u64> {
    unsafe { c::slot_count(slot_no) }.into()
}

/// Locate an object based on its keylet and place it into a slot
#[inline(always)]
pub fn slot_set<const PARAM_KEYLET_LEN: usize>(
    keylet: &[u8; PARAM_KEYLET_LEN],
    slot_no: u32,
) -> Result<u64> {
    // The Hook APIs which accept a 34 byte keylet will also generally accept a 32 byte canonical transaction hash.
    // TODO: are these the only allowed keylet kinds?
    if PARAM_KEYLET_LEN != KEYLET_LEN && PARAM_KEYLET_LEN != HASH_LEN {
        return Err(Error::InvalidKeyletLength);
    }
    let res = unsafe { c::slot_set(keylet.as_ptr() as u32, PARAM_KEYLET_LEN as u32, slot_no) };

    res.into()
}

/// Compute the serialized size of an object in a slot
#[inline(always)]
pub fn slot_size(slot_no: u32) -> Result<u64> {
    unsafe { c::slot_size(slot_no) }.into()
}

/// Index into a slotted array and assign a sub-object to another slot
///
/// @param array_index The sf code of the field you are searching for.
/// To compute this manually take the serialized type and shift it into the 16 highest bits of uint32_t, then take the field and place it in the 16 lowest bits.
/// For example: sfEmitNonce has type 5 and field 11 thus its value is 0x050BU
#[inline(always)]
pub fn slot_subarray(parent_slot: u32, array_index: u32, new_slot: u32) -> Result<u64> {
    unsafe { c::slot_subarray(parent_slot, array_index, new_slot) }.into()
}

/// Index into a slotted object and assign a sub-object to another slot
#[inline(always)]
pub fn slot_subfield(parent_slot: u32, field_id: FieldId, new_slot: u32) -> Result<u64> {
    let res = unsafe { c::slot_subfield(parent_slot, field_id as u32, new_slot) };
    res.into()
}

/// Retrieve the field code of an object in a slot and, optionally, some other information
#[inline(always)]
pub fn slot_type(slot_no: u32, flags: SlotTypeFlags) -> Result<FieldOrXahAmount> {
    match flags {
        SlotTypeFlags::Field => {
            let res = unsafe { c::slot_type(slot_no, 0) };

            match res {
                res if res >= 0 => Ok(FieldOrXahAmount::Field((res as u32).into())),
                _ => Err(Error::from_code(res as _)),
            }
        }

        SlotTypeFlags::STIAmount => {
            let res = unsafe { c::slot_type(slot_no, 1) };

            match res {
                1 => Ok(FieldOrXahAmount::XahAmount),
                res if res >= 0 => Ok(FieldOrXahAmount::NonXahAmount),
                _ => Err(Error::from_code(res as _)),
            }
        }
    }
}

/// Parse the STI_AMOUNT in the specified slot and return it as an XFL enclosed number
/// XAH Balance is normalized by 10^-6 (drops decimals) for some reason when using slot_float
#[inline(always)]
pub fn slot_float(slot_no: u32) -> Result<XFL> {
    XFL::from_verified_i64(unsafe { c::slot_float(slot_no) })
}
