extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    CreateLayout, DeleteLayout, ExtendLayout, ImmutableOwnerLayout, ResizeLayout, TransferLayout,
    UpdateLayout,
};
const CREATE_DISCRIMINATOR: u64 = u64::from_be_bytes([24, 30, 200, 40, 5, 28, 7, 119]);
const UPDATE_DISCRIMINATOR: u64 = u64::from_be_bytes([219, 200, 88, 176, 158, 63, 253, 127]);
const TRANSFER_DISCRIMINATOR: u64 = u64::from_be_bytes([163, 52, 200, 231, 140, 3, 69, 186]);
const DELETE_DISCRIMINATOR: u64 = u64::from_be_bytes([165, 204, 60, 98, 134, 15, 83, 134]);
const RESIZE_DISCRIMINATOR: u64 = u64::from_be_bytes([74, 27, 74, 155, 56, 134, 175, 125]);
const EXTEND_DISCRIMINATOR: u64 = u64::from_be_bytes([228, 127, 0, 1, 227, 154, 54, 168]);
const IMMUTABLE_OWNER_DISCRIMINATOR: u64 =
    u64::from_be_bytes([203, 139, 201, 92, 25, 75, 195, 226]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub create: CreateLayout,
    pub update: UpdateLayout,
    pub transfer: TransferLayout,
    pub delete: DeleteLayout,
    pub resize: ResizeLayout,
    pub extend: ExtendLayout,
    pub immutableOwner: ImmutableOwnerLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        CREATE_DISCRIMINATOR => {
            result.instructionType = "Create".to_string();
            result.create = CreateLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        UPDATE_DISCRIMINATOR => {
            result.instructionType = "Update".to_string();
            result.update = UpdateLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        TRANSFER_DISCRIMINATOR => {
            result.instructionType = "Transfer".to_string();
            result.transfer = TransferLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        DELETE_DISCRIMINATOR => {
            result.instructionType = "Delete".to_string();
            result.delete = DeleteLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        RESIZE_DISCRIMINATOR => {
            result.instructionType = "Resize".to_string();
            result.resize = ResizeLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        EXTEND_DISCRIMINATOR => {
            result.instructionType = "Extend".to_string();
            result.extend = ExtendLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        IMMUTABLE_OWNER_DISCRIMINATOR => {
            result.instructionType = "ImmutableOwner".to_string();
            result.immutableOwner =
                ImmutableOwnerLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        _ => {}
    }

    return result;
}
