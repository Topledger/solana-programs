use crate::{
    instructions::{self, parser::Instruction},
    pb::sf::solana::block_meta::v1::Arg,
};
use instructions::parser::parse_instruction;

pub fn prepare_arg(instruction_data: Vec<u8>, tx_id: String) -> Arg {
    let mut arg: Arg = Arg::default();
    let mut instruction: Instruction = parse_instruction(instruction_data);

    arg.instruction_type = instruction.instructionType;

    match arg.instruction_type.as_str() {
        "CreateMetadataAccount" => {
            arg.create_metadata_account_args =
                Some(instruction.createMetadataAccountArgs.to_proto_struct());
        }
        "UpdateMetadataAccount" => {
            arg.update_metadata_account_args =
                Some(instruction.updateMetadataAccountArgs.to_proto_struct());
        }
        "DeprecatedCreateMasterEdition" => {
            arg.create_master_edition_args =
                Some(instruction.createMasterEditionArgs.to_proto_struct());
        }
        "DeprecatedMintNewEditionFromMasterEditionViaPrintingToken" => {}
        "UpdatePrimarySaleHappenedViaToken" => {}
        "DeprecatedSetReservationList" => {}
        "DeprecatedCreateReservationList" => {}
        "SignMetadata" => {}
        "DeprecatedMintPrintingTokensViaToken" => {
            arg.mint_printing_tokens_via_token_args =
                Some(instruction.mintPrintingTokensViaTokenArgs.to_proto_struct());
        }
        "DeprecatedMintPrintingTokens" => {
            arg.mint_printing_tokens_via_token_args =
                Some(instruction.mintPrintingTokensViaTokenArgs.to_proto_struct());
        }
        "CreateMasterEdition" => {
            arg.create_master_edition_args =
                Some(instruction.createMasterEditionArgs.to_proto_struct());
        }
        "MintNewEditionFromMasterEditionViaToken" => {
            arg.mint_new_edition_from_master_edition_via_token_args = Some(
                instruction
                    .mintNewEditionFromMasterEditionViaTokenArgs
                    .to_proto_struct(),
            );
        }
        "ConvertMasterEditionV1ToV2" => {}
        "MintNewEditionFromMasterEditionViaVaultProxy" => {
            arg.mint_new_edition_from_master_edition_via_token_args = Some(
                instruction
                    .mintNewEditionFromMasterEditionViaTokenArgs
                    .to_proto_struct(),
            );
        }
        "PuffMetadata" => {}
        "UpdateMetadataAccountV2" => {
            arg.update_metadata_account_args_v2 =
                Some(instruction.updateMetadataAccountArgsV2.to_proto_struct());
        }
        "CreateMetadataAccountV2" => {
            arg.create_metadata_account_args_v2 =
                Some(instruction.createMetadataAccountArgsV2.to_proto_struct());
        }
        "CreateMasterEditionV3" => {
            arg.create_master_edition_args =
                Some(instruction.createMasterEditionArgs.to_proto_struct());
        }
        "VerifyCollection" => {}
        "Utilize" => {
            arg.utilize_args = Some(instruction.utilizeArgs.to_proto_struct());
        }
        "ApproveUseAuthority" => {
            arg.approve_use_authority_args =
                Some(instruction.approveUseAuthorityArgs.to_proto_struct());
        }
        "RevokeUseAuthority" => {}
        "UnverifyCollection" => {}
        "ApproveCollectionAuthority" => {}
        "RevokeCollectionAuthority" => {}
        "SetAndVerifyCollection" => {}
        "FreezeDelegatedAccount" => {}
        "ThawDelegatedAccount" => {}
        "RemoveCreatorVerification" => {}
        "BurnNft" => {}
        "VerifySizedCollectionItem" => {}
        "UnverifySizedCollectionItem" => {}
        "SetAndVerifySizedCollectionItem" => {}
        "CreateMetadataAccountV3" => {
            arg.create_metadata_account_args_v3 =
                Some(instruction.createMetadataAccountArgsV3.to_proto_struct());
        }
        "SetCollectionSize" => {
            arg.set_collection_size_args =
                Some(instruction.setCollectionSizeArgs.to_proto_struct());
        }
        "SetTokenStandard" => {}
        "BubblegumSetCollectionSize" => {
            arg.set_collection_size_args =
                Some(instruction.setCollectionSizeArgs.to_proto_struct());
        }
        "BurnEditionNft" => {}
        "CreateEscrowAccount" => {}
        "CloseEscrowAccount" => {}
        "TransferOutOfEscrow" => {
            arg.transfer_out_of_escrow_args =
                Some(instruction.transferOutOfEscrowArgs.to_proto_struct());
        }
        "Burn" => {
            arg.burn_args = Some(instruction.burnArgs.to_proto_struct());
        }
        "Create" => {
            arg.create_args = Some(instruction.createArgs.to_proto_struct());
        }
        "Mint" => {
            arg.mint_args = Some(instruction.mintArgs.to_proto_struct());
        }
        "Delegate" => {
            arg.delegate_args = Some(instruction.delegateArgs.to_proto_struct());
        }
        "Revoke" => {
            arg.revoke_args = Some(instruction.revokeArgs.to_proto_struct());
        }
        "Lock" => {
            arg.lock_args = Some(instruction.lockArgs.to_proto_struct());
        }
        "Unlock" => {
            arg.unlock_args = Some(instruction.unlockArgs.to_proto_struct());
        }
        "Migrate" => {
            arg.migrate_args = Some(instruction.migrateArgs.to_proto_struct());
        }
        "Transfer" => {
            arg.transfer_args = Some(instruction.transferArgs.to_proto_struct());
        }
        "Update" => {
            arg.update_args = Some(instruction.updateArgs.to_proto_struct());
        }
        "Use" => {
            arg.use_args = Some(instruction.useArgs.to_proto_struct());
        }
        "Verify" => {
            arg.verification_args = Some(instruction.verificationArgs.to_proto_struct());
        }
        "Unverify" => {
            arg.verification_args = Some(instruction.verificationArgs.to_proto_struct());
        }
        "Collect" => {}
        _ => {}
    }

    return arg;
}
