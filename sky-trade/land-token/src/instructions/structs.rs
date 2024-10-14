use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::PbMintTokenLayout;

#[derive(BorshDeserialize, Debug, Default)]
pub struct MintTokenLayout {
    pub metadata_args: Vec<u8>,
}

impl MintTokenLayout {
    pub fn to_proto_struct(&self) -> PbMintTokenLayout {
        let mut metadata_args: Vec<u32> = vec![];
        for x in self.metadata_args.iter() {
            metadata_args.push(*x as u32);
        }

        PbMintTokenLayout {
            metadata_args: metadata_args,
        }
    }
}
