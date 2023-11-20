#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod instructions;
mod pb;
mod prepare_arg;
mod utils;

use pb::sf::solana::block_meta::v1::{Arg, Output, TokenMetadataMeta};
use prepare_arg::prepare_arg;
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    let mut data: Vec<TokenMetadataMeta> = vec![];

    for trx in block.transactions_owned() {
        let accounts = trx.resolved_accounts_as_strings();
        if let Some(transaction) = trx.transaction {
            let msg = transaction.message.unwrap();
            let meta = trx.meta.unwrap();

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];
                let parsed_arg_data = get_arg(program, inst.data, &inst.accounts, &accounts);
                if parsed_arg_data.is_some() {
                    let mut tokenMetadataMeta: TokenMetadataMeta = TokenMetadataMeta::default();
                    tokenMetadataMeta.arg = parsed_arg_data.unwrap();
                    tokenMetadataMeta.block_date = convert_to_date(timestamp);
                    tokenMetadataMeta.block_time = timestamp;
                    tokenMetadataMeta.block_slot = slot;
                    tokenMetadataMeta.tx_id =
                        bs58::encode(&transaction.signatures[0]).into_string();
                    tokenMetadataMeta.dapp = constants::TOKEN_METADATA_PROGRAM_ADDRESS.to_string();
                    tokenMetadataMeta.instruction_index = idx as u32;
                    tokenMetadataMeta.is_inner_instruction = false;
                    tokenMetadataMeta.inner_instruction_index = 0;
                    data.push(tokenMetadataMeta);
                }
            }
        }
    }

    log::info!("{:#?}", slot);
    Ok(Output { data })
}

