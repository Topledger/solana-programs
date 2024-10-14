use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbPrefundSwapViaStakeLayout, PbPrefundWithdrawStakeLayout, PbStakeWrappedSolLayout,
    PbSwapViaStakeArgsLayout, PbSwapViaStakeLayout,
};

#[derive(BorshDeserialize, Debug, Default)]
pub struct SwapViaStakeArgsLayout {
    pub amount: u64,
    pub bridgeStakeSeed: u32,
}

impl SwapViaStakeArgsLayout {
    pub fn to_proto_struct(&self) -> PbSwapViaStakeArgsLayout {
        PbSwapViaStakeArgsLayout {
            amount: self.amount,
            bridge_stake_seed: self.bridgeStakeSeed,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct StakeWrappedSolLayout {
    pub amount: u64,
}

impl StakeWrappedSolLayout {
    pub fn to_proto_struct(&self) -> PbStakeWrappedSolLayout {
        PbStakeWrappedSolLayout {
            amount: self.amount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SwapViaStakeLayout {
    pub args: SwapViaStakeArgsLayout,
}

impl SwapViaStakeLayout {
    pub fn to_proto_struct(&self) -> PbSwapViaStakeLayout {
        PbSwapViaStakeLayout {
            args: self.args.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct PrefundWithdrawStakeLayout {
    pub args: SwapViaStakeArgsLayout,
}

impl PrefundWithdrawStakeLayout {
    pub fn to_proto_struct(&self) -> PbPrefundWithdrawStakeLayout {
        PbPrefundWithdrawStakeLayout {
            args: self.args.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct PrefundSwapViaStakeLayout {
    pub args: SwapViaStakeArgsLayout,
}

impl PrefundSwapViaStakeLayout {
    pub fn to_proto_struct(&self) -> PbPrefundSwapViaStakeLayout {
        PbPrefundSwapViaStakeLayout {
            args: self.args.to_proto_struct(),
        }
    }
}
