extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    CloseMarkerV0Layout, ExecuteTransactionV0Layout, InitializeLazyTransactionsV0Layout,
    SetCanopyV0Layout, UpdateLazyTransactionsV0Layout,
};

const INITIALIZE_LAZY_TRANSACTIONS_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([32, 227, 207, 127, 112, 50, 31, 157]);
const EXECUTE_TRANSACTION_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([217, 93, 174, 97, 130, 183, 126, 44]);
const CLOSE_MARKER_V0_DISCRIMINATOR: u64 = u64::from_be_bytes([202, 88, 149, 144, 81, 215, 1, 175]);
const CLOSE_CANOPY_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([92, 189, 176, 245, 129, 173, 166, 169]);
const UPDATE_LAZY_TRANSACTIONS_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([56, 223, 165, 245, 150, 236, 173, 37]);
const SET_CANOPY_V0_DISCRIMINATOR: u64 = u64::from_be_bytes([25, 86, 129, 124, 184, 195, 134, 89]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initializeLazyTransactionsV0: InitializeLazyTransactionsV0Layout,
    pub executeTransactionV0: ExecuteTransactionV0Layout,
    pub closeMarkerV0: CloseMarkerV0Layout,
    pub updateLazyTransactionsV0: UpdateLazyTransactionsV0Layout,
    pub setCanopyV0: SetCanopyV0Layout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_LAZY_TRANSACTIONS_V0_DISCRIMINATOR => {
            result.instructionType = "InitializeLazyTransactionsV0".to_string();
            result.initializeLazyTransactionsV0 =
                InitializeLazyTransactionsV0Layout::deserialize(rest_bytes).unwrap();
        }
        EXECUTE_TRANSACTION_V0_DISCRIMINATOR => {
            result.instructionType = "ExecuteTransactionV0".to_string();
            result.executeTransactionV0 =
                ExecuteTransactionV0Layout::deserialize(rest_bytes).unwrap();
        }
        CLOSE_MARKER_V0_DISCRIMINATOR => {
            result.instructionType = "CloseMarkerV0".to_string();
            result.closeMarkerV0 = CloseMarkerV0Layout::deserialize(rest_bytes).unwrap();
        }
        CLOSE_CANOPY_V0_DISCRIMINATOR => {
            result.instructionType = "CloseCanopyV0".to_string();
        }
        UPDATE_LAZY_TRANSACTIONS_V0_DISCRIMINATOR => {
            result.instructionType = "UpdateLazyTransactionsV0".to_string();
            result.updateLazyTransactionsV0 =
                UpdateLazyTransactionsV0Layout::deserialize(rest_bytes).unwrap();
        }
        SET_CANOPY_V0_DISCRIMINATOR => {
            result.instructionType = "SetCanopyV0".to_string();
            result.setCanopyV0 = SetCanopyV0Layout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
