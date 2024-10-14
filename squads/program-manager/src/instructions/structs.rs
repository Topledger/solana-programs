use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbCreateManagedProgramLayout, PbCreateProgramUpgradeLayout,
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
pub struct CreateManagedProgramLayout {
    pub programAddress: PubKeyLayout,
    pub name: String,
}

impl CreateManagedProgramLayout {
    pub fn to_proto_struct(&self) -> PbCreateManagedProgramLayout {
        PbCreateManagedProgramLayout {
            program_address: self.programAddress.to_proto_struct(),
            name: self.name.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreateProgramUpgradeLayout {
    pub buffer: PubKeyLayout,
    pub spill: PubKeyLayout,
    pub authority: PubKeyLayout,
    pub name: String,
}

impl CreateProgramUpgradeLayout {
    pub fn to_proto_struct(&self) -> PbCreateProgramUpgradeLayout {
        PbCreateProgramUpgradeLayout {
            buffer: self.buffer.to_proto_struct(),
            spill: self.spill.to_proto_struct(),
            authority: self.authority.to_proto_struct(),
            name: self.name.to_string(),
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
