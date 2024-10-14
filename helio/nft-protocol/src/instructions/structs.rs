use borsh::BorshDeserialize;

use crate::pb::sf::solana::block_meta::v1::PbEscrowNftLayout;

#[derive(BorshDeserialize, Debug, Clone, Default, Copy)]
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

#[derive(BorshDeserialize, Default, Debug)]
pub struct EscrowNftLayout {
    pub unitPrice: u64,
    pub fee: u64,
    pub bump: u8,
}

impl EscrowNftLayout {
    pub fn to_proto_struct(&self) -> PbEscrowNftLayout {
        PbEscrowNftLayout {
            unit_price: self.unitPrice,
            fee: self.fee,
            bump: self.bump as u32,
        }
    }
}
