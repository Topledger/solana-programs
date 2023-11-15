use borsh::{BorshDeserialize, BorshSerialize};
use substreams_solana::pb::sf::solana::r#type::v1::InnerInstructions;

use crate::{pb::sf::solana::cnft::trades::v1::TradeData, utils::prepare_input_accounts};

const BUY_DISCRIMINATOR: u64 = 16927863322537952870;
const TAKE_BID_FULL_META_DISCRIMINATOR: u64 = 6920403060335035122;
const TAKE_BID_META_HASH_DISCRIMINATOR: u64 = 13910166988548399957;

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct BuyLayout {
    nonce: u64,
    index: u32,
    root: [u8; 32],
    metaHash: [u8; 32],
    creatorShares: Vec<u8>,
    creatorVerified: Vec<bool>,
    sellerFeeBasisPoints: u16,
    maxAmount: u64,
    currency: Option<[u8; 32]>,
    makerBroker: Option<[u8; 32]>,
    optionalRoyaltyPct: Option<u16>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub enum TTokenStandardLayout {
    #[default]
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TCollectionLayout {
    verified: bool,
    key: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub enum TUseMethodLayout {
    #[default]
    Burn,
    Multiple,
    Single,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TUsesLayout {
    useMethod: TUseMethodLayout,
    remaining: u64,
    total: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub enum TTokenProgramVersionLayout {
    #[default]
    Original,
    Token2022,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TMetadataArgsLayout {
    name: String,
    symbol: String,
    uri: String,
    sellerFeeBasisPoints: u16,
    primarySaleHappened: bool,
    isMutable: bool,
    editionNonce: Option<u8>,
    tokenStandard: Option<TTokenStandardLayout>,
    collection: Option<TCollectionLayout>,
    uses: Option<TUsesLayout>,
    tokenProgramVersion: TTokenProgramVersionLayout,
    creatorShares: Vec<u8>,
    creatorVerified: Vec<bool>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TakeBidFullMetaLayout {
    nonce: u64,
    index: u32,
    root: [u8; 32],
    metaArgs: TMetadataArgsLayout,
    minAmount: u64,
    currency: Option<[u8; 32]>,
    makerBroker: Option<[u8; 32]>,
    optionalRoyaltyPct: Option<u16>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TakeBidMetaHashLayout {
    nonce: u64,
    index: u32,
    root: [u8; 32],
    metaHash: [u8; 32],
    creatorShares: Vec<u8>,
    creatorVerified: Vec<bool>,
    sellerFeeBasisPoints: u16,
    minAmount: u64,
    currency: Option<[u8; 32]>,
    makerBroker: Option<[u8; 32]>,
    optionalRoyaltyPct: Option<u16>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct BubblegumTransferLayout {
    root: [u8; 32],
    dataHash: [u8; 32],
    creatorHash: [u8; 32],
    nonce: u64,
    index: u32,
}

pub fn enrich_with_inner_instructions_data(
    trade_data: &mut TradeData,
    accounts: &Vec<String>,
    inner_instructions: &Vec<InnerInstructions>,
) -> () {
    inner_instructions.iter().for_each(|inner_instruction| {
        inner_instruction
            .instructions
            .iter()
            .enumerate()
            .for_each(|(inner_idx, inner_inst)| {
                let inner_program = &accounts[inner_inst.program_id_index as usize];
                let (discriminator, rest) = inner_inst.data.split_at(8);

                if inner_program
                    .as_str()
                    .eq("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY")
                    & inner_inst.data.clone().starts_with(discriminator)
                {
                    let transfer_data = BubblegumTransferLayout::try_from_slice(rest).unwrap();
                    trade_data.leaf_id = transfer_data.index;

                    let input_accounts = prepare_input_accounts(&inner_inst.accounts, accounts);
                    trade_data.merkle_tree = input_accounts.get(4).unwrap().to_string();
                }
            })
    });
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    accounts: &Vec<String>,
    log_messages: &Vec<String>,
    inner_instructions: &Vec<InnerInstructions>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;
    let mut trade_data: TradeData;

    match discriminator {
        BUY_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "Buy".to_string();
            trade_data.platform = "tensorswap".to_string();
            trade_data.currency = "SOL".to_string();

            trade_data.signer = accounts.get(0).unwrap().to_string();
            trade_data.buyer = input_accounts.get(9).unwrap().to_string();
            trade_data.seller = input_accounts.get(8).unwrap().to_string();

            if trade_data.signer == trade_data.buyer {
                trade_data.category = "buy".to_string();
            } else {
                trade_data.category = "sell".to_string();
            }

            let mut mut_rest = rest.clone();
            let instruction_data = BuyLayout::deserialize(&mut mut_rest).unwrap();

            trade_data.amount = instruction_data.maxAmount as f64;
            trade_data.taker_fee = 0.014 * trade_data.amount;
            trade_data.maker_fee = 0.0;
            trade_data.amm_fee = 0.0;
            trade_data.royalty =
                (instruction_data.sellerFeeBasisPoints as f64 * trade_data.amount / 10000.0) as f64;

            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);

            result = Some(trade_data);
        }
        TAKE_BID_FULL_META_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "TakeBidFullMeta".to_string();
            trade_data.platform = "tensorswap".to_string();
            trade_data.currency = "SOL".to_string();

            trade_data.signer = accounts.get(0).unwrap().to_string();
            trade_data.buyer = input_accounts.get(12).unwrap().to_string();
            trade_data.seller = input_accounts.get(2).unwrap().to_string();

            trade_data.category = "sell".to_string();

            let instruction_data = TakeBidFullMetaLayout::try_from_slice(rest).unwrap();

            trade_data.amount = instruction_data.minAmount as f64;
            trade_data.taker_fee = 0.014 * trade_data.amount;
            trade_data.maker_fee = 0.0;
            trade_data.amm_fee = 0.0;
            trade_data.royalty = (instruction_data.metaArgs.sellerFeeBasisPoints as f64
                * trade_data.amount
                / 10000.0) as f64;

            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);

            result = Some(trade_data);
        }
        TAKE_BID_META_HASH_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "TakeBidMetaHash".to_string();
            trade_data.platform = "tensorswap".to_string();
            trade_data.currency = "SOL".to_string();

            trade_data.signer = accounts.get(0).unwrap().to_string();
            trade_data.buyer = input_accounts.get(12).unwrap().to_string();
            trade_data.seller = input_accounts.get(2).unwrap().to_string();

            trade_data.category = "sell".to_string();

            let instruction_data = TakeBidMetaHashLayout::try_from_slice(rest).unwrap();

            trade_data.amount = instruction_data.minAmount as f64;
            trade_data.taker_fee = 0.014 * trade_data.amount;
            trade_data.maker_fee = 0.0;
            trade_data.amm_fee = 0.0;
            trade_data.royalty =
                (instruction_data.sellerFeeBasisPoints as f64 * trade_data.amount / 10000.0) as f64;

            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);

            result = Some(trade_data);
        }
        _ => {}
    }

    return result;
}
