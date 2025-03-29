use crate::api;
use crate::c;

use super::*;

/// Serialize and output a slotted object
#[inline(always)]
pub fn slot(slotted_obj: &mut [u8], slot_no: u32) -> Result<u64> {
    let res = unsafe {
        c::slot(
            slotted_obj.as_mut_ptr() as u32,
            slotted_obj.len() as u32,
            slot_no,
        )
    };

    res.into()
}

/// Free up a currently occupied slot
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
    if PARAM_KEYLET_LEN != KEYLET_LEN || PARAM_KEYLET_LEN != HASH_LEN {
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
#[inline(always)]
pub fn slot_subarray(parent_slot: u32, array_id: u32, new_slot: u32) -> Result<u64> {
    api_3arg_call(parent_slot, array_id, new_slot, c::slot_subarray)
}

/// Index into a slotted object and assign a sub-object to another slot
#[inline(always)]
pub fn slot_subfield(parent_slot: u32, field_id: FieldId, new_slot: u32) -> Result<u64> {
    api_3arg_call(parent_slot, field_id as _, new_slot, c::slot_subfield)
}

/// Retrieve the field code of an object in a slot and, optionally, some other information
#[inline(always)]
pub fn slot_type(slot_no: u32, flags: SlotTypeFlags) -> Result<FieldOrXrpAmount> {
    match flags {
        SlotTypeFlags::Field => {
            let res = unsafe { c::slot_type(slot_no, 0) };

            match res {
                res if res >= 0 => Ok(FieldOrXrpAmount::Field(unsafe {
                    core::mem::transmute::<u32, api::FieldId>(res as u32)
                })),
                _ => Err(Error::from_code(res as _)),
            }
        }

        SlotTypeFlags::XrpAmount => {
            let res = unsafe { c::slot_type(slot_no, 1) };

            match res {
                1 => Ok(FieldOrXrpAmount::XrpAmount),
                res if res >= 0 => Ok(FieldOrXrpAmount::NonXrpAmount),
                _ => Err(Error::from_code(res as _)),
            }
        }
    }
}

/// Parse the STI_AMOUNT in the specified slot and return it as an XFL enclosed number
#[inline(always)]
pub fn slot_float(slot_no: u32) -> Result<XFL> {
    XFL::from_verified_i64(unsafe { c::slot_float(slot_no) })
}
