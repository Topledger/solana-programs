use std::default;

use borsh::BorshDeserialize;

use crate::pb::sf::solana::block_meta::v1::{
    PbDepositLayout, PbFulfillDlmmFillLayout, PbFulfillFlashFillLayout, PbOpenDcaLayout,
    PbOpenDcaV2Layout, PbWithdrawFeesLayout, PbWithdrawLayout, PbWithdrawParamsLayout,
};

#[derive(BorshDeserialize, Debug, Default)]
pub enum WithdrawalLayout {
    #[default]
    In,
    Out,
}

impl WithdrawalLayout {
    pub fn to_proto_struct(&self) -> String {
        let mut result: String = "".to_string();
        match self {
            WithdrawalLayout::In => {
                result = "In".to_string();
            }
            WithdrawalLayout::Out => {
                result = "Out".to_string();
            }
        }
        result
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct WithdrawParamsLayout {
    withdrawAmount: u64,
    withdrawal: WithdrawalLayout,
}

impl WithdrawParamsLayout {
    pub fn to_proto_struct(&self) -> PbWithdrawParamsLayout {
        PbWithdrawParamsLayout {
            withdraw_amount: self.withdrawAmount,
            withdrawal: self.withdrawal.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct OpenDcaLayout {
    applicationIdx: u64,
    inAmount: u64,
    inAmountPerCycle: u64,
    cycleFrequency: f64,
    minPrice: Option<u64>,
    maxPrice: Option<u64>,
    startAt: Option<f64>,
    closeWsolInAta: Option<bool>,
}

impl OpenDcaLayout {
    pub fn to_proto_struct(&self) -> PbOpenDcaLayout {
        let mut min_price: Option<u64> = None;
        if self.minPrice.is_some() {
            min_price = Some(self.minPrice.unwrap());
        }

        let mut max_price: Option<u64> = None;
        if self.maxPrice.is_some() {
            max_price = Some(self.maxPrice.unwrap());
        }

        let mut start_at: Option<f64> = None;
        if self.startAt.is_some() {
            start_at = Some(self.startAt.unwrap());
        }

        let mut close_wsol_in_ata: Option<bool> = None;
        if self.closeWsolInAta.is_some() {
            close_wsol_in_ata = Some(self.closeWsolInAta.unwrap());
        }

        PbOpenDcaLayout {
            application_idx: self.applicationIdx,
            in_amount: self.inAmount,
            in_amount_per_cycle: self.inAmountPerCycle,
            cycle_frequency: self.cycleFrequency,
            min_price: min_price,
            max_price: max_price,
            start_at: start_at,
            close_wsol_in_ata: close_wsol_in_ata,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct WithdrawLayout {
    pub withdrawParams: WithdrawParamsLayout,
}

impl WithdrawLayout {
    pub fn to_proto_struct(&self) -> PbWithdrawLayout {
        PbWithdrawLayout {
            withdraw_params: self.withdrawParams.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct DepositLayout {
    depositIn: u64,
}

impl DepositLayout {
    pub fn to_proto_struct(&self) -> PbDepositLayout {
        PbDepositLayout {
            deposit_in: self.depositIn,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct WithdrawFeesLayout {
    amount: u64,
}

impl WithdrawFeesLayout {
    pub fn to_proto_struct(&self) -> PbWithdrawFeesLayout {
        PbWithdrawFeesLayout {
            amount: self.amount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct FulfillFlashFillLayout {
    repayAmount: u64,
}

impl FulfillFlashFillLayout {
    pub fn to_proto_struct(&self) -> PbFulfillFlashFillLayout {
        PbFulfillFlashFillLayout {
            repay_amount: self.repayAmount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct OpenDcaV2Layout {
    applicationIdx: u64,
    inAmount: u64,
    inAmountPerCycle: u64,
    cycleFrequency: f64,
    minOutAmount: Option<u64>,
    maxOutAmount: Option<u64>,
    startAt: Option<f64>,
}

impl OpenDcaV2Layout {
    pub fn to_proto_struct(&self) -> PbOpenDcaV2Layout {
        let mut min_out_amount: Option<u64> = None;
        if self.minOutAmount.is_some() {
            min_out_amount = Some(self.minOutAmount.unwrap());
        }

        let mut max_out_amount: Option<u64> = None;
        if self.maxOutAmount.is_some() {
            max_out_amount = Some(self.maxOutAmount.unwrap());
        }

        let mut start_at: Option<f64> = None;
        if self.startAt.is_some() {
            start_at = Some(self.startAt.unwrap());
        }

        PbOpenDcaV2Layout {
            application_idx: self.applicationIdx,
            in_amount: self.inAmount,
            in_amount_per_cycle: self.inAmountPerCycle,
            cycle_frequency: self.cycleFrequency,
            min_out_amount: min_out_amount,
            max_out_amount: max_out_amount,
            start_at: start_at,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct FulfillDlmmFillLayout {
    repayAmount: u64,
}

impl FulfillDlmmFillLayout {
    pub fn to_proto_struct(&self) -> PbFulfillDlmmFillLayout {
        PbFulfillDlmmFillLayout {
            repay_amount: self.repayAmount,
        }
    }
}