fn get_arg(
    program: &String,
    instruction_data: Vec<u8>,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> Option<Arg> {
    let mut result = None;

    if program
        .to_string()
        .ne(constants::TOKEN_METADATA_PROGRAM_ADDRESS)
    {
        return result;
    } else {
        result = Some(prepare_arg(instruction_data, account_indices, accounts));
        return result;
    }
}

// fn get_arg(instruction_data: Vec<u8>, account_indices: &Vec<u8>, accounts: &Vec<String>) -> Arg {
//     let mut arg: Arg = Arg::default();
//     let instruction: Instruction = parse_instruction(instruction_data);

//     match instruction.name.as_str() {
//         "CreateTree" => {
//             arg.instruction_type = String::from("CreateTree");
//             arg.max_depth = instruction.createTreeArgs.maxDepth;
//             arg.max_buffer_size = instruction.createTreeArgs.maxBufferSize;
//             arg.public = instruction.createTreeArgs.public;
//         }
//         "SetTreeDelegate" => {
//             arg.instruction_type = String::from("SetTreeDelegate");
//         }
//         "MintV1" => {
//             arg.instruction_type = String::from("MintV1");
//             let mut edition_nonce = None;
//             if instruction.mintV1Args.message.editionNonce.is_some() {
//                 edition_nonce = Some(i32::from(
//                     instruction.mintV1Args.message.editionNonce.unwrap(),
//                 ));
//             }

//             let mut collection = None;
//             if instruction.mintV1Args.message.collection.is_some() {
//                 collection = Some(Collection {
//                     verified: instruction.mintV1Args.message.collection.unwrap().verified,
//                     key: bs58::encode(instruction.mintV1Args.message.collection.unwrap().key.value)
//                         .into_string(),
//                 });
//             }

//             arg.message = Some(Message {
//                 name: instruction.mintV1Args.message.name,
//                 symbol: instruction.mintV1Args.message.symbol,
//                 uri: instruction.mintV1Args.message.uri,
//                 seller_fee_basis_points: i32::from(
//                     instruction.mintV1Args.message.sellerFeeBasisPoints,
//                 ),
//                 primary_sale_happened: instruction.mintV1Args.message.primarySaleHappened,
//                 is_mutable: instruction.mintV1Args.message.isMutable,
//                 edition_nonce: edition_nonce,
//                 token_standard: instruction
//                     .mintV1Args
//                     .message
//                     .tokenStandard
//                     .unwrap()
//                     .to_string(),
//                 collection: collection,
//             });
//         }
//         "MintToCollectionV1" => {
//             arg.instruction_type = String::from("MintToCollectionV1");
//             let mut edition_nonce = None;
//             if instruction
//                 .mintToCollectionV1Args
//                 .metadataArgs
//                 .editionNonce
//                 .is_some()
//             {
//                 edition_nonce = Some(i32::from(
//                     instruction
//                         .mintToCollectionV1Args
//                         .metadataArgs
//                         .editionNonce
//                         .unwrap(),
//                 ));
//             }

//             let mut collection = None;
//             if instruction
//                 .mintToCollectionV1Args
//                 .metadataArgs
//                 .collection
//                 .is_some()
//             {
//                 collection = Some(Collection {
//                     verified: instruction
//                         .mintToCollectionV1Args
//                         .metadataArgs
//                         .collection
//                         .unwrap()
//                         .verified,
//                     key: bs58::encode(
//                         instruction
//                             .mintToCollectionV1Args
//                             .metadataArgs
//                             .collection
//                             .unwrap()
//                             .key
//                             .value,
//                     )
//                     .into_string(),
//                 });
//             }

//             let mut creators: Vec<Creator> = vec![];
//             for creator in instruction
//                 .mintToCollectionV1Args
//                 .metadataArgs
//                 .creators
//                 .iter()
//             {
//                 creators.push(Creator {
//                     address: get_b58_string(creator.address.value),
//                     verified: creator.verified,
//                     share: i32::from(creator.share),
//                 });
//             }

//             arg.metadata_args = Some(MetadataArgs {
//                 name: instruction.mintToCollectionV1Args.metadataArgs.name,
//                 symbol: instruction.mintToCollectionV1Args.metadataArgs.symbol,
//                 uri: instruction.mintToCollectionV1Args.metadataArgs.uri,
//                 seller_fee_basis_points: i32::from(
//                     instruction
//                         .mintToCollectionV1Args
//                         .metadataArgs
//                         .sellerFeeBasisPoints,
//                 ),
//                 primary_sale_happened: instruction
//                     .mintToCollectionV1Args
//                     .metadataArgs
//                     .primarySaleHappened,
//                 is_mutable: instruction.mintToCollectionV1Args.metadataArgs.isMutable,
//                 edition_nonce: edition_nonce,
//                 token_standard: instruction
//                     .mintToCollectionV1Args
//                     .metadataArgs
//                     .tokenStandard
//                     .unwrap()
//                     .to_string(),
//                 collection: collection,
//                 uses: Some(Uses {
//                     use_method: instruction
//                         .mintToCollectionV1Args
//                         .metadataArgs
//                         .uses
//                         .unwrap_or_default()
//                         .useMethod
//                         .to_string(),
//                     remaining: instruction
//                         .mintToCollectionV1Args
//                         .metadataArgs
//                         .uses
//                         .unwrap_or_default()
//                         .remaining,
//                     total: instruction
//                         .mintToCollectionV1Args
//                         .metadataArgs
//                         .uses
//                         .unwrap_or_default()
//                         .total,
//                 }),
//                 token_program_version: instruction
//                     .mintToCollectionV1Args
//                     .metadataArgs
//                     .tokenProgramVersion
//                     .to_string(),
//                 creators: creators,
//             });
//         }
//         "VerifyCreator" => {
//             arg.instruction_type = String::from("VerifyCreator");
//             arg.root = instruction
//                 .verifyCreatorArgs
//                 .root
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.data_hash = instruction
//                 .verifyCreatorArgs
//                 .dataHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.creator_hash = instruction
//                 .verifyCreatorArgs
//                 .creatorHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.nonce = instruction.verifyCreatorArgs.nonce;
//             arg.index = instruction.verifyCreatorArgs.index;

//             let mut edition_nonce = None;
//             if instruction
//                 .mintToCollectionV1Args
//                 .metadataArgs
//                 .editionNonce
//                 .is_some()
//             {
//                 edition_nonce = Some(i32::from(
//                     instruction
//                         .mintToCollectionV1Args
//                         .metadataArgs
//                         .editionNonce
//                         .unwrap(),
//                 ));
//             }

//             let mut collection = None;
//             if instruction.verifyCreatorArgs.message.collection.is_some() {
//                 collection = Some(Collection {
//                     verified: instruction
//                         .verifyCreatorArgs
//                         .message
//                         .collection
//                         .unwrap()
//                         .verified,
//                     key: bs58::encode(
//                         instruction
//                             .verifyCreatorArgs
//                             .message
//                             .collection
//                             .unwrap()
//                             .key
//                             .value,
//                     )
//                     .into_string(),
//                 });
//             }

//             arg.message = Some(Message {
//                 name: instruction.verifyCreatorArgs.message.name,
//                 symbol: instruction.verifyCreatorArgs.message.symbol,
//                 uri: instruction.verifyCreatorArgs.message.uri,
//                 seller_fee_basis_points: i32::from(
//                     instruction.verifyCreatorArgs.message.sellerFeeBasisPoints,
//                 ),
//                 primary_sale_happened: instruction.verifyCreatorArgs.message.primarySaleHappened,
//                 is_mutable: instruction.verifyCreatorArgs.message.isMutable,
//                 edition_nonce: edition_nonce,
//                 token_standard: instruction
//                     .verifyCreatorArgs
//                     .message
//                     .tokenStandard
//                     .unwrap()
//                     .to_string(),
//                 collection: collection,
//             });
//         }
//         "UnverifyCreator" => {
//             arg.instruction_type = String::from("UnverifyCreator");
//             arg.root = instruction
//                 .unverifyCreatorArgs
//                 .root
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.data_hash = instruction
//                 .unverifyCreatorArgs
//                 .dataHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.creator_hash = instruction
//                 .unverifyCreatorArgs
//                 .creatorHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.nonce = instruction.unverifyCreatorArgs.nonce;
//             arg.index = instruction.unverifyCreatorArgs.index;

//             let mut edition_nonce = None;
//             if instruction
//                 .unverifyCreatorArgs
//                 .message
//                 .editionNonce
//                 .is_some()
//             {
//                 edition_nonce = Some(i32::from(
//                     instruction
//                         .unverifyCreatorArgs
//                         .message
//                         .editionNonce
//                         .unwrap(),
//                 ));
//             }

//             let mut collection = None;
//             if instruction.unverifyCreatorArgs.message.collection.is_some() {
//                 collection = Some(Collection {
//                     verified: instruction
//                         .unverifyCreatorArgs
//                         .message
//                         .collection
//                         .unwrap()
//                         .verified,
//                     key: bs58::encode(
//                         instruction
//                             .unverifyCreatorArgs
//                             .message
//                             .collection
//                             .unwrap()
//                             .key
//                             .value,
//                     )
//                     .into_string(),
//                 });
//             }

//             arg.message = Some(Message {
//                 name: instruction.unverifyCreatorArgs.message.name,
//                 symbol: instruction.unverifyCreatorArgs.message.symbol,
//                 uri: instruction.unverifyCreatorArgs.message.uri,
//                 seller_fee_basis_points: i32::from(
//                     instruction.unverifyCreatorArgs.message.sellerFeeBasisPoints,
//                 ),
//                 primary_sale_happened: instruction.unverifyCreatorArgs.message.primarySaleHappened,
//                 is_mutable: instruction.unverifyCreatorArgs.message.isMutable,
//                 edition_nonce: edition_nonce,
//                 token_standard: instruction
//                     .unverifyCreatorArgs
//                     .message
//                     .tokenStandard
//                     .unwrap()
//                     .to_string(),
//                 collection: collection,
//             });
//         }
//         "VerifyCollection" => {
//             arg.instruction_type = String::from("VerifyCollection");
//             arg.root = instruction
//                 .verifyCollectionArgs
//                 .root
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.data_hash = instruction
//                 .verifyCollectionArgs
//                 .dataHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.creator_hash = instruction
//                 .verifyCollectionArgs
//                 .creatorHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.nonce = instruction.verifyCollectionArgs.nonce;
//             arg.index = instruction.verifyCollectionArgs.index;

//             let mut edition_nonce = None;
//             if instruction
//                 .verifyCollectionArgs
//                 .message
//                 .editionNonce
//                 .is_some()
//             {
//                 edition_nonce = Some(i32::from(
//                     instruction
//                         .verifyCollectionArgs
//                         .message
//                         .editionNonce
//                         .unwrap(),
//                 ));
//             }

//             let mut collection = None;
//             if instruction
//                 .verifyCollectionArgs
//                 .message
//                 .collection
//                 .is_some()
//             {
//                 collection = Some(Collection {
//                     verified: instruction
//                         .verifyCollectionArgs
//                         .message
//                         .collection
//                         .unwrap()
//                         .verified,
//                     key: bs58::encode(
//                         instruction
//                             .verifyCollectionArgs
//                             .message
//                             .collection
//                             .unwrap()
//                             .key
//                             .value,
//                     )
//                     .into_string(),
//                 });
//             }

//             arg.message = Some(Message {
//                 name: instruction.verifyCollectionArgs.message.name,
//                 symbol: instruction.verifyCollectionArgs.message.symbol,
//                 uri: instruction.verifyCollectionArgs.message.uri,
//                 seller_fee_basis_points: i32::from(
//                     instruction
//                         .verifyCollectionArgs
//                         .message
//                         .sellerFeeBasisPoints,
//                 ),
//                 primary_sale_happened: instruction.verifyCollectionArgs.message.primarySaleHappened,
//                 is_mutable: instruction.verifyCollectionArgs.message.isMutable,
//                 edition_nonce: edition_nonce,
//                 token_standard: instruction
//                     .verifyCollectionArgs
//                     .message
//                     .tokenStandard
//                     .unwrap()
//                     .to_string(),
//                 collection: collection,
//             });
//         }
//         "UnverifyCollection" => {
//             arg.instruction_type = String::from("UnverifyCollection");
//             arg.root = instruction
//                 .unverifyCollectionArgs
//                 .root
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.data_hash = instruction
//                 .unverifyCollectionArgs
//                 .dataHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.creator_hash = instruction
//                 .unverifyCollectionArgs
//                 .creatorHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.nonce = instruction.unverifyCollectionArgs.nonce;
//             arg.index = instruction.unverifyCollectionArgs.index;

//             let mut edition_nonce = None;
//             if instruction
//                 .unverifyCollectionArgs
//                 .message
//                 .editionNonce
//                 .is_some()
//             {
//                 edition_nonce = Some(i32::from(
//                     instruction
//                         .unverifyCollectionArgs
//                         .message
//                         .editionNonce
//                         .unwrap(),
//                 ));
//             }

//             let mut collection = None;
//             if instruction
//                 .unverifyCollectionArgs
//                 .message
//                 .collection
//                 .is_some()
//             {
//                 collection = Some(Collection {
//                     verified: instruction
//                         .unverifyCollectionArgs
//                         .message
//                         .collection
//                         .unwrap()
//                         .verified,
//                     key: bs58::encode(
//                         instruction
//                             .unverifyCollectionArgs
//                             .message
//                             .collection
//                             .unwrap()
//                             .key
//                             .value,
//                     )
//                     .into_string(),
//                 });
//             }

//             arg.message = Some(Message {
//                 name: instruction.unverifyCollectionArgs.message.name,
//                 symbol: instruction.unverifyCollectionArgs.message.symbol,
//                 uri: instruction.unverifyCollectionArgs.message.uri,
//                 seller_fee_basis_points: i32::from(
//                     instruction
//                         .unverifyCollectionArgs
//                         .message
//                         .sellerFeeBasisPoints,
//                 ),
//                 primary_sale_happened: instruction
//                     .unverifyCollectionArgs
//                     .message
//                     .primarySaleHappened,
//                 is_mutable: instruction.unverifyCollectionArgs.message.isMutable,
//                 edition_nonce: edition_nonce,
//                 token_standard: instruction
//                     .unverifyCollectionArgs
//                     .message
//                     .tokenStandard
//                     .unwrap()
//                     .to_string(),
//                 collection: collection,
//             });
//         }
//         "SetAndVerifyCollection" => {
//             arg.instruction_type = String::from("SetAndVerifyCollection");
//             arg.root = instruction
//                 .setAndVerifyCollectionArgs
//                 .root
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.data_hash = instruction
//                 .setAndVerifyCollectionArgs
//                 .dataHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.creator_hash = instruction
//                 .setAndVerifyCollectionArgs
//                 .creatorHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.nonce = instruction.setAndVerifyCollectionArgs.nonce;
//             arg.index = instruction.setAndVerifyCollectionArgs.index;

//             let mut edition_nonce = None;
//             if instruction
//                 .setAndVerifyCollectionArgs
//                 .message
//                 .editionNonce
//                 .is_some()
//             {
//                 edition_nonce = Some(i32::from(
//                     instruction
//                         .setAndVerifyCollectionArgs
//                         .message
//                         .editionNonce
//                         .unwrap(),
//                 ));
//             }

//             let mut collection = None;
//             if instruction
//                 .setAndVerifyCollectionArgs
//                 .message
//                 .collection
//                 .is_some()
//             {
//                 collection = Some(Collection {
//                     verified: instruction
//                         .setAndVerifyCollectionArgs
//                         .message
//                         .collection
//                         .unwrap()
//                         .verified,
//                     key: bs58::encode(
//                         instruction
//                             .setAndVerifyCollectionArgs
//                             .message
//                             .collection
//                             .unwrap()
//                             .key
//                             .value,
//                     )
//                     .into_string(),
//                 });
//             }

//             arg.message = Some(Message {
//                 name: instruction.setAndVerifyCollectionArgs.message.name,
//                 symbol: instruction.unverifyCollectionArgs.message.symbol,
//                 uri: instruction.unverifyCollectionArgs.message.uri,
//                 seller_fee_basis_points: i32::from(
//                     instruction
//                         .unverifyCollectionArgs
//                         .message
//                         .sellerFeeBasisPoints,
//                 ),
//                 primary_sale_happened: instruction
//                     .unverifyCollectionArgs
//                     .message
//                     .primarySaleHappened,
//                 is_mutable: instruction.unverifyCollectionArgs.message.isMutable,
//                 edition_nonce: edition_nonce,
//                 token_standard: instruction
//                     .unverifyCollectionArgs
//                     .message
//                     .tokenStandard
//                     .unwrap()
//                     .to_string(),
//                 collection: collection,
//             });

//             arg.collection =
//                 get_b58_string(instruction.setAndVerifyCollectionArgs.collection.value);
//         }
//         "Transfer" => {
//             arg.instruction_type = String::from("Transfer");
//             arg.root = instruction.transferArgs.root.map(|x| i32::from(x)).to_vec();
//             arg.data_hash = instruction
//                 .transferArgs
//                 .dataHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.creator_hash = instruction
//                 .transferArgs
//                 .creatorHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.nonce = instruction.transferArgs.nonce;
//             arg.index = instruction.transferArgs.index;
//         }
//         "Delegate" => {
//             arg.instruction_type = String::from("Delegate");
//             arg.root = instruction.delegateArgs.root.map(|x| i32::from(x)).to_vec();
//             arg.data_hash = instruction
//                 .delegateArgs
//                 .dataHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.creator_hash = instruction
//                 .delegateArgs
//                 .creatorHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.nonce = instruction.delegateArgs.nonce;
//             arg.index = instruction.delegateArgs.index;
//         }
//         "Burn" => {
//             arg.instruction_type = String::from("Burn");
//             arg.root = instruction.burnArgs.root.map(|x| i32::from(x)).to_vec();
//             arg.data_hash = instruction.burnArgs.dataHash.map(|x| i32::from(x)).to_vec();
//             arg.creator_hash = instruction
//                 .burnArgs
//                 .creatorHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.nonce = instruction.burnArgs.nonce;
//             arg.index = instruction.burnArgs.index;
//         }
//         "Redeem" => {
//             arg.instruction_type = String::from("Redeem");
//             arg.root = instruction.burnArgs.root.map(|x| i32::from(x)).to_vec();
//             arg.data_hash = instruction.burnArgs.dataHash.map(|x| i32::from(x)).to_vec();
//             arg.creator_hash = instruction
//                 .burnArgs
//                 .creatorHash
//                 .map(|x| i32::from(x))
//                 .to_vec();
//             arg.nonce = instruction.burnArgs.nonce;
//             arg.index = instruction.burnArgs.index;
//         }
//         "CancelRedeem" => {
//             arg.instruction_type = String::from("CancelRedeem");
//             arg.root = instruction
//                 .cancelRedeemArgs
//                 .root
//                 .map(|x| i32::from(x))
//                 .to_vec();
//         }
//         "DecompressV1" => {
//             arg.instruction_type = String::from("DecompressV1");

//             let mut edition_nonce = None;
//             if instruction.decompressV1Args.metadata.editionNonce.is_some() {
//                 edition_nonce = Some(i32::from(
//                     instruction.decompressV1Args.metadata.editionNonce.unwrap(),
//                 ));
//             }

//             let mut collection = None;
//             if instruction.decompressV1Args.metadata.collection.is_some() {
//                 collection = Some(Collection {
//                     verified: instruction
//                         .decompressV1Args
//                         .metadata
//                         .collection
//                         .unwrap()
//                         .verified,
//                     key: get_b58_string(
//                         instruction
//                             .decompressV1Args
//                             .metadata
//                             .collection
//                             .unwrap()
//                             .key
//                             .value,
//                     ),
//                 });
//             }

//             let mut creators: Vec<Creator> = vec![];
//             for creator in instruction.decompressV1Args.metadata.creators.iter() {
//                 creators.push(Creator {
//                     address: get_b58_string(creator.address.value),
//                     verified: creator.verified,
//                     share: i32::from(creator.share),
//                 });
//             }

//             arg.metadata = Some(Metadata {
//                 name: instruction.decompressV1Args.metadata.name,
//                 symbol: instruction.decompressV1Args.metadata.symbol,
//                 uri: instruction.decompressV1Args.metadata.uri,
//                 seller_fee_basis_points: i32::from(
//                     instruction.decompressV1Args.metadata.sellerFeeBasisPoints,
//                 ),
//                 primary_sale_happened: instruction.decompressV1Args.metadata.primarySaleHappened,
//                 is_mutable: instruction.decompressV1Args.metadata.isMutable,
//                 edition_nonce: edition_nonce,
//                 token_standard: instruction
//                     .decompressV1Args
//                     .metadata
//                     .tokenStandard
//                     .unwrap()
//                     .to_string(),
//                 collection: collection,
//                 uses: Some(Uses {
//                     use_method: instruction
//                         .decompressV1Args
//                         .metadata
//                         .uses
//                         .unwrap()
//                         .useMethod
//                         .to_string(),
//                     remaining: instruction
//                         .decompressV1Args
//                         .metadata
//                         .uses
//                         .unwrap()
//                         .remaining,
//                     total: instruction.decompressV1Args.metadata.uses.unwrap().total,
//                 }),
//                 token_program_version: instruction
//                     .decompressV1Args
//                     .metadata
//                     .tokenProgramVersion
//                     .to_string(),
//                 creators: creators,
//             });
//         }
//         "Compress" => {
//             arg.instruction_type = String::from("Compress");
//         }
//         _ => {
//             arg.instruction_type = String::from("Unknown Instruction");
//         }
//     }

//     return arg;
// }

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
