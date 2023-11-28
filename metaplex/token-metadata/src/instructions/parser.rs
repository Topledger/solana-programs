extern crate bs58;

use borsh::BorshDeserialize;

use super::structs::{
    ApproveUseAuthorityArgsLayout, BurnArgsLayout, CreateArgsLayout, CreateMasterEditionArgsLayout,
    CreateMetadataAccountArgsLayout, CreateMetadataAccountArgsV2Layout,
    CreateMetadataAccountArgsV3Layout, DelegateArgsLayout, LockArgsLayout, MigrateArgsLayout,
    MintArgsLayout, MintNewEditionFromMasterEditionViaTokenArgsLayout,
    MintPrintingTokensViaTokenArgsLayout, RevokeArgsLayout, SetCollectionSizeArgsLayout,
    SetReservationListArgsLayout, TransferArgsLayout, TransferOutOfEscrowArgsLayout,
    UnlockArgsLayout, UpdateArgsLayout, UpdateMetadataAccountArgsLayout,
    UpdateMetadataAccountArgsV2Layout, UseArgsLayout, UtilizeArgsLayout, VerificationArgsLayout,
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
    pub createMetadataAccountArgsV3: CreateMetadataAccountArgsV3Layout,
    pub setCollectionSizeArgs: SetCollectionSizeArgsLayout,
    pub transferOutOfEscrowArgs: TransferOutOfEscrowArgsLayout,
    pub burnArgs: BurnArgsLayout,
    pub createArgs: CreateArgsLayout,
    pub mintArgs: MintArgsLayout,
    pub delegateArgs: DelegateArgsLayout,
    pub revokeArgs: RevokeArgsLayout,
    pub lockArgs: LockArgsLayout,
    pub unlockArgs: UnlockArgsLayout,
    pub migrateArgs: MigrateArgsLayout,
    pub transferArgs: TransferArgsLayout,
    pub updateArgs: UpdateArgsLayout,
    pub useArgs: UseArgsLayout,
    pub verificationArgs: VerificationArgsLayout,
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
        21 => {
            result.instructionType = "RevokeUseAuthority".to_string();
        }
        22 => {
            result.instructionType = "UnverifyCollection".to_string();
        }
        23 => {
            result.instructionType = "ApproveCollectionAuthority".to_string();
        }
        24 => {
            result.instructionType = "RevokeCollectionAuthority".to_string();
        }
        25 => {
            result.instructionType = "SetAndVerifyCollection".to_string();
        }
        26 => {
            result.instructionType = "FreezeDelegatedAccount".to_string();
        }
        27 => {
            result.instructionType = "ThawDelegatedAccount".to_string();
        }
        28 => {
            result.instructionType = "RemoveCreatorVerification".to_string();
        }
        29 => {
            result.instructionType = "BurnNft".to_string();
        }
        30 => {
            result.instructionType = "VerifySizedCollectionItem".to_string();
        }
        31 => {
            result.instructionType = "UnverifySizedCollectionItem".to_string();
        }
        32 => {
            result.instructionType = "SetAndVerifySizedCollectionItem".to_string();
        }
        33 => {
            result.instructionType = "CreateMetadataAccountV3".to_string();
            result.createMetadataAccountArgsV3 =
                CreateMetadataAccountArgsV3Layout::deserialize(rest_bytes).unwrap();
        }
        34 => {
            result.instructionType = "SetCollectionSize".to_string();
            result.setCollectionSizeArgs =
                SetCollectionSizeArgsLayout::deserialize(rest_bytes).unwrap();
        }
        35 => {
            result.instructionType = "SetTokenStandard".to_string();
        }
        36 => {
            result.instructionType = "BubblegumSetCollectionSize".to_string();
            result.setCollectionSizeArgs =
                SetCollectionSizeArgsLayout::deserialize(rest_bytes).unwrap();
        }
        37 => {
            result.instructionType = "BurnEditionNft".to_string();
        }
        38 => {
            result.instructionType = "CreateEscrowAccount".to_string();
        }
        39 => {
            result.instructionType = "CloseEscrowAccount".to_string();
        }
        40 => {
            result.instructionType = "TransferOutOfEscrow".to_string();
            result.transferOutOfEscrowArgs =
                TransferOutOfEscrowArgsLayout::deserialize(rest_bytes).unwrap();
        }
        41 => {
            result.instructionType = "Burn".to_string();
            result.burnArgs = BurnArgsLayout::deserialize(rest_bytes).unwrap();
        }
        42 => {
            result.instructionType = "Create".to_string();
            result.createArgs = CreateArgsLayout::deserialize(rest_bytes).unwrap();
        }
        43 => {
            result.instructionType = "Mint".to_string();
            result.mintArgs = MintArgsLayout::deserialize(rest_bytes).unwrap();
        }
        44 => {
            result.instructionType = "Delegate".to_string();
            result.delegateArgs = DelegateArgsLayout::deserialize(rest_bytes).unwrap();
        }
        45 => {
            result.instructionType = "Revoke".to_string();
            result.revokeArgs = RevokeArgsLayout::deserialize(rest_bytes).unwrap();
        }
        46 => {
            result.instructionType = "Lock".to_string();
            result.lockArgs = LockArgsLayout::deserialize(rest_bytes).unwrap();
        }
        47 => {
            result.instructionType = "Unlock".to_string();
            result.unlockArgs = UnlockArgsLayout::deserialize(rest_bytes).unwrap();
        }
        48 => {
            result.instructionType = "Migrate".to_string();
            result.migrateArgs = MigrateArgsLayout::deserialize(rest_bytes).unwrap();
        }
        49 => {
            result.instructionType = "Transfer".to_string();
            result.transferArgs = TransferArgsLayout::deserialize(rest_bytes).unwrap();
        }
        50 => {
            result.instructionType = "Update".to_string();
            result.updateArgs = UpdateArgsLayout::deserialize(rest_bytes).unwrap();
        }
        51 => {
            result.instructionType = "Use".to_string();
            result.useArgs = UseArgsLayout::deserialize(rest_bytes).unwrap();
        }
        52 => {
            result.instructionType = "Verify".to_string();
            result.verificationArgs = VerificationArgsLayout::deserialize(rest_bytes).unwrap();
        }
        53 => {
            result.instructionType = "Unverify".to_string();
            result.verificationArgs = VerificationArgsLayout::deserialize(rest_bytes).unwrap();
        }
        54 => {
            result.instructionType = "Collect".to_string();
        }
        _ => {}
    }

    return result;
}
