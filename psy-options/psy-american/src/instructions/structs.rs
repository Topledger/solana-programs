use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbBurnWriterForQuoteLayout, PbCloseOptionPositionLayout, PbClosePostExpirationLayout,
    PbExerciseOptionLayout, PbExerciseOptionV2Layout, PbInitSerumMarketLayout,
    PbInitializeMarketLayout, PbMintOptionLayout, PbMintOptionV2Layout,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default, Copy)]
pub struct PubKeyLayout {
    pub value: [u8; 32],
}

impl PubKeyLayout {
    pub fn to_proto_struct(&self) -> String {
        let result = get_b58_string(self.value);
        result
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct InitializeMarketLayout {
    pub underlyingAmountPerContract: u64,
    pub quoteAmountPerContract: u64,
    pub expirationUnixTimestamp: f64,
    pub bumpSeed: u8,
}

impl InitializeMarketLayout {
    pub fn to_proto_struct(&self) -> PbInitializeMarketLayout {
        PbInitializeMarketLayout {
            underlying_amount_per_contract: self.underlyingAmountPerContract,
            quote_amount_per_contract: self.quoteAmountPerContract,
            expiration_unix_timestamp: self.expirationUnixTimestamp,
            bump_seed: self.bumpSeed as u32,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct MintOptionLayout {
    pub size: u64,
}

impl MintOptionLayout {
    pub fn to_proto_struct(&self) -> PbMintOptionLayout {
        PbMintOptionLayout { size: self.size }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct MintOptionV2Layout {
    pub size: u64,
}

impl MintOptionV2Layout {
    pub fn to_proto_struct(&self) -> PbMintOptionV2Layout {
        PbMintOptionV2Layout { size: self.size }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ExerciseOptionLayout {
    pub size: u64,
}

impl ExerciseOptionLayout {
    pub fn to_proto_struct(&self) -> PbExerciseOptionLayout {
        PbExerciseOptionLayout { size: self.size }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ExerciseOptionV2Layout {
    pub size: u64,
}

impl ExerciseOptionV2Layout {
    pub fn to_proto_struct(&self) -> PbExerciseOptionV2Layout {
        PbExerciseOptionV2Layout { size: self.size }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ClosePostExpirationLayout {
    pub size: u64,
}

impl ClosePostExpirationLayout {
    pub fn to_proto_struct(&self) -> PbClosePostExpirationLayout {
        PbClosePostExpirationLayout { size: self.size }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CloseOptionPositionLayout {
    pub size: u64,
}

impl CloseOptionPositionLayout {
    pub fn to_proto_struct(&self) -> PbCloseOptionPositionLayout {
        PbCloseOptionPositionLayout { size: self.size }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct BurnWriterForQuoteLayout {
    pub size: u64,
}

impl BurnWriterForQuoteLayout {
    pub fn to_proto_struct(&self) -> PbBurnWriterForQuoteLayout {
        PbBurnWriterForQuoteLayout { size: self.size }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct InitSerumMarketLayout {
    pub marketSpace: u64,
    pub vaultSignerNonce: u64,
    pub coinLotSize: u64,
    pub pcLotSize: u64,
    pub pcDustThreshold: u64,
}

impl InitSerumMarketLayout {
    pub fn to_proto_struct(&self) -> PbInitSerumMarketLayout {
        PbInitSerumMarketLayout {
            market_space: self.marketSpace,
            vault_signer_nonce: self.vaultSignerNonce,
            coin_lot_size: self.coinLotSize,
            pc_lot_size: self.pcLotSize,
            pc_dust_threshold: self.pcDustThreshold,
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
