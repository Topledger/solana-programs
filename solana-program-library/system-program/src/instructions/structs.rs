use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{PbCreateAccount, PbCreateAccountWithSeed};

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
pub struct CreateAccountLayout {
    pub lamports: u64,
    pub space: u64,
    pub owner: PubKeyLayout,
}

impl CreateAccountLayout {
    pub fn to_proto_struct(&self) -> PbCreateAccount {
        PbCreateAccount {
            lamports: self.lamports,
            space: self.space,
            owner: self.owner.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreateAccountWithSeedLayout {
    pub base: PubKeyLayout,
    pub seed: String,
    pub lamports: u64,
    pub space: u64,
    pub owner: PubKeyLayout,
}

impl CreateAccountWithSeedLayout {
    pub fn to_proto_struct(&self) -> PbCreateAccountWithSeed {
        PbCreateAccountWithSeed {
            base: self.base.to_proto_struct(),
            seed: self.seed.clone(),
            lamports: self.lamports,
            space: self.space,
            owner: self.owner.to_proto_struct(),
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
