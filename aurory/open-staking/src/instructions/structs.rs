use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbInitializeLayout, PbReclaimMintAuthorityLayout, PbStakeLayout, PbUnstakeLayout,
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
pub struct InitializeLayout {
    pub nonce: u8,
}

impl InitializeLayout {
    pub fn to_proto_struct(&self) -> PbInitializeLayout {
        PbInitializeLayout {
            nonce: self.nonce as u32,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ReclaimMintAuthorityLayout {
    pub nonce: u8,
}

impl ReclaimMintAuthorityLayout {
    pub fn to_proto_struct(&self) -> PbReclaimMintAuthorityLayout {
        PbReclaimMintAuthorityLayout {
            nonce: self.nonce as u32,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct StakeLayout {
    pub nonce: u8,
    pub amount: u64,
}

impl StakeLayout {
    pub fn to_proto_struct(&self) -> PbStakeLayout {
        PbStakeLayout {
            nonce: self.nonce as u32,
            amount: self.amount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UnstakeLayout {
    pub nonce: u8,
    pub amount: u64,
}

impl UnstakeLayout {
    pub fn to_proto_struct(&self) -> PbUnstakeLayout {
        PbUnstakeLayout {
            nonce: self.nonce as u32,
            amount: self.amount,
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
