use borsh::BorshDeserialize;

use crate::pb::sf::solana::block_meta::v1::{
    PbBurnPtLayout, PbDepositLayout, PbMintPtLayout, PbSwapLayout, PbWithdrawLayout,
};

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
pub struct DepositLayout {
    pub amount: u64,
}

impl DepositLayout {
    pub fn to_proto_struct(&self) -> PbDepositLayout {
        PbDepositLayout {
            amount: self.amount,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct WithdrawLayout {
    pub withdrawPercent: u64,
}

impl WithdrawLayout {
    pub fn to_proto_struct(&self) -> PbWithdrawLayout {
        PbWithdrawLayout {
            withdraw_percent: self.withdrawPercent,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct MintPtLayout {
    pub amountToMint: u64,
}

impl MintPtLayout {
    pub fn to_proto_struct(&self) -> PbMintPtLayout {
        PbMintPtLayout {
            amount_to_mint: self.amountToMint,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct BurnPtLayout {
    pub amountToBurn: u64,
}

impl BurnPtLayout {
    pub fn to_proto_struct(&self) -> PbBurnPtLayout {
        PbBurnPtLayout {
            amount_to_burn: self.amountToBurn,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct SwapLayout {
    pub amountIn: u64,
    pub minOut: u64,
}

impl SwapLayout {
    pub fn to_proto_struct(&self) -> PbSwapLayout {
        PbSwapLayout {
            amount_in: self.amountIn,
            min_out: self.minOut,
        }
    }
}
