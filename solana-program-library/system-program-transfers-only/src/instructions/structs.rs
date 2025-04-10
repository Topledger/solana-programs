use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{PbTransfer, PbTransferWithSeed};

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
pub struct TransferLayout {
    pub lamports: u64,
}

impl TransferLayout {
    pub fn to_proto_struct(&self) -> PbTransfer {
        PbTransfer {
            lamports: self.lamports,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct TransferWithSeedLayout {
    pub lamports: u64,
    pub from_seed: String,
    pub from_owner: PubKeyLayout,
}

impl TransferWithSeedLayout {
    pub fn to_proto_struct(&self) -> PbTransferWithSeed {
        PbTransferWithSeed {
            lamports: self.lamports,
            from_seed: self.from_seed.clone(),
            from_owner: self.from_owner.to_proto_struct(),
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
