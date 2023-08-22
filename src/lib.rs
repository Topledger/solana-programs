#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod utils;
mod pb;

use pb::sf::solana::block_meta::v1::{
    Arg, BubblegumMeta, Collection, Creator, Message, MetadataArgs, Uses, Metadata, Output,
};

use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

mod instruction;
use instruction::{parse, Instruction};
use utils::convert_to_date;
use substreams::store::{StoreGet, StoreGetArray};

#[substreams::handlers::map]
fn map_block(block: Block, address_lookup_table_store: StoreGetArray<String>) -> Result<Output, substreams::errors::Error> {
    // log::info!("{:#?}", block);

    let mut data: Vec<BubblegumMeta> = vec![];

    for trx in block
        .clone().transactions_owned()
        .filter(|txn| txn.meta().is_some())
    {
        if let Some(transaction) = trx.transaction {

            let msg = transaction.message.unwrap();
            let mut accounts = vec![];
            let mut writable_accounts = vec![];
            let mut readable_accounts = vec![];
            msg.account_keys
                .into_iter()
                .for_each(|addr| accounts.push(bs58::encode(addr).into_string()));
            msg.address_table_lookups.into_iter().for_each(|addr| {
                let acc = bs58::encode(&addr.account_key).into_string();
                match address_lookup_table_store.get_last(format!("table:{acc}")) {
                    None => panic!("Address Lookup Table Account {} does not exist", acc),
                    Some(accs) => {
                        addr.writable_indexes.into_iter().for_each(|idx| {
                            writable_accounts.push(accs[idx as usize].clone());
                        });
                        addr.readonly_indexes.into_iter().for_each(|idx| {
                            readable_accounts.push(accs[idx as usize].clone());
                        })
                    }
                }
            });

            accounts.append(&mut writable_accounts);
            accounts.append(&mut readable_accounts);



            
            for (idx, inst) in msg.instructions.iter().enumerate() {
                
                let program = &accounts[inst.program_id_index as usize];
                
                
                if program != constants::BUBBLEGUM_PROGRAM_ADDRESS {
                    continue;
                }
                
                data.push(BubblegumMeta {
                    block_date: convert_to_date(block.block_time.as_ref().unwrap().timestamp),
                    block_time: block.block_time.as_ref().unwrap().timestamp,
                    tx_id: bs58::encode(&transaction.signatures[0]).into_string(),
                    dapp: constants::BUBBLEGUM_PROGRAM_ADDRESS.to_string(),
                    block_slot: block.parent_slot + 1,
                    instruction_index: inst.program_id_index,
                    is_inner_instruction: false,
                    inner_instruction_index: 0,
                    arg: Some(get_arg(inst.clone().data))
                });

                let meta = trx.meta.as_ref().unwrap();
                meta.inner_instructions
                    .iter()
                    .filter(|&inner_instruction| inner_instruction.index == idx as u32)
                    .for_each(|inner_instruction| {
                        inner_instruction.instructions
                        .iter()
                        .for_each(|inner_inst: &substreams_solana::pb::sf::solana::r#type::v1::InnerInstruction| {
                            let program = &accounts[inner_inst.program_id_index as usize];
                            if program == constants::BUBBLEGUM_PROGRAM_ADDRESS {
                                data.push(BubblegumMeta {
                                    block_date: convert_to_date(block.block_time.as_ref().unwrap().timestamp),
                                    block_time: block.block_time.as_ref().unwrap().timestamp,
                                    tx_id: bs58::encode(&transaction.signatures[0]).into_string(),
                                    dapp: constants::BUBBLEGUM_PROGRAM_ADDRESS.to_string(),
                                    block_slot: block.parent_slot + 1,
                                    instruction_index: inst.program_id_index,
                                    is_inner_instruction: true,
                                    inner_instruction_index: inner_inst.program_id_index,
                                    arg: Some(get_arg(inner_inst.clone().data))
                                });
                            }
                        })
                    });
            }
            
        }
    }

    log::info!("{:#?}", block.slot);
    log::info!("{:#?}", data.len());
    Ok(Output{data})
}

