use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::PbProcessEnlistPlayerLayout;

#[derive(BorshDeserialize, Debug, Default)]
pub struct ProcessEnlistPlayerLayout {
    pub bump: u8,
    pub factionId: u8,
}

impl ProcessEnlistPlayerLayout {
    pub fn to_proto_struct(&self) -> PbProcessEnlistPlayerLayout {
        PbProcessEnlistPlayerLayout {
            bump: self.bump as u32,
            faction_id: self.factionId as u32,
        }
    }
}
