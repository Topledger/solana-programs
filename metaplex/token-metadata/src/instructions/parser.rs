extern crate bs58;

use borsh::BorshDeserialize;

use super::structs::{
    ApproveUseAuthorityArgsLayout, CreateMasterEditionArgsLayout, CreateMetadataAccountArgsLayout,
    CreateMetadataAccountArgsV2Layout, MintNewEditionFromMasterEditionViaTokenArgsLayout,
    MintPrintingTokensViaTokenArgsLayout, SetReservationListArgsLayout,
    UpdateMetadataAccountArgsLayout, UpdateMetadataAccountArgsV2Layout, UtilizeArgsLayout,
};

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub createMetadataAccountArgs: CreateMetadataAccountArgsLayout,
    pub updateMetadataAccountArgs: UpdateMetadataAccountArgsLayout,
    pub createMasterEditionArgs: CreateMasterEditionArgsLayout,
    pub setReservationListArgs: SetReservationListArgsLayout,
    pub mintPrintingTokensViaTokenArgs: MintPrintingTokensViaTokenArgsLayout,
    pub mintNewEditionFromMasterEditionViaTokenArgs:
        MintNewEditionFromMasterEditionViaTokenArgsLayout,
    pub updateMetadataAccountArgsV2: UpdateMetadataAccountArgsV2Layout,
    pub createMetadataAccountArgsV2: CreateMetadataAccountArgsV2Layout,
    pub utilizeArgs: UtilizeArgsLayout,
    pub approveUseAuthorityArgs: ApproveUseAuthorityArgsLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);
    let rest_bytes = &mut rest.clone();

    match discriminator {
        0 => {
            result.instructionType = "CreateMetadataAccount".to_string();
            result.createMetadataAccountArgs =
                CreateMetadataAccountArgsLayout::deserialize(rest_bytes).unwrap();
        }
        1 => {
            result.instructionType = "UpdateMetadataAccount".to_string();
            result.updateMetadataAccountArgs =
                UpdateMetadataAccountArgsLayout::deserialize(rest_bytes).unwrap();
        }
        2 => {
            result.instructionType = "DeprecatedCreateMasterEdition".to_string();
            result.createMasterEditionArgs =
                CreateMasterEditionArgsLayout::deserialize(rest_bytes).unwrap();
        }
        3 => {
            result.instructionType =
                "DeprecatedMintNewEditionFromMasterEditionViaPrintingToken".to_string();
        }
        4 => {
            result.instructionType = "UpdatePrimarySaleHappenedViaToken".to_string();
        }
        5 => {
            result.instructionType = "DeprecatedSetReservationList".to_string();
            result.setReservationListArgs =
                SetReservationListArgsLayout::deserialize(rest_bytes).unwrap();
        }
        6 => {
            result.instructionType = "DeprecatedCreateReservationList".to_string();
        }
        7 => {
            result.instructionType = "SignMetadata".to_string();
        }
        8 => {
            result.instructionType = "DeprecatedMintPrintingTokensViaToken".to_string();
            result.mintPrintingTokensViaTokenArgs =
                MintPrintingTokensViaTokenArgsLayout::deserialize(rest_bytes).unwrap();
        }
        9 => {
            result.instructionType = "DeprecatedMintPrintingTokens".to_string();
            result.mintPrintingTokensViaTokenArgs =
                MintPrintingTokensViaTokenArgsLayout::deserialize(rest_bytes).unwrap();
        }
        10 => {
            result.instructionType = "CreateMasterEdition".to_string();
            result.createMasterEditionArgs =
                CreateMasterEditionArgsLayout::deserialize(rest_bytes).unwrap();
        }
        11 => {
            result.instructionType = "MintNewEditionFromMasterEditionViaToken".to_string();
            result.mintNewEditionFromMasterEditionViaTokenArgs =
                MintNewEditionFromMasterEditionViaTokenArgsLayout::deserialize(rest_bytes).unwrap();
        }
        12 => {
            result.instructionType = "ConvertMasterEditionV1ToV2".to_string();
        }
        13 => {
            result.instructionType = "MintNewEditionFromMasterEditionViaVaultProxy".to_string();
            result.mintNewEditionFromMasterEditionViaTokenArgs =
                MintNewEditionFromMasterEditionViaTokenArgsLayout::deserialize(rest_bytes).unwrap();
        }
        14 => {
            result.instructionType = "PuffMetadata".to_string();
        }
        15 => {
            result.instructionType = "UpdateMetadataAccountV2".to_string();
            result.updateMetadataAccountArgsV2 =
                UpdateMetadataAccountArgsV2Layout::deserialize(rest_bytes).unwrap();
        }
        16 => {
            result.instructionType = "CreateMetadataAccountV2".to_string();
            result.createMetadataAccountArgsV2 =
                CreateMetadataAccountArgsV2Layout::deserialize(rest_bytes).unwrap();
        }
        17 => {
            result.instructionType = "CreateMasterEditionV3".to_string();
            result.createMasterEditionArgs =
                CreateMasterEditionArgsLayout::deserialize(rest_bytes).unwrap();
        }
        18 => {
            result.instructionType = "VerifyCollection".to_string();
        }
        19 => {
            result.instructionType = "Utilize".to_string();
            result.utilizeArgs = UtilizeArgsLayout::deserialize(rest_bytes).unwrap();
        }
        20 => {
            result.instructionType = "ApproveUseAuthority".to_string();
            result.approveUseAuthorityArgs =
                ApproveUseAuthorityArgsLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
