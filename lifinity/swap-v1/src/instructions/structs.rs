use borsh::BorshDeserialize;

use crate::pb::sf::solana::block_meta::v1::PbSwapLayout;

#[derive(BorshDeserialize, Debug, Default)]
pub struct SwapLayout {
    pub amountIn: u64,
    pub minimumAmountOut: u64,
}

impl SwapLayout {
    pub fn to_proto_struct(&self) -> PbSwapLayout {
        PbSwapLayout {
            amount_in: self.amountIn,
            minimum_amount_out: self.minimumAmountOut,
        }
    }
}
