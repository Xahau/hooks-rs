#![feature(maybe_uninit_uninit_array)]
#![no_std]
#![no_main]

use hooks_rs::*;

const OTXN_SLOT_ID: u32 = 1;
const MEMOS_SLOT_ID: u32 = 2;
const MEMOS_INDEX_0_ID: u32 = 3;
const MEMOS_INDEX_1_ID: u32 = 4;
const MEMO_DATA_INDEX_0_ID: u32 = 5;
const MEMO_DATA_INDEX_1_ID: u32 = 6;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    // Every hook needs to import guard function
    // and use it at least once
    max_iter(1);

    let result_code = otxn_slot(OTXN_SLOT_ID).unwrap_line_number();
    if result_code != OTXN_SLOT_ID.into() {
        rollback(b"otxn_slot incorrect slot number", -1);
    }
    // Memos: [
    //       {
    //         Memo: {
    //           MemoData: "21",
    //         },
    //       },
    //       {
    //         Memo: {
    //           MemoData: "31",
    //         },
    //       },
    //     ],
    slot_subfield(OTXN_SLOT_ID, FieldId::Memos, MEMOS_SLOT_ID).unwrap_line_number();
    let memos_length = slot_count(MEMOS_SLOT_ID).unwrap_line_number();

    if memos_length != 2 {
        rollback(b"memos length incorrect", -2);
    }

    slot_subarray(MEMOS_SLOT_ID, 0, MEMOS_INDEX_0_ID).expect(b"MEMOS_INDEX_0_ID");
    slot_subarray(MEMOS_SLOT_ID, 1, MEMOS_INDEX_1_ID).expect(b"MEMOS_INDEX_1_ID");

    let a = slot_size(MEMOS_INDEX_0_ID).unwrap_line_number();
    let b = slot_size(MEMOS_INDEX_1_ID).unwrap_line_number();

    let _ = trace_num(b"memos_index_0", a.try_into().unwrap());
    let _ = trace_num(b"memos_index_1", b.try_into().unwrap());

    slot_subfield(MEMOS_INDEX_0_ID, FieldId::MemoData, MEMO_DATA_INDEX_0_ID)
        .expect(b"MEMO_INDEX_0_ID");
    slot_subfield(MEMOS_INDEX_1_ID, FieldId::MemoData, MEMO_DATA_INDEX_1_ID)
        .expect(b"MEMO_INDEX_1_ID");

    // 0x01 is the data length
    // [0x01, 0x21]
    let memo_data_0: [u8; 2] = slot(MEMO_DATA_INDEX_0_ID).expect(b"SLOT_MEMO_DATA_INDEX_0_ID");
    // [0x01, 0x31]
    let memo_data_1: [u8; 2] = slot(MEMO_DATA_INDEX_1_ID).expect(b"SLOT_MEMO_DATA_INDEX_1_ID");

    let expected_memo_data_0: [u8; 2] = [0x01, 0x21];
    let expected_memo_data_1: [u8; 2] = [0x01, 0x31];

    if !is_buffer_equal(&memo_data_0, &expected_memo_data_0) {
        rollback(b"memo_data_0 incorrect", -3);
    }

    if !is_buffer_equal(&memo_data_1, &expected_memo_data_1) {
        rollback(b"memo_data_1 incorrect", -4);
    }

    accept(b"passing", 0);
}
