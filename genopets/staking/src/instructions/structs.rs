use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbChangePoolWeightLayout, PbChangeTotalGeneAllocatedLayout, PbClaimRewardsLayout,
    PbCreateGlobalStateLayout, PbCreateStakingPoolLayout, PbReLockDepositLayout, PbStakeLayout,
    PbWithdrawLayout,
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
pub struct ChangeTotalGeneAllocatedLayout {
    pub newTotalGeneAllocated: u64,
}

impl ChangeTotalGeneAllocatedLayout {
    pub fn to_proto_struct(&self) -> PbChangeTotalGeneAllocatedLayout {
        PbChangeTotalGeneAllocatedLayout {
            new_total_gene_allocated: self.newTotalGeneAllocated,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ChangePoolWeightLayout {
    pub newPoolWeight: u32,
}

impl ChangePoolWeightLayout {
    pub fn to_proto_struct(&self) -> PbChangePoolWeightLayout {
        PbChangePoolWeightLayout {
            new_pool_weight: self.newPoolWeight,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreateGlobalStateLayout {
    pub startTime: u64,
    pub epochTime: u64,
    pub endTime: u64,
    pub decayFactorPerEpoch: f64,
    pub totalGeneAllocated: u64,
    pub timeFactor: Option<u64>,
}

impl CreateGlobalStateLayout {
    pub fn to_proto_struct(&self) -> PbCreateGlobalStateLayout {
        PbCreateGlobalStateLayout {
            start_time: self.startTime,
            epoch_time: self.epochTime,
            end_time: self.endTime,
            decay_factor_per_epoch: self.decayFactorPerEpoch,
            total_gene_allocated: self.totalGeneAllocated,
            time_factor: self.timeFactor,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreateStakingPoolLayout {
    pub earliestUnlockDate: u64,
    pub poolWeight: u32,
    pub weightPerToken: u32,
    pub governanceEligible: bool,
}

impl CreateStakingPoolLayout {
    pub fn to_proto_struct(&self) -> PbCreateStakingPoolLayout {
        PbCreateStakingPoolLayout {
            earliest_unlock_date: self.earliestUnlockDate,
            pool_weight: self.poolWeight,
            weight_per_token: self.weightPerToken,
            governance_eligible: self.governanceEligible,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct StakeLayout {
    pub amount: u64,
    pub lockForMonths: u8,
    pub asSgene: bool,
}

impl StakeLayout {
    pub fn to_proto_struct(&self) -> PbStakeLayout {
        PbStakeLayout {
            amount: self.amount,
            lock_for_months: self.lockForMonths as u32,
            as_sgene: self.asSgene,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ReLockDepositLayout {
    pub newLockForMonths: u8,
    pub asSgene: bool,
}

impl ReLockDepositLayout {
    pub fn to_proto_struct(&self) -> PbReLockDepositLayout {
        PbReLockDepositLayout {
            new_lock_for_months: self.newLockForMonths as u32,
            as_sgene: self.asSgene,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct WithdrawLayout {
    pub asSgene: bool,
}

impl WithdrawLayout {
    pub fn to_proto_struct(&self) -> PbWithdrawLayout {
        PbWithdrawLayout {
            as_sgene: self.asSgene,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ClaimRewardsLayout {
    pub asSgene: bool,
}

impl ClaimRewardsLayout {
    pub fn to_proto_struct(&self) -> PbClaimRewardsLayout {
        PbClaimRewardsLayout {
            as_sgene: self.asSgene,
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