fn get_arg(instruction_data: Vec<u8>) -> Arg {
    let mut arg: Arg = Arg::default();
    let instruction: Instruction = parse(instruction_data);


    match instruction.name.as_str() {
        "CreateTree" => {
            arg.instruction_type = String::from("CreateTree");
            arg.max_depth = instruction.createTreeArgs.maxDepth;
            arg.max_buffer_size = instruction.createTreeArgs.maxBufferSize;
            arg.public = instruction.createTreeArgs.public;
        }
        "SetTreeDelegate" => {
            arg.instruction_type = String::from("SetTreeDelegate");
        }
        "MintV1" => {
            arg.instruction_type = String::from("MintV1");
            let mut edition_nonce = None;
            if instruction.mintV1Args.message.editionNonce.is_some() {
                edition_nonce = Some(i32::from(
                    instruction.mintV1Args.message.editionNonce.unwrap(),
                ));
            }

            let mut collection = None;
            if instruction.mintV1Args.message.collection.is_some() {
                collection = Some(Collection {
                    verified: instruction.mintV1Args.message.collection.unwrap().verified,
                    key: bs58::encode(instruction.mintV1Args.message.collection.unwrap().key.value)
                        .into_string(),
                });
            }

            arg.message = Some(Message {
                name: instruction.mintV1Args.message.name,
                symbol: instruction.mintV1Args.message.symbol,
                uri: instruction.mintV1Args.message.uri,
                seller_fee_basis_points: i32::from(
                    instruction.mintV1Args.message.sellerFeeBasisPoints,
                ),
                primary_sale_happened: instruction.mintV1Args.message.primarySaleHappened,
                is_mutable: instruction.mintV1Args.message.isMutable,
                edition_nonce: edition_nonce,
                token_standard: instruction
                    .mintV1Args
                    .message
                    .tokenStandard
                    .unwrap()
                    .to_string(),
                collection: collection,
            });
        }
        "MintToCollectionV1" => {
            arg.instruction_type = String::from("MintToCollectionV1");
            let mut edition_nonce = None;
            if instruction
                .mintToCollectionV1Args
                .metadataArgs
                .editionNonce
                .is_some()
            {
                edition_nonce = Some(i32::from(
                    instruction
                        .mintToCollectionV1Args
                        .metadataArgs
                        .editionNonce
                        .unwrap(),
                ));
            }

            let mut collection = None;
            if instruction
                .mintToCollectionV1Args
                .metadataArgs
                .collection
                .is_some()
            {
                collection = Some(Collection {
                    verified: instruction
                        .mintToCollectionV1Args
                        .metadataArgs
                        .collection
                        .unwrap()
                        .verified,
                    key: bs58::encode(
                        instruction
                            .mintToCollectionV1Args
                            .metadataArgs
                            .collection
                            .unwrap()
                            .key
                            .value,
                    )
                    .into_string(),
                });
            }

            let mut creators: Vec<Creator> = vec![];
            for creator in instruction
                .mintToCollectionV1Args
                .metadataArgs
                .creators
                .iter()
            {
                creators.push(Creator {
                    address: get_b58_string(creator.address.value),
                    verified: creator.verified,
                    share: i32::from(creator.share),
                });
            }

            arg.metadata_args = Some(MetadataArgs {
                name: instruction.mintToCollectionV1Args.metadataArgs.name,
                symbol: instruction.mintToCollectionV1Args.metadataArgs.symbol,
                uri: instruction.mintToCollectionV1Args.metadataArgs.uri,
                seller_fee_basis_points: i32::from(
                    instruction
                        .mintToCollectionV1Args
                        .metadataArgs
                        .sellerFeeBasisPoints,
                ),
                primary_sale_happened: instruction
                    .mintToCollectionV1Args
                    .metadataArgs
                    .primarySaleHappened,
                is_mutable: instruction.mintToCollectionV1Args.metadataArgs.isMutable,
                edition_nonce: edition_nonce,
                token_standard: instruction
                    .mintToCollectionV1Args
                    .metadataArgs
                    .tokenStandard
                    .unwrap()
                    .to_string(),
                collection: collection,
                uses: Some(Uses {
                    use_method: instruction.mintToCollectionV1Args.metadataArgs.uses.unwrap_or_default().useMethod.to_string(),
                    remaining: instruction
                        .mintToCollectionV1Args
                        .metadataArgs
                        .uses
                        .unwrap_or_default()
                        .remaining,
                    total: instruction
                        .mintToCollectionV1Args
                        .metadataArgs
                        .uses
                        .unwrap_or_default()
                        .total,
                }),
                token_program_version: instruction
                    .mintToCollectionV1Args
                    .metadataArgs
                    .tokenProgramVersion
                    .to_string(),
                creators: creators,
            });
        }
        "VerifyCreator" => {
            arg.instruction_type = String::from("VerifyCreator");
            arg.root = instruction
                .verifyCreatorArgs
                .root
                .map(|x| i32::from(x))
                .to_vec();
            arg.data_hash = instruction
                .verifyCreatorArgs
                .dataHash
                .map(|x| i32::from(x))
                .to_vec();
            arg.creator_hash = instruction
                .verifyCreatorArgs
                .creatorHash
                .map(|x| i32::from(x))
                .to_vec();
            arg.nonce = instruction.verifyCreatorArgs.nonce;
            arg.index = instruction.verifyCreatorArgs.index;

            let mut edition_nonce = None;
            if instruction
                .mintToCollectionV1Args
                .metadataArgs
                .editionNonce
                .is_some()
            {
                edition_nonce = Some(i32::from(
                    instruction
                        .mintToCollectionV1Args
                        .metadataArgs
                        .editionNonce
                        .unwrap(),
                ));
            }

            let mut collection = None;
            if instruction.verifyCreatorArgs.message.collection.is_some() {
                collection = Some(Collection {
                    verified: instruction
                        .verifyCreatorArgs
                        .message
                        .collection
                        .unwrap()
                        .verified,
                    key: bs58::encode(
                        instruction
                            .verifyCreatorArgs
                            .message
                            .collection
                            .unwrap()
                            .key
                            .value,
                    )
                    .into_string(),
                });
            }

            arg.message = Some(Message {
                name: instruction.verifyCreatorArgs.message.name,
                symbol: instruction.verifyCreatorArgs.message.symbol,
                uri: instruction.verifyCreatorArgs.message.uri,
                seller_fee_basis_points: i32::from(
                    instruction.verifyCreatorArgs.message.sellerFeeBasisPoints,
                ),
                primary_sale_happened: instruction.verifyCreatorArgs.message.primarySaleHappened,
                is_mutable: instruction.verifyCreatorArgs.message.isMutable,
                edition_nonce: edition_nonce,
                token_standard: instruction
                    .verifyCreatorArgs
                    .message
                    .tokenStandard
                    .unwrap()
                    .to_string(),
                collection: collection,
            });
        }
        "UnverifyCreator" => {
            arg.instruction_type = String::from("UnverifyCreator");
            arg.root = instruction
                .unverifyCreatorArgs
                .root
                .map(|x| i32::from(x))
                .to_vec();
            arg.data_hash = instruction
                .unverifyCreatorArgs
                .dataHash
                .map(|x| i32::from(x))
                .to_vec();
            arg.creator_hash = instruction
                .unverifyCreatorArgs
                .creatorHash
                .map(|x| i32::from(x))
                .to_vec();
            arg.nonce = instruction.unverifyCreatorArgs.nonce;
            arg.index = instruction.unverifyCreatorArgs.index;

            let mut edition_nonce = None;
            if instruction
                .unverifyCreatorArgs
                .message
                .editionNonce
                .is_some()
            {
                edition_nonce = Some(i32::from(
                    instruction
                        .unverifyCreatorArgs
                        .message
                        .editionNonce
                        .unwrap(),
                ));
            }

            let mut collection = None;
            if instruction.unverifyCreatorArgs.message.collection.is_some() {
                collection = Some(Collection {
                    verified: instruction
                        .unverifyCreatorArgs
                        .message
                        .collection
                        .unwrap()
                        .verified,
                    key: bs58::encode(
                        instruction
                            .unverifyCreatorArgs
                            .message
                            .collection
                            .unwrap()
                            .key
                            .value,
                    )
                    .into_string(),
                });
            }

            arg.message = Some(Message {
                name: instruction.unverifyCreatorArgs.message.name,
                symbol: instruction.unverifyCreatorArgs.message.symbol,
                uri: instruction.unverifyCreatorArgs.message.uri,
                seller_fee_basis_points: i32::from(
                    instruction.unverifyCreatorArgs.message.sellerFeeBasisPoints,
                ),
                primary_sale_happened: instruction.unverifyCreatorArgs.message.primarySaleHappened,
                is_mutable: instruction.unverifyCreatorArgs.message.isMutable,
                edition_nonce: edition_nonce,
                token_standard: instruction
                    .unverifyCreatorArgs
                    .message
                    .tokenStandard
                    .unwrap()
                    .to_string(),
                collection: collection,
            });
        }
        "VerifyCollection" => {
            arg.instruction_type = String::from("VerifyCollection");
            arg.root = instruction
                .verifyCollectionArgs
                .root
                .map(|x| i32::from(x))
                .to_vec();
            arg.data_hash = instruction
                .verifyCollectionArgs
                .dataHash
                .map(|x| i32::from(x))
                .to_vec();
            arg.creator_hash = instruction
                .verifyCollectionArgs
                .creatorHash
                .map(|x| i32::from(x))
                .to_vec();
            arg.nonce = instruction.verifyCollectionArgs.nonce;
            arg.index = instruction.verifyCollectionArgs.index;

            let mut edition_nonce = None;
            if instruction
                .verifyCollectionArgs
                .message
                .editionNonce
                .is_some()
            {
                edition_nonce = Some(i32::from(
                    instruction
                        .verifyCollectionArgs
                        .message
                        .editionNonce
                        .unwrap(),
                ));
            }

            let mut collection = None;
            if instruction.verifyCollectionArgs.message.collection.is_some() {
                collection = Some(Collection {
                    verified: instruction
                        .verifyCollectionArgs
                        .message
                        .collection
                        .unwrap()
                        .verified,
                    key: bs58::encode(
                        instruction
                            .verifyCollectionArgs
                            .message
                            .collection
                            .unwrap()
                            .key
                            .value,
                    )
                    .into_string(),
                });
            }

            arg.message = Some(Message {
                name: instruction.verifyCollectionArgs.message.name,
                symbol: instruction.verifyCollectionArgs.message.symbol,
                uri: instruction.verifyCollectionArgs.message.uri,
                seller_fee_basis_points: i32::from(
                    instruction.verifyCollectionArgs.message.sellerFeeBasisPoints,
                ),
                primary_sale_happened: instruction.verifyCollectionArgs.message.primarySaleHappened,
                is_mutable: instruction.verifyCollectionArgs.message.isMutable,
                edition_nonce: edition_nonce,
                token_standard: instruction
                    .verifyCollectionArgs
                    .message
                    .tokenStandard
                    .unwrap()
                    .to_string(),
                collection: collection,
            });
        }
        "UnverifyCollection" => {
            arg.instruction_type = String::from("UnverifyCollection");
            arg.root = instruction
                .unverifyCollectionArgs
                .root
                .map(|x| i32::from(x))
                .to_vec();
            arg.data_hash = instruction
                .unverifyCollectionArgs
                .dataHash
                .map(|x| i32::from(x))
                .to_vec();
            arg.creator_hash = instruction
                .unverifyCollectionArgs
                .creatorHash
                .map(|x| i32::from(x))
                .to_vec();
            arg.nonce = instruction.unverifyCollectionArgs.nonce;
            arg.index = instruction.unverifyCollectionArgs.index;

            let mut edition_nonce = None;
            if instruction
                .unverifyCollectionArgs
                .message
                .editionNonce
                .is_some()
            {
                edition_nonce = Some(i32::from(
                    instruction
                        .unverifyCollectionArgs
                        .message
                        .editionNonce
                        .unwrap(),
                ));
            }

            let mut collection = None;
            if instruction.unverifyCollectionArgs.message.collection.is_some() {
                collection = Some(Collection {
                    verified: instruction
                        .unverifyCollectionArgs
                        .message
                        .collection
                        .unwrap()
                        .verified,
                    key: bs58::encode(
                        instruction
                            .unverifyCollectionArgs
                            .message
                            .collection
                            .unwrap()
                            .key
                            .value,
                    )
                    .into_string(),
                });
            }

            arg.message = Some(Message {
                name: instruction.unverifyCollectionArgs.message.name,
                symbol: instruction.unverifyCollectionArgs.message.symbol,
                uri: instruction.unverifyCollectionArgs.message.uri,
                seller_fee_basis_points: i32::from(
                    instruction.unverifyCollectionArgs.message.sellerFeeBasisPoints,
                ),
                primary_sale_happened: instruction.unverifyCollectionArgs.message.primarySaleHappened,
                is_mutable: instruction.unverifyCollectionArgs.message.isMutable,
                edition_nonce: edition_nonce,
                token_standard: instruction
                    .unverifyCollectionArgs
                    .message
                    .tokenStandard
                    .unwrap()
                    .to_string(),
                collection: collection,
            });
        }
        "SetAndVerifyCollection" => {
            arg.instruction_type = String::from("SetAndVerifyCollection");
            arg.root = instruction.setAndVerifyCollectionArgs.root.map(|x| i32::from(x)).to_vec();
            arg.data_hash = instruction.setAndVerifyCollectionArgs.dataHash.map(|x| i32::from(x)).to_vec();
            arg.creator_hash = instruction.setAndVerifyCollectionArgs.creatorHash.map(|x| i32::from(x)).to_vec();
            arg.nonce = instruction.setAndVerifyCollectionArgs.nonce;
            arg.index = instruction.setAndVerifyCollectionArgs.index;
            
            let mut edition_nonce = None;
            if instruction
                .setAndVerifyCollectionArgs
                .message
                .editionNonce
                .is_some()
            {
                edition_nonce = Some(i32::from(
                    instruction
                        .setAndVerifyCollectionArgs
                        .message
                        .editionNonce
                        .unwrap(),
                ));
            }

            let mut collection = None;
            if instruction.setAndVerifyCollectionArgs.message.collection.is_some() {
                collection = Some(Collection {
                    verified: instruction
                        .setAndVerifyCollectionArgs
                        .message
                        .collection
                        .unwrap()
                        .verified,
                    key: bs58::encode(
                        instruction
                            .setAndVerifyCollectionArgs
                            .message
                            .collection
                            .unwrap()
                            .key
                            .value,
                    )
                    .into_string(),
                });
            }
            
            arg.message = Some(Message {
                name: instruction.setAndVerifyCollectionArgs.message.name,
                symbol: instruction.unverifyCollectionArgs.message.symbol,
                uri: instruction.unverifyCollectionArgs.message.uri,
                seller_fee_basis_points: i32::from(
                    instruction.unverifyCollectionArgs.message.sellerFeeBasisPoints,
                ),
                primary_sale_happened: instruction.unverifyCollectionArgs.message.primarySaleHappened,
                is_mutable: instruction.unverifyCollectionArgs.message.isMutable,
                edition_nonce: edition_nonce,
                token_standard: instruction
                    .unverifyCollectionArgs
                    .message
                    .tokenStandard
                    .unwrap()
                    .to_string(),
                collection: collection,
            });

            arg.collection = get_b58_string(instruction.setAndVerifyCollectionArgs.collection.value);
        }
        "Transfer" => {
            arg.instruction_type = String::from("Transfer");
            arg.root = instruction.transferArgs.root.map(|x| i32::from(x)).to_vec();
            arg.data_hash = instruction.transferArgs.dataHash.map(|x| i32::from(x)).to_vec();
            arg.creator_hash = instruction.transferArgs.creatorHash.map(|x| i32::from(x)).to_vec();
            arg.nonce = instruction.transferArgs.nonce;
            arg.index = instruction.transferArgs.index;
        }
        "Delegate" => {
            arg.instruction_type = String::from("Delegate");
            arg.root = instruction.delegateArgs.root.map(|x| i32::from(x)).to_vec();
            arg.data_hash = instruction.delegateArgs.dataHash.map(|x| i32::from(x)).to_vec();
            arg.creator_hash = instruction.delegateArgs.creatorHash.map(|x| i32::from(x)).to_vec();
            arg.nonce = instruction.delegateArgs.nonce;
            arg.index = instruction.delegateArgs.index;
        }
        "Burn" => {
            arg.instruction_type = String::from("Burn");
            arg.root = instruction.burnArgs.root.map(|x| i32::from(x)).to_vec();
            arg.data_hash = instruction.burnArgs.dataHash.map(|x| i32::from(x)).to_vec();
            arg.creator_hash = instruction.burnArgs.creatorHash.map(|x| i32::from(x)).to_vec();
            arg.nonce = instruction.burnArgs.nonce;
            arg.index = instruction.burnArgs.index;
        }
        "Redeem" => {
            arg.instruction_type = String::from("Redeem");
            arg.root = instruction.burnArgs.root.map(|x| i32::from(x)).to_vec();
            arg.data_hash = instruction.burnArgs.dataHash.map(|x| i32::from(x)).to_vec();
            arg.creator_hash = instruction.burnArgs.creatorHash.map(|x| i32::from(x)).to_vec();
            arg.nonce = instruction.burnArgs.nonce;
            arg.index = instruction.burnArgs.index;
        }
        "CancelRedeem" => {
            arg.instruction_type = String::from("CancelRedeem");
            arg.root = instruction.cancelRedeemArgs.root.map(|x| i32::from(x)).to_vec();
        }
        "DecompressV1" => {
            arg.instruction_type = String::from("DecompressV1");

            let mut edition_nonce = None;
            if instruction.decompressV1Args.metadata.editionNonce.is_some() {
                edition_nonce = Some(i32::from(instruction.decompressV1Args.metadata.editionNonce.unwrap()));
            }

            let mut collection = None;
            if instruction.decompressV1Args.metadata.collection.is_some() {
                collection = Some(Collection {
                    verified: instruction.decompressV1Args.metadata.collection.unwrap().verified,
                    key: get_b58_string(instruction.decompressV1Args.metadata.collection.unwrap().key.value)
                });
            }

            let mut creators: Vec<Creator> = vec![];
            for creator in instruction.decompressV1Args.metadata.creators.iter() {
                creators.push(Creator {
                    address: get_b58_string(creator.address.value),
                    verified: creator.verified,
                    share: i32::from(creator.share),
                });
            }

            arg.metadata = Some(Metadata {
                name: instruction.decompressV1Args.metadata.name,
                symbol: instruction.decompressV1Args.metadata.symbol,
                uri: instruction.decompressV1Args.metadata.uri,
                seller_fee_basis_points: i32::from(instruction.decompressV1Args.metadata.sellerFeeBasisPoints),
                primary_sale_happened: instruction.decompressV1Args.metadata.primarySaleHappened,
                is_mutable: instruction.decompressV1Args.metadata.isMutable,
                edition_nonce: edition_nonce,
                token_standard: instruction.decompressV1Args.metadata.tokenStandard.unwrap().to_string(),
                collection: collection,
                uses: Some(Uses {
                    use_method: instruction.decompressV1Args.metadata.uses.unwrap().useMethod.to_string(),
                    remaining: instruction.decompressV1Args.metadata.uses.unwrap().remaining,
                    total: instruction.decompressV1Args.metadata.uses.unwrap().total,
                }),
                token_program_version: instruction.decompressV1Args.metadata.tokenProgramVersion.to_string(),
                creators: creators,
            });
        }
        "Compress" => {
            arg.instruction_type = String::from("Compress");
        }
        _ => {
            arg.instruction_type = String::from("Unknown Instruction");
        }
    }

    return arg;
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
