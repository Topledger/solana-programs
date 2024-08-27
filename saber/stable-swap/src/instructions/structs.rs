use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbDepositLayout, PbFeesLayout, PbInitializeLayout, PbSwapLayout, PbWithdrawLayout,
    PbWithdrawOneLayout,
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
pub struct FeesLayout {
    pub admin_trade_fee_numerator: u64,
    pub admin_trade_fee_denominator: u64,
    pub admin_withdraw_fee_numerator: u64,
    pub admin_withdraw_fee_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub withdraw_fee_numerator: u64,
    pub withdraw_fee_denominator: u64,
}

impl FeesLayout {
    pub fn to_proto_struct(&self) -> PbFeesLayout {
        PbFeesLayout {
            admin_trade_fee_numerator: self.admin_trade_fee_numerator,
            admin_trade_fee_denominator: self.admin_trade_fee_denominator,
            admin_withdraw_fee_numerator: self.admin_withdraw_fee_numerator,
            admin_withdraw_fee_denominator: self.admin_withdraw_fee_denominator,
            trade_fee_numerator: self.trade_fee_numerator,
            trade_fee_denominator: self.trade_fee_denominator,
            withdraw_fee_numerator: self.withdraw_fee_numerator,
            withdraw_fee_denominator: self.withdraw_fee_denominator,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct InitializeLayout {
    pub nonce: u8,
    pub amp_factor: u64,
    pub fees: FeesLayout,
}

impl InitializeLayout {
    pub fn to_proto_struct(&self) -> PbInitializeLayout {
        PbInitializeLayout {
            nonce: self.nonce as u32,
            amp_factor: self.amp_factor,
            fees: self.fees.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SwapLayout {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

impl SwapLayout {
    pub fn to_proto_struct(&self) -> PbSwapLayout {
        PbSwapLayout {
            amount_in: self.amount_in,
            minimum_amount_out: self.minimum_amount_out,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct DepositLayout {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub min_mint_amount: u64,
}

impl DepositLayout {
    pub fn to_proto_struct(&self) -> PbDepositLayout {
        PbDepositLayout {
            token_a_amount: self.token_a_amount,
            token_b_amount: self.token_b_amount,
            min_mint_amount: self.min_mint_amount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct WithdrawLayout {
    pub pool_token_amount: u64,
    pub minimum_token_a_amount: u64,
    pub minimum_token_b_amount: u64,
}

impl WithdrawLayout {
    pub fn to_proto_struct(&self) -> PbWithdrawLayout {
        PbWithdrawLayout {
            pool_token_amount: self.pool_token_amount,
            minimum_token_a_amount: self.minimum_token_a_amount,
            minimum_token_b_amount: self.minimum_token_b_amount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct WithdrawOneLayout {
    pub pool_token_amount: u64,
    pub minimum_token_amount: u64,
}

impl WithdrawOneLayout {
    pub fn to_proto_struct(&self) -> PbWithdrawOneLayout {
        PbWithdrawOneLayout {
            pool_token_amount: self.pool_token_amount,
            minimum_token_amount: self.minimum_token_amount,
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
