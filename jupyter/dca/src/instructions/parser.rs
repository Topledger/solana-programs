extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    DepositLayout, FulfillDlmmFillLayout, FulfillFlashFillLayout, OpenDcaLayout, OpenDcaV2Layout,
    WithdrawFeesLayout, WithdrawLayout,
};

const OPEN_DCA_DISCRIMINATOR: u64 = u64::from_be_bytes([36, 65, 185, 54, 1, 210, 100, 163]);
const CLOSE_DCA_DISCRIMINATOR: u64 = u64::from_be_bytes([22, 7, 33, 98, 168, 183, 34, 243]);
const WITHDRAW_DISCRIMINATOR: u64 = u64::from_be_bytes([183, 18, 70, 156, 148, 109, 161, 34]);
const DEPOSIT_DISCRIMINATOR: u64 = u64::from_be_bytes([242, 35, 198, 137, 82, 225, 242, 182]);
const WITHDRAW_FEES_DISCRIMINATOR: u64 =
    u64::from_be_bytes([198, 212, 171, 109, 144, 215, 174, 89]);
const INITIATE_FLASH_FILL_DISCRIMINATOR: u64 =
    u64::from_be_bytes([143, 205, 3, 191, 162, 215, 245, 49]);
const FULFILL_FLASH_FILL_DISCRIMINATOR: u64 =
    u64::from_be_bytes([115, 64, 226, 78, 33, 211, 105, 162]);
const TRANSFER_DISCRIMINATOR: u64 = u64::from_be_bytes([163, 52, 200, 231, 140, 3, 69, 186]);
const END_AND_CLOSE_DISCRIMINATOR: u64 = u64::from_be_bytes([83, 125, 166, 69, 247, 252, 103, 133]);
const OPEN_DCA_V2_DISCRIMINATOR: u64 = u64::from_be_bytes([142, 119, 43, 109, 162, 52, 11, 177]);
const INITIATE_DLMM_FILL_DISCRIMINATOR: u64 =
    u64::from_be_bytes([155, 193, 80, 121, 91, 147, 254, 187]);
const FULFILL_DLMM_FILL_DISCRIMINATOR: u64 =
    u64::from_be_bytes([1, 230, 118, 251, 45, 177, 101, 187]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub openDca: OpenDcaLayout,
    pub withdraw: WithdrawLayout,
    pub deposit: DepositLayout,
    pub withdrawFees: WithdrawFeesLayout,
    pub fulfillFlashFill: FulfillFlashFillLayout,
    pub openDcaV2: OpenDcaV2Layout,
    pub fulfillDlmmFill: FulfillDlmmFillLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        OPEN_DCA_DISCRIMINATOR => {
            result.instructionType = "OpenDca".to_string();
            result.openDca = OpenDcaLayout::deserialize(rest_bytes).unwrap();
        }
        CLOSE_DCA_DISCRIMINATOR => {
            result.instructionType = "CloseDca".to_string();
        }
        WITHDRAW_DISCRIMINATOR => {
            result.instructionType = "Withdraw".to_string();
            result.withdraw = WithdrawLayout::deserialize(rest_bytes).unwrap();
        }
        DEPOSIT_DISCRIMINATOR => {
            result.instructionType = "Deposit".to_string();
            result.deposit = DepositLayout::deserialize(rest_bytes).unwrap();
        }
        WITHDRAW_FEES_DISCRIMINATOR => {
            result.instructionType = "WithdrawFees".to_string();
            result.withdrawFees = WithdrawFeesLayout::deserialize(rest_bytes).unwrap();
        }
        INITIATE_FLASH_FILL_DISCRIMINATOR => {
            result.instructionType = "InitiateFlashFill".to_string();
        }
        FULFILL_FLASH_FILL_DISCRIMINATOR => {
            result.instructionType = "FulfillFlashFill".to_string();
            result.fulfillFlashFill = FulfillFlashFillLayout::deserialize(rest_bytes).unwrap();
        }
        TRANSFER_DISCRIMINATOR => {
            result.instructionType = "Transfer".to_string();
        }
        END_AND_CLOSE_DISCRIMINATOR => {
            result.instructionType = "EndAndClose".to_string();
        }
        OPEN_DCA_V2_DISCRIMINATOR => {
            result.instructionType = "OpenDcaV2".to_string();
            result.openDcaV2 = OpenDcaV2Layout::deserialize(rest_bytes).unwrap();
        }
        INITIATE_DLMM_FILL_DISCRIMINATOR => {
            result.instructionType = "InitiateDlmmFill".to_string();
        }
        FULFILL_DLMM_FILL_DISCRIMINATOR => {
            result.instructionType = "FulfillDlmmFill".to_string();
            result.fulfillDlmmFill = FulfillDlmmFillLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
