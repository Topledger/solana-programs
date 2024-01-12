use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbSetMinPublishersArgsLayout, PbUpdatePriceNoFailOnErrorArgsLayout,
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

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct TradingStatus {
    pub val: u32,
}

impl TradingStatus {
    pub fn to_proto_struct(&self) -> String {
        let mut result = "".to_string();

        match self.val {
            0 => {
                result = "Unknown".to_string();
            }
            1 => {
                result = "Trading".to_string();
            }
            2 => {
                result = "Halted".to_string();
            }
            3 => {
                result = "Auction".to_string();
            }
            _ => {}
        }

        return result;
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UpdatePriceNoFailOnErrorArgsLayout {
    pub status: TradingStatus,
    pub unused1: [u8; 4],
    pub price: i64,
    pub conf: u64,
    pub publishSlot: u64,
}

impl UpdatePriceNoFailOnErrorArgsLayout {
    pub fn to_proto_struct(&self) -> PbUpdatePriceNoFailOnErrorArgsLayout {
        PbUpdatePriceNoFailOnErrorArgsLayout {
            status: self.status.to_proto_struct(),
            unused1: 0,
            price: self.price,
            conf: self.conf,
            publish_slot: self.publishSlot,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SetMinPublishersArgsLayout {
    pub minPublishers: u8,
    pub unused1: [u8; 3],
}

impl SetMinPublishersArgsLayout {
    pub fn to_proto_struct(&self) -> PbSetMinPublishersArgsLayout {
        let mut unused1: [u32; 3] = [0, 0, 0];
        for (elem, index) in self.unused1.into_iter().enumerate() {
            unused1[index as usize] = elem as u32;
        }
        PbSetMinPublishersArgsLayout {
            min_publishers: self.minPublishers as u32,
            unused1: unused1.to_vec(),
        }
    }
}
