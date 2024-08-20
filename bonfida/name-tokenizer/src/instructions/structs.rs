use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{PbCreateNftLayout, PbEditDataLayout};

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
pub struct CreateNftLayout {
    pub name: String,
    pub uri: String,
}

impl CreateNftLayout {
    pub fn to_proto_struct(&self) -> PbCreateNftLayout {
        PbCreateNftLayout {
            name: self.name.to_string(),
            uri: self.uri.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Clone, Default)]
pub struct EditDataLayout {
    pub offset: u32,
    pub data: Vec<u8>,
}

impl EditDataLayout {
    pub fn to_proto_struct(&self) -> PbEditDataLayout {
        let mut data: Vec<u32> = vec![];
        for x in self.data.iter() {
            data.push(*x as u32);
        }

        PbEditDataLayout {
            offset: self.offset,
            data: data,
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
